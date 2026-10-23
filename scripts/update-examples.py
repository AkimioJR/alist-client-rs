#!/usr/bin/env python3
# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""
AList 与 OpenList 源码版本管理与更新工具。

用于管理与更新 `examples/` 目录下的 AList 与 OpenList 源码副本，
支持列出可用版本（Git Tags）、查看当前检出状态、切换指定版本或一键更新至最新稳定版。
"""

import argparse
import logging
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

# 终端彩色输出辅助（仅当 stdout 为终端且未设置 NO_COLOR 时启用）
USE_COLOR = sys.stdout.isatty() and os.environ.get("NO_COLOR") is None

logger = logging.getLogger(__name__)


def colorize(text: str, color_code: str) -> str:
    if not USE_COLOR:
        return text
    return f"\033[{color_code}m{text}\033[0m"


def green(text: str) -> str:
    return colorize(text, "32")


def yellow(text: str) -> str:
    return colorize(text, "33")


def cyan(text: str) -> str:
    return colorize(text, "36")


def red(text: str) -> str:
    return colorize(text, "31")


def bold(text: str) -> str:
    return colorize(text, "1")


def dim(text: str) -> str:
    return colorize(text, "2")


# 目标仓库定义
REPOS_CONFIG: dict[str, dict[str, Any]] = {
    "alist": {
        "display_name": "AList",
        "dir_candidates": ["alist"],
        "default_dir": "alist",
        "url": "https://github.com/AlistGo/alist.git",
        "semver_pattern": re.compile(r"^v?\d+\.\d+(?:\.\d+)?"),
    },
    "openlist": {
        "display_name": "OpenList",
        "dir_candidates": ["OpenList", "openlist"],
        "default_dir": "OpenList",
        "url": "https://github.com/OpenListTeam/OpenList.git",
        "semver_pattern": re.compile(r"^v?\d+\.\d+(?:\.\d+)?"),
    },
}


def find_workspace_root() -> Path:
    """自动向上查找工作区根目录（以 Cargo.toml 为定位准则）。"""
    current = Path(__file__).resolve().parent
    while current != current.parent:
        if (current / "Cargo.toml").exists() and (current / "examples").exists():
            return current
        current = current.parent
    return Path.cwd()


def get_repo_dir(workspace_root: Path, repo_key: str) -> Path:
    config = REPOS_CONFIG[repo_key]
    examples_dir = workspace_root / "examples"
    for cand in config["dir_candidates"]:
        target = examples_dir / cand
        if target.exists():
            return target
    return examples_dir / config["default_dir"]


def run_git(
    args: list[str], cwd: Path, log_error: bool = True
) -> subprocess.CompletedProcess[str]:
    """运行 Git 子命令并捕获输出，防止子进程输出污染 stdout。"""
    try:
        return subprocess.run(
            ["git", *args],
            cwd=cwd,
            text=True,
            capture_output=True,
            check=True,
        )
    except subprocess.CalledProcessError as e:
        if e.stderr and log_error:
            logger.error("Git 错误: %s", e.stderr.strip())
        raise


def is_git_repo(path: Path) -> bool:
    return path.exists() and (path / ".git").exists()


def ensure_repo_cloned(workspace_root: Path, repo_key: str) -> Path:
    repo_dir = get_repo_dir(workspace_root, repo_key)
    config = REPOS_CONFIG[repo_key]

    if is_git_repo(repo_dir):
        return repo_dir

    logger.info(
        "检测到 %s 尚未克隆，正在从 %s 克隆...",
        config["display_name"],
        config["url"],
    )
    repo_dir.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["git", "clone", "--depth", "50", "--tags", config["url"], str(repo_dir)],
        check=True,
        capture_output=True,
        text=True,
    )
    logger.info("克隆完成: %s", repo_dir)
    return repo_dir


def get_repo_status(
    repo_dir: Path, semver_pattern: re.Pattern[str] | None = None
) -> dict[str, Any]:
    if not is_git_repo(repo_dir):
        return {"exists": False}

    # 获取当前精确 Tag（若多个 Tag 指向同一 Commit，优先挑选符合语义规则的稳定版本 Tag）
    exact_tag = ""
    try:
        proc = run_git(["tag", "--points-at", "HEAD"], cwd=repo_dir, log_error=False)
        tags_at_head = [t.strip() for t in proc.stdout.splitlines() if t.strip()]
        if tags_at_head:
            pattern = semver_pattern or re.compile(r"^v?\d+\.\d+(?:\.\d+)?")
            stable = [t for t in tags_at_head if is_stable_release(t, pattern)]
            exact_tag = stable[0] if stable else tags_at_head[0]
    except (subprocess.CalledProcessError, OSError):
        exact_tag = ""

    # 获取当前分支
    branch = ""
    try:
        proc = run_git(
            ["rev-parse", "--abbrev-ref", "HEAD"],
            cwd=repo_dir,
            log_error=False,
        )
        out = proc.stdout.strip()
        branch = "detached HEAD" if out == "HEAD" else out
    except (subprocess.CalledProcessError, OSError):
        branch = "detached HEAD"

    # 获取当前 commit
    commit_sha = ""
    commit_date = ""
    commit_msg = ""
    try:
        proc = run_git(
            ["log", "-1", "--format=%h|%cd|%s", "--date=short"], cwd=repo_dir
        )
        parts = proc.stdout.strip().split("|", 2)
        if len(parts) >= 3:
            commit_sha, commit_date, commit_msg = parts[0], parts[1], parts[2]
    except (subprocess.CalledProcessError, OSError, ValueError):
        pass

    # 工作树状态
    dirty = False
    try:
        proc = run_git(["status", "--porcelain"], cwd=repo_dir)
        dirty = bool(proc.stdout.strip())
    except (subprocess.CalledProcessError, OSError):
        pass

    return {
        "exists": True,
        "path": str(repo_dir),
        "branch": branch,
        "exact_tag": exact_tag,
        "commit_sha": commit_sha,
        "commit_date": commit_date,
        "commit_msg": commit_msg,
        "dirty": dirty,
    }


def fetch_tags(repo_dir: Path) -> bool:
    logger.info("正在获取最新远程 Tags: %s...", repo_dir.name)
    try:
        subprocess.run(
            [
                "git",
                "-C",
                str(repo_dir),
                "fetch",
                "origin",
                "--tags",
                "--force",
                "--prune",
            ],
            check=True,
            capture_output=True,
            text=True,
        )
        return True
    except subprocess.CalledProcessError as e:
        err_detail = e.stderr.strip() if e.stderr else str(e)
        logger.warning("获取远程 Tags 失败 (%s): %s", repo_dir.name, err_detail)
        return False


def get_all_tags(repo_dir: Path) -> list[tuple[str, str, str]]:
    """获取所有 tags 及其提交日期与 SHA，按版本号语义逆序排列。"""
    try:
        proc = run_git(
            [
                "tag",
                "-l",
                "--sort=-v:refname",
                "--format=%(refname:short)|%(creatordate:short)|%(objectname:short)",
            ],
            cwd=repo_dir,
        )
        tags: list[tuple[str, str, str]] = []
        for line in proc.stdout.strip().splitlines():
            line = line.strip()
            if not line:
                continue
            parts = line.split("|")
            tag_name = parts[0]
            date = parts[1] if len(parts) > 1 else ""
            sha = parts[2] if len(parts) > 2 else ""
            tags.append((tag_name, date, sha))
        return tags
    except (subprocess.CalledProcessError, OSError):
        return []


def is_stable_release(tag: str, pattern: re.Pattern[str]) -> bool:
    # 必须匹配主版本号且不含 beta/rc/alpha 等预发布标识
    if not pattern.match(tag):
        return False
    lower = tag.lower()
    return not any(pre in lower for pre in ["beta", "rc", "alpha", "dev", "test"])


def format_repo_status_lines(
    key: str, cfg: dict[str, Any], root: Path, repo_dir: Path, status: dict[str, Any]
) -> list[str]:
    """格式化单个仓库的状态信息行。"""
    lines = [
        f"[{bold(cyan(cfg['display_name']))}] ({dim(str(repo_dir.relative_to(root)))})"
    ]
    if not status.get("exists"):
        lines.append(yellow("  状态: 尚未克隆或目录不存在"))
        return lines

    if status["exact_tag"]:
        ref_desc = f"{green(status['exact_tag'])} (Tag)"
    elif status["branch"] != "detached HEAD":
        ref_desc = f"{cyan(status['branch'])} (分支)"
    else:
        ref_desc = dim("detached HEAD")

    dirty_desc = red(" [有未提交修改]") if status["dirty"] else green(" [干净]")
    lines.append(f"  当前检出: {ref_desc} @ {bold(status['commit_sha'])}{dirty_desc}")
    lines.append(f"  最新提交: {status['commit_date']} - {status['commit_msg']}")

    tags = get_all_tags(repo_dir)
    stable_tags = [t[0] for t in tags if is_stable_release(t[0], cfg["semver_pattern"])]
    if stable_tags:
        latest_stable = stable_tags[0]
        if status["exact_tag"] == latest_stable:
            lines.append(f"  稳定版本: {green(latest_stable)} (当前已是最新稳定 Tag)")
        else:
            lines.append(
                f"  最新稳定: {yellow(latest_stable)} (可运行: update {key} --latest)"
            )
    return lines


# ---------------------------------------------------------------------------
# 命令实现
# ---------------------------------------------------------------------------


def cmd_status(_args: argparse.Namespace) -> None:
    root = find_workspace_root()
    output_blocks: list[str] = [bold("=== @examples/ 依赖源码状态 ===")]

    for key, cfg in REPOS_CONFIG.items():
        repo_dir = get_repo_dir(root, key)
        status = get_repo_status(repo_dir, cfg.get("semver_pattern"))
        if not status.get("exists"):
            logger.warning("%s 尚未克隆或目录不存在: %s", cfg["display_name"], repo_dir)
        block_lines = format_repo_status_lines(key, cfg, root, repo_dir, status)
        output_blocks.append("\n".join(block_lines))

    print("\n\n".join(output_blocks))


def cmd_list(args: argparse.Namespace) -> None:
    root = find_workspace_root()
    target_keys = [args.target] if args.target else list(REPOS_CONFIG.keys())
    output_blocks: list[str] = []

    for key in target_keys:
        cfg = REPOS_CONFIG[key]
        try:
            repo_dir = ensure_repo_cloned(root, key)
        except (subprocess.CalledProcessError, OSError) as e:
            logger.error("克隆 %s 失败: %s", cfg["display_name"], e)
            sys.exit(1)

        if args.remote:
            fetch_tags(repo_dir)

        status = get_repo_status(repo_dir, cfg.get("semver_pattern"))
        current_tag = status.get("exact_tag", "")
        all_tags = get_all_tags(repo_dir)

        display_name = cfg["display_name"]
        block_lines: list[str] = [bold(cyan(f"=== {display_name} 可用版本列表 ==="))]
        if not all_tags:
            logger.warning("%s 暂无可用 Tag", display_name)
            block_lines.append(yellow("  暂无可用 Tag。"))
            output_blocks.append("\n".join(block_lines))
            continue

        # 过滤展示
        if args.all_tags:
            display_tags = all_tags
        else:
            display_tags = [
                t for t in all_tags if is_stable_release(t[0], cfg["semver_pattern"])
            ]
            if not display_tags:
                display_tags = all_tags  # 若无匹配则回退展示全部

        total_count = len(display_tags)
        limit = None if args.all else (args.limit or 15)
        sliced_tags = display_tags[:limit] if limit else display_tags

        for tag_name, date, sha in sliced_tags:
            is_current = tag_name == current_tag
            marker = green(" * (当前)") if is_current else "   "
            tag_label = bold(green(tag_name)) if is_current else cyan(tag_name)
            date_str = dim(f"[{date}]") if date else ""
            sha_str = dim(f"({sha})") if sha else ""
            block_lines.append(f"{marker} {tag_label:<14} {date_str:<12} {sha_str}")

        if limit and total_count > limit:
            logger.info(
                "%s 其余 %d 个历史版本已省略，使用 --all 显示全部版本。",
                display_name,
                total_count - limit,
            )

        output_blocks.append("\n".join(block_lines))

    print("\n\n".join(output_blocks))


def cmd_update(args: argparse.Namespace) -> None:
    root = find_workspace_root()
    target_keys = [args.target] if args.target else list(REPOS_CONFIG.keys())
    output_blocks: list[str] = []

    for key in target_keys:
        cfg = REPOS_CONFIG[key]
        try:
            repo_dir = ensure_repo_cloned(root, key)
        except (subprocess.CalledProcessError, OSError) as e:
            logger.error("克隆 %s 失败: %s", cfg["display_name"], e)
            sys.exit(1)

        # 1. 默认先获取最新远程更新
        if not args.no_fetch:
            fetch_tags(repo_dir)

        tags = get_all_tags(repo_dir)
        target_ref = args.version

        # 2. 解析目标版本/分支
        if args.latest:
            stable_tags = [
                t[0] for t in tags if is_stable_release(t[0], cfg["semver_pattern"])
            ]
            if not stable_tags:
                logger.error(
                    "%s 未找到符合规则的稳定 Release Tag！", cfg["display_name"]
                )
                sys.exit(1)
            target_ref = stable_tags[0]
        elif args.main:
            target_ref = "origin/main"
        elif not target_ref:
            logger.error(
                "请为 %s 指定要更新的版本 (例如: update %s v3.64.0)，或使用 --latest / --main 参数。",
                cfg["display_name"],
                key,
            )
            sys.exit(1)

        # 3. 检查是否有未提交修改
        status = get_repo_status(repo_dir, cfg.get("semver_pattern"))
        if status.get("dirty") and not args.force:
            logger.error(
                "%s 存在未提交修改，为防止丢失代码已停止切换。使用 -f / --force 强制覆盖。",
                cfg["display_name"],
            )
            sys.exit(1)

        logger.info("正在将 %s 切换至: %s...", cfg["display_name"], target_ref)

        # 4. 执行切换
        try:
            if args.force:
                run_git(["reset", "--hard", "HEAD"], cwd=repo_dir)
            run_git(["checkout", target_ref], cwd=repo_dir)

            # 如果检出的是本地分支并且追踪远端，做一次 pull
            cur_status = get_repo_status(repo_dir, cfg.get("semver_pattern"))
            if cur_status["branch"] not in ["", "detached HEAD"] and not args.version:
                try:
                    run_git(["pull", "--ff-only"], cwd=repo_dir, log_error=False)
                except (subprocess.CalledProcessError, OSError):
                    pass
                cur_status = get_repo_status(repo_dir, cfg.get("semver_pattern"))

            logger.info("%s 已成功更新至: %s", cfg["display_name"], target_ref)
            block_lines = format_repo_status_lines(key, cfg, root, repo_dir, cur_status)
            output_blocks.append("\n".join(block_lines))
        except (subprocess.CalledProcessError, OSError) as e:
            logger.error("更新 %s 失败: %s", cfg["display_name"], e)
            sys.exit(1)

    print("\n\n".join(output_blocks))


def main() -> None:
    logging.basicConfig(
        level=logging.INFO,
        format="[%(levelname)s] %(message)s",
        stream=sys.stderr,
    )

    parser = argparse.ArgumentParser(
        prog="update-examples",
        description="管理与更新 @examples/ 下 AList 与 OpenList 源码依赖版本的统一 CLI 工具。",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
常用示例:
  # 1. 查看当前两者的检出版本状态
  uv run scripts/update-examples.py status

  # 2. 列出 AList 可用版本列表
  uv run scripts/update-examples.py list alist

  # 3. 列出 OpenList 所有可用版本 (包括旧版与预发布)
  uv run scripts/update-examples.py list openlist --all-tags

  # 4. 更新 AList 至指定版本 Tag
  uv run scripts/update-examples.py update alist v3.64.0

  # 5. 一键更新两者到最新的稳定 Release Tag
  uv run scripts/update-examples.py update --latest

  # 6. 一键更新两者到最新的远程 main 主分支
  uv run scripts/update-examples.py update --main
        """,
    )

    subparsers = parser.add_subparsers(dest="command", help="子命令")

    # status 子命令
    subparsers.add_parser("status", help="显示当前 alist 和 openlist 的检出版本与状态")

    # list 子命令
    list_parser = subparsers.add_parser("list", help="列出可用的版本 Tags")
    list_parser.add_argument(
        "target",
        nargs="?",
        choices=["alist", "openlist"],
        help="目标组件 (可选: alist / openlist，缺省列出两者)",
    )
    list_parser.add_argument(
        "-r",
        "--remote",
        action="store_true",
        help="在列出前先 fetch 远端以获取最新 Tags",
    )
    list_parser.add_argument(
        "-n",
        "--limit",
        type=int,
        default=15,
        help="限制显示的最新版本数量 (默认 15 条)",
    )
    list_parser.add_argument(
        "-a",
        "--all",
        action="store_true",
        help="显示全部版本列表，不限制条数",
    )
    list_parser.add_argument(
        "--all-tags",
        action="store_true",
        help="包含 beta/rc 等预发布 Tags (默认仅显示正规稳定版)",
    )

    # update 子命令
    update_parser = subparsers.add_parser("update", help="更新或切换版本")
    update_parser.add_argument(
        "target",
        nargs="?",
        choices=["alist", "openlist"],
        help="目标组件 (可选: alist / openlist，缺省时结合 --latest/--main 同时更新两者)",
    )
    update_parser.add_argument(
        "version",
        nargs="?",
        help="要检出的版本 Tag、分支名或 Commit SHA (例如: v3.64.0)",
    )
    update_parser.add_argument(
        "--latest",
        action="store_true",
        help="一键更新至最新的稳定 Release Tag",
    )
    update_parser.add_argument(
        "--main",
        action="store_true",
        help="一键更新至远程 origin/main 分支最新代码",
    )
    update_parser.add_argument(
        "--no-fetch",
        action="store_true",
        help="切换前不从远端 fetch (仅使用本地已下载的 Tags/Commits)",
    )
    update_parser.add_argument(
        "-f",
        "--force",
        action="store_true",
        help="强制切换并丢弃工作区可能存在的修改",
    )

    args = parser.parse_args()

    # 缺省无子命令时，默认运行 status
    if not args.command or args.command == "status":
        cmd_status(args)
    elif args.command == "list":
        cmd_list(args)
    elif args.command == "update":
        cmd_update(args)


if __name__ == "__main__":
    main()
