#!/usr/bin/env uv run
# /// script
# requires-python = ">=3.10"
# dependencies = [
#     "pyyaml>=6.0",
# ]
# ///
"""
AList 与 OpenList OpenAPI 文档自动抓取与更新工具。

支持从 Apifox 在线公开文档站点抓取最新 OpenAPI 规范数据并更新到 docs/api/ 目录：
- AList:    https://alist-public.apifox.cn/ -> docs/api/alistv3.openapi.yaml
- OpenList: https://fox.oplist.org/         -> docs/api/openlistv4.openapi.yaml
"""

import argparse
import json
import os
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


import yaml

# 终端彩色输出
USE_COLOR = sys.stdout.isatty() and os.environ.get("NO_COLOR") is None


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


# 站点配置定义
PROJECTS_CONFIG = {
    "alist": {
        "display_name": "AList",
        "site_url": "https://alist-public.apifox.cn/",
        "default_project_id": 5461134,
        # api.apifox.cn 是官方极速后端节点，可规避前端站点反代超时
        "api_hosts": [
            "https://api.apifox.cn",
            "https://api.apifox.com",
            "https://alist-public.apifox.cn",
        ],
        "output_yaml": "docs/api/alistv3.openapi.yaml",
        "output_json": "docs/api/alistv3.openapi.json",
    },
    "openlist": {
        "display_name": "OpenList",
        "site_url": "https://fox.oplist.org/",
        "default_project_id": 5473311,
        "api_hosts": [
            "https://fox.oplist.org",
            "https://api.apifox.cn",
            "https://api.apifox.com",
        ],
        "output_yaml": "docs/api/openlistv4.openapi.yaml",
        "output_json": "docs/api/openlistv4.openapi.json",
    },
}


def find_workspace_root() -> Path:
    """自动查找项目根目录（以 Cargo.toml 为定位准则）。"""
    current = Path(__file__).resolve().parent
    while current != current.parent:
        if (current / "Cargo.toml").exists() and (current / "docs").exists():
            return current
        current = current.parent
    return Path.cwd()


def extract_project_id_from_site(site_url: str) -> Optional[int]:
    """尝试从 Apifox 前端 HTML 中动态提取项目 ID。"""
    try:
        req = urllib.request.Request(
            site_url,
            headers={
                "User-Agent": (
                    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
                    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
                ),
            },
        )
        with urllib.request.urlopen(req, timeout=6) as response:
            html = response.read().decode("utf-8", errors="ignore")
            # 常见格式: "id":5461134 或 id: 5461134
            matches = re.findall(r'"id":\s*(\d{6,10})', html)
            if matches:
                return int(matches[0])
    except Exception:
        pass
    return None


def fetch_openapi_data(project_id: int, api_hosts: List[str], spec_version: str = "openapi31") -> Dict[str, Any]:
    """向 Apifox 后端接口请求导出 OpenAPI 数据。"""
    body_data = json.dumps({
        "type": "openapi",
        "version": spec_version,
        "id": str(project_id),
    }).encode("utf-8")

    last_error = None
    for host in api_hosts:
        endpoint = f"{host.rstrip('/')}/api/v1/projects/{project_id}/published-projects/export-data"
        try:
            req = urllib.request.Request(
                endpoint,
                data=body_data,
                headers={
                    "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
                    "Content-Type": "application/json;charset=UTF-8",
                    "Accept": "application/json",
                },
            )
            with urllib.request.urlopen(req, timeout=12) as resp:
                if resp.status == 200:
                    raw_bytes = resp.read()
                    data = json.loads(raw_bytes.decode("utf-8"))
                    return data
        except Exception as e:
            last_error = e
            continue

    raise RuntimeError(f"从所有可用节点导出数据均失败: {last_error}")


def save_as_json(data: Dict[str, Any], target_path: Path) -> None:
    target_path.parent.mkdir(parents=True, exist_ok=True)
    with open(target_path, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)


def save_as_yaml(data: Dict[str, Any], target_path: Path) -> None:
    if yaml is None:
        raise RuntimeError("缺少 pyyaml 模块，无法输出 yaml 格式，请使用 uv run 运行该脚本。")

    target_path.parent.mkdir(parents=True, exist_ok=True)

    class CustomDumper(yaml.SafeDumper):
        pass

    # 保持中文正常显示，不转义为 \uXXXX
    def represent_str(dumper, value):
        if "\n" in value:
            return dumper.represent_scalar("tag:yaml.org,2002:str", value, style="|")
        return dumper.represent_scalar("tag:yaml.org,2002:str", value)

    CustomDumper.add_representer(str, represent_str)

    with open(target_path, "w", encoding="utf-8") as f:
        yaml.dump(
            data,
            f,
            Dumper=CustomDumper,
            allow_unicode=True,
            sort_keys=False,
            indent=2,
            width=120,
        )


def format_size(bytes_len: int) -> str:
    if bytes_len < 1024:
        return f"{bytes_len} B"
    elif bytes_len < 1024 * 1024:
        return f"{bytes_len / 1024:.1f} KB"
    else:
        return f"{bytes_len / (1024 * 1024):.2f} MB"


