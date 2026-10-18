//! public 公共域数据模型。
//!
//! 建模无需认证的 public 端点响应：
//! `GET /api/public/settings` 返回的站点设置——服务端以字符串键值映射
//! 返回全部公共设置项（服务端 `op.GetPublicSettingsMap()` 的
//! `map[string]string`；公共项按 `PUBLIC`/`READONLY` 标记过滤），因此主模型
//! 为开放的 [`PublicSettings`]（`HashMap<String, String>`）；
//! [`KnownPublicSettings`] 为 AList OpenAPI 规范的
//! `/api/public/settings` 响应已记录键的具名辅助结构。
//! `GET /api/public/offline_download_tools` 与
//! `GET /api/public/archive_extensions` 返回的字符串数组分别建模为
//! [`OfflineDownloadTools`] 与 [`ArchiveExtensions`]。
//! 字段形状以 AList OpenAPI 规范与服务端源码为准。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// [`/api/public/settings`](crate::endpoint::public::Public::settings) 返回的
/// 站点设置集合（响应 `data`）。
///
/// 对应服务端 `op.GetPublicSettingsMap()` 的 `map[string]string`：AList 把所有设置项
/// 一律以字符串序列化（布尔值是 `"true"`/`"false"`，数字是 `"30"` 这样的十进制文本），
/// 且键集合随版本增减，故此处有意保持开放，不做封闭结构建模。
pub type PublicSettings = HashMap<String, String>;

/// 公共设置中 openapi 已记录的具名键。
///
/// [`PublicSettings`] 是开放的字符串映射；需要按名访问常用键时，可把它反序列化为本结构：
/// 未记录的键会被忽略（serde 默认行为），缺失的键为 [`Option::None`]。
/// 键清单与类型（均为字符串）来源：AList OpenAPI 规范的
/// `/api/public/settings` 响应字段表。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownPublicSettings {
    /// 是否允许建立站点索引（字符串化的布尔值，如 `"false"`）。
    #[serde(default)]
    pub allow_indexed: Option<String>,
    /// 是否允许挂载（字符串化的布尔值）。
    #[serde(default)]
    pub allow_mounted: Option<String>,
    /// 站点公告。
    ///
    /// 未设置时为空字符串。
    #[serde(default)]
    pub announcement: Option<String>,
    /// 音频是否自动播放（字符串化的布尔值）。
    #[serde(default)]
    pub audio_autoplay: Option<String>,
    /// 音频封面地址。
    #[serde(default)]
    pub audio_cover: Option<String>,
    /// 是否自动更新索引（字符串化的布尔值）。
    #[serde(default)]
    pub auto_update_index: Option<String>,
    /// 默认分页大小（十进制字符串，如 `"30"`）。
    #[serde(default)]
    pub default_page_size: Option<String>,
    /// 外部预览配置（JSON 字符串）。
    #[serde(default)]
    pub external_previews: Option<String>,
    /// 网站图标地址。
    #[serde(default)]
    pub favicon: Option<String>,
    /// 文件名字符映射规则（JSON 字符串，如 `"{\"/\": \"|\"}"`）。
    #[serde(default)]
    pub filename_char_mapping: Option<String>,
    /// 是否向直链转发查询参数（字符串化的布尔值）。
    #[serde(default)]
    pub forward_direct_link_params: Option<String>,
    /// 隐藏文件规则（正则文本，如 `/\/README.md/i`）。
    #[serde(default)]
    pub hide_files: Option<String>,
    /// 主页容器样式（如 `hope_container`）。
    #[serde(default)]
    pub home_container: Option<String>,
    /// 主页图标（如 `🏠`）。
    #[serde(default)]
    pub home_icon: Option<String>,
    /// iframe 预览配置（JSON 字符串）。
    #[serde(default)]
    pub iframe_previews: Option<String>,
    /// 站点 logo 地址。
    #[serde(default)]
    pub logo: Option<String>,
    /// 主题颜色（如 `"#1890ff"`）。
    #[serde(default)]
    pub main_color: Option<String>,
    /// OCR 接口地址。
    #[serde(default)]
    pub ocr_api: Option<String>,
    /// 是否启用打包下载（字符串化的布尔值）。
    #[serde(default)]
    pub package_download: Option<String>,
    /// 前端分页类型（示例值 `all`）。
    #[serde(default)]
    pub pagination_type: Option<String>,
    /// robots.txt 内容（多行文本）。
    #[serde(default)]
    pub robots_txt: Option<String>,
    /// 搜索索引类型（示例值 `none`）。
    #[serde(default)]
    pub search_index: Option<String>,
    /// 设置页面布局（示例值 `responsive`）。
    #[serde(default)]
    pub settings_layout: Option<String>,
    /// 站点标题。
    #[serde(default)]
    pub site_title: Option<String>,
    /// 是否启用 SSO 登录（字符串化的布尔值）。
    #[serde(default)]
    pub sso_login_enabled: Option<String>,
    /// SSO 登录平台名称。
    ///
    /// 未配置时为空字符串。
    #[serde(default)]
    pub sso_login_platform: Option<String>,
    /// 服务端版本号（如 `v3.25.1`）。
    #[serde(default)]
    pub version: Option<String>,
    /// 视频是否自动播放（字符串化的布尔值）。
    #[serde(default)]
    pub video_autoplay: Option<String>,
}