def update_target(target_key: str, cfg: Dict[str, Any], root: Path, args: argparse.Namespace) -> bool:
    name = cfg["display_name"]
    print(f"\n{bold(cyan(f'=== 正在同步 {name} OpenAPI 文档 ==='))}")
    print(f"  来源站点: {dim(cfg['site_url'])}")

    # 1. 获取项目 ID
    pid = None
    if not args.fixed_id:
        pid = extract_project_id_from_site(cfg["site_url"])
    if not pid:
        pid = cfg["default_project_id"]
    print(f"  项目 ID  : {bold(str(pid))}")

    # 2. 拉取 OpenAPI 数据
    print(f"  正在请求导出 OpenAPI 数据 (版本: {args.version})...")
    spec_ver_code = "openapi31" if args.version == "3.1" else "openapi30"
    try:
        data = fetch_openapi_data(pid, cfg["api_hosts"], spec_ver_code)
    except Exception as e:
        print(red(f"  [!] 数据导出失败: {e}"))
        return False

    spec_title = data.get("info", {}).get("title", "未命名")
    paths_count = len(data.get("paths", {}))
    schemas_count = len(data.get("components", {}).get("schemas", {}))
    openapi_ver = data.get("openapi", "未知")

    print(green(f"  [✓] 成功拉取 OpenAPI {openapi_ver} 数据！"))
    print(f"      - 标题: {spec_title}")
    print(f"      - 接口数量: {bold(str(paths_count))} 个路径")
    print(f"      - 数据模型: {bold(str(schemas_count))} 个 Schema 定义")

    if args.dry_run:
        print(yellow("  [提示] --dry-run 模式已开启，跳过写入本地文件。"))
        return True

    # 3. 保存文件
    yaml_path = root / cfg["output_yaml"]
    json_path = root / cfg["output_json"]

    if args.format in ["yaml", "both"]:
        print(f"  正在写入 YAML: {dim(str(yaml_path.relative_to(root)))}...")
        try:
            save_as_yaml(data, yaml_path)
            size = yaml_path.stat().st_size
            print(green(f"  [✓] YAML 文件已保存 ({format_size(size)})"))
        except Exception as e:
            print(red(f"  [!] 写入 YAML 失败: {e}"))
            return False

    if args.format in ["json", "both"]:
        print(f"  正在写入 JSON: {dim(str(json_path.relative_to(root)))}...")
        try:
            save_as_json(data, json_path)
            size = json_path.stat().st_size
            print(green(f"  [✓] JSON 文件已保存 ({format_size(size)})"))
        except Exception as e:
            print(red(f"  [!] 写入 JSON 失败: {e}"))
            return False

    return True


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="update-openapi",
        description="从 Apifox 在线文档站点自动抓取并更新 AList 与 OpenList 的 OpenAPI 规范文档。",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
常用示例:
  # 1. 默认更新两者全部文档为 YAML 格式 (保存至 docs/api/)
  uv run scripts/update-openapi.py

  # 2. 仅更新 AList 文档
  uv run scripts/update-openapi.py alist

  # 3. 仅更新 OpenList 文档
  uv run scripts/update-openapi.py openlist

  # 4. 同时输出 YAML 与 JSON 两种格式
  uv run scripts/update-openapi.py --format both

  # 5. 试运行模式，仅拉取检查接口统计，不覆盖本地文件
  uv run scripts/update-openapi.py --dry-run
        """,
    )

    parser.add_argument(
        "target",
        nargs="?",
        choices=["all", "alist", "openlist"],
        default="all",
        help="目标组件 (默认: all，同时更新 alist 与 openlist)",
    )
    parser.add_argument(
        "--format",
        choices=["yaml", "json", "both"],
        default="yaml",
        help="输出格式 (默认: yaml，符合 docs/api/ 目录规范)",
    )
    parser.add_argument(
        "--version",
        choices=["3.1", "3.0"],
        default="3.1",
        help="OpenAPI 规范版本 (默认: 3.1)",
    )
    parser.add_argument(
        "--fixed-id",
        action="store_true",
        help="直接使用预设的项目 ID，跳过网页动态探测",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="试运行模式，拉取并验证接口统计，但不写入或覆盖本地文件",
    )

    args = parser.parse_args()
    root = find_workspace_root()

    targets = list(PROJECTS_CONFIG.keys()) if args.target == "all" else [args.target]

    print(bold("=== OpenAPI 文档同步任务开始 ==="))
    success_count = 0
    for t in targets:
        if update_target(t, PROJECTS_CONFIG[t], root, args):
            success_count += 1

    print("\n" + bold("=== 同步任务结束 ==="))
    if success_count == len(targets):
        print(green(f"[✓] 全部 {success_count} 个文档同步成功！"))
    else:
        print(yellow(f"[!] 完成: {success_count}/{len(targets)} 成功。"))


if __name__ == "__main__":
    main()