/// [`/api/public/offline_download_tools`](crate::endpoint::public::Public::offline_download_tools)
/// 返回的离线下载工具名列表（响应 `data`）。
///
/// 对应服务端 offline download 工具管理器就绪工具名的 `[]string`：
/// 仅包含服务端判定就绪（已完成初始化）的工具，且已按字典序排序，
/// 如 `115 Cloud`、`aria2`、`qBittorrent`、`Transmission`、`PikPak` 等；
/// 工具集合随服务端编译内置与配置状态变化，全部未就绪时为空数组。
pub type OfflineDownloadTools = Vec<String>;

/// [`/api/public/archive_extensions`](crate::endpoint::public::Public::archive_extensions)
/// 返回的压缩包扩展名列表（响应 `data`）。
///
/// 对应服务端 archive 工具注册表的键集合 `[]string`：即已注册解压驱动
/// 支持的扩展名，自带前导点号（如 `.7z`、`.iso`、`.rar`、`.tar`）；
/// 集合由服务端编译内置的解压驱动决定，且无序（Go map 遍历顺序不固定）。
pub type ArchiveExtensions = Vec<String>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::Response;

    /// AList OpenAPI 规范中的 `/api/public/settings` 200 响应示例。
    fn openapi_settings_example() -> serde_json::Value {
        serde_json::json!({
            "allow_indexed": "false",
            "allow_mounted": "false",
            "announcement": "",
            "audio_autoplay": "true",
            "audio_cover": "https://jsd.nn.ci/gh/alist-org/logo@main/logo.svg",
            "auto_update_index": "false",
            "default_page_size": "30",
            "external_previews": "{}",
            "favicon": "https://cdn.jsdelivr.net/gh/alist-org/logo@main/logo.svg",
            "filename_char_mapping": "{\"/\": \"|\"}",
            "forward_direct_link_params": "false",
            "hide_files": "/\\/README.md/i",
            "home_container": "hope_container",
            "home_icon": "🏠",
            "iframe_previews": "{\n\t\"doc,docx,xls,xlsx,ppt,pptx\": {\n\t\t\"Microsoft\":\"https://view.officeapps.live.com/op/view.aspx?src=$e_url\",\n\t\t\"Google\":\"https://docs.google.com/gview?url=$e_url&embedded=true\"\n\t},\n\t\"pdf\": {\n\t\t\"PDF.js\":\"https://alist-org.github.io/pdf.js/web/viewer.html?file=$e_url\"\n\t},\n\t\"epub\": {\n\t\t\"EPUB.js\":\"https://alist-org.github.io/static/epub.js/viewer.html?url=$e_url\"\n\t}\n}",
            "logo": "https://cdn.jsdelivr.net/gh/alist-org/logo@main/logo.svg",
            "main_color": "#1890ff",
            "ocr_api": "https://api.nn.ci/ocr/file/json",
            "package_download": "true",
            "pagination_type": "all",
            "robots_txt": "User-agent: *\nAllow: /",
            "search_index": "none",
            "settings_layout": "responsive",
            "site_title": "AList",
            "sso_login_enabled": "false",
            "sso_login_platform": "",
            "version": "v3.25.1",
            "video_autoplay": "true"
        })
    }

    /// 1) 正向钉扎：OpenAPI 示例 JSON 按响应封装解码为 `PublicSettings` 映射。
    #[test]
    fn public_settings_map_decodes_openapi_example() {
        let resp: Response<PublicSettings> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": openapi_settings_example()
        }))
        .unwrap();

        assert_eq!(resp.code, 200);
        let settings = resp.data;
        assert_eq!(settings.len(), 28);
        assert_eq!(settings["allow_indexed"], "false");
        assert_eq!(settings["default_page_size"], "30");
        assert_eq!(settings["site_title"], "AList");
        assert_eq!(settings["version"], "v3.25.1");
        assert_eq!(settings["robots_txt"], "User-agent: *\nAllow: /");
        // 空字符串值与缺失键不同：公告未设置时是显式空字符串
        assert_eq!(settings["announcement"], "");
        assert_eq!(settings["sso_login_platform"], "");
    }

    /// 1b) 具名辅助结构从同一示例解码，逐字段断言。
    #[test]
    fn known_public_settings_decodes_openapi_example() {
        let known: KnownPublicSettings =
            serde_json::from_value(openapi_settings_example()).unwrap();

        assert_eq!(known.allow_indexed.as_deref(), Some("false"));
        assert_eq!(known.allow_mounted.as_deref(), Some("false"));
        assert_eq!(known.announcement.as_deref(), Some(""));
        assert_eq!(known.audio_autoplay.as_deref(), Some("true"));
        assert_eq!(known.default_page_size.as_deref(), Some("30"));
        assert_eq!(
            known.filename_char_mapping.as_deref(),
            Some("{\"/\": \"|\"}")
        );
        assert_eq!(known.home_icon.as_deref(), Some("🏠"));
        assert_eq!(known.robots_txt.as_deref(), Some("User-agent: *\nAllow: /"));
        assert_eq!(known.site_title.as_deref(), Some("AList"));
        assert_eq!(known.sso_login_platform.as_deref(), Some(""));
        assert_eq!(known.version.as_deref(), Some("v3.25.1"));
    }

    /// 2) 兼容钉扎：未知键被忽略，缺失键为 `None`（设置键集合随版本增减）。
    #[test]
    fn known_public_settings_tolerates_missing_and_unknown_keys() {
        let known: KnownPublicSettings = serde_json::from_value(serde_json::json!({
            "site_title": "AList",
            "future_key": "future-value" // 新版本新增、本结构未记录的键
        }))
        .unwrap();

        assert_eq!(known.site_title.as_deref(), Some("AList"));
        assert_eq!(known.allow_indexed, None);
        assert_eq!(known.version, None);
        assert_eq!(
            known,
            KnownPublicSettings {
                site_title: Some("AList".to_owned()),
                ..KnownPublicSettings::default()
            }
        );
    }

    /// 3) 序列化键名钉扎：具名结构序列化后键名与 API 一致（无需 rename）。
    #[test]
    fn known_public_settings_serializes_with_api_field_names() {
        let known = KnownPublicSettings {
            site_title: Some("AList".to_owned()),
            default_page_size: Some("30".to_owned()),
            ..KnownPublicSettings::default()
        };
        let value = serde_json::to_value(&known).unwrap();
        let map = value.as_object().unwrap();
        assert_eq!(map.len(), 28, "应包含已记录的全部 28 个键: {map:?}");
        assert_eq!(value["site_title"], "AList");
        assert_eq!(value["default_page_size"], "30");
    }

    /// 正向钉扎：`/api/public/offline_download_tools` 信封响应解码为就绪工具名列表。
    ///
    /// 示例形状取自 AList 服务端 offline download 工具管理器的实际返回
    /// （按字典序排序的就绪工具名；`115 Cloud` 等含空格的原生工具名按原样返回）。
    #[test]
    fn offline_download_tools_decodes_envelope() {
        let resp: Response<OfflineDownloadTools> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": ["115 Cloud", "aria2", "qBittorrent"]
        }))
        .unwrap();

        assert_eq!(resp.code, 200);
        assert_eq!(
            resp.data,
            vec![
                "115 Cloud".to_owned(),
                "aria2".to_owned(),
                "qBittorrent".to_owned()
            ]
        );
    }

    /// 空列表钉扎：全部工具未就绪（如 aria2 未配置）时服务端返回空数组——
    /// Go `make([]string, 0, ...)` 序列化为 `[]` 而非 `null`。
    /// （本端点 `data` 为裸数组、无可选字段；引入前的老版本服务端无此路由，
    /// 请求将得到 404 错误而非缺字段响应。）
    #[test]
    fn offline_download_tools_decodes_empty_list() {
        let resp: Response<OfflineDownloadTools> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": []
        }))
        .unwrap();

        assert!(resp.data.is_empty());
    }

    /// 正向钉扎：`/api/public/archive_extensions` 信封响应解码为扩展名列表。
    ///
    /// 示例集合取自 AList 服务端内置解压驱动注册的扩展名（sevenzip 的 `.7z`、
    /// iso9660 的 `.iso`、archives 驱动的 `.tar` 等），元素自带前导点号。
    #[test]
    fn archive_extensions_decodes_envelope() {
        let resp: Response<ArchiveExtensions> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": [".7z", ".iso", ".rar", ".tar"]
        }))
        .unwrap();

        assert_eq!(resp.code, 200);
        assert_eq!(
            resp.data,
            vec![
                ".7z".to_owned(),
                ".iso".to_owned(),
                ".rar".to_owned(),
                ".tar".to_owned()
            ]
        );
    }

    /// 空列表钉扎：`data` 为空数组时归约为空列表。
    ///
    /// 注意：注册表为空时服务端（Go `var ext []string` 的 nil 切片）实际返回
    /// `"data": null`，本模型无法将其解码为 `Vec<String>`；该状态实践中不可达——
    /// 服务端内置解压驱动均在启动时经 init() 注册，注册表恒非空。
    /// 本测试仅钉住空数组形状的解码行为。
    #[test]
    fn archive_extensions_decodes_empty_list() {
        let resp: Response<ArchiveExtensions> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": []
        }))
        .unwrap();

        assert!(resp.data.is_empty());
    }
}
