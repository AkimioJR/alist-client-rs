//! admin-storage 存储域数据模型。
//!
//! 覆盖 `/api/admin/storage/*` 端点的数据模型：存储驱动实例 [`Storage`]
//! （`list`/`get` 响应，同时也是创建/更新端点的请求体形状）与
//! 创建操作返回的 [`StorageCreateResponse`]。
//! ## 字段形状来源
//!
//! 字段形状以 AList 服务端 model.Storage 数据模型（内嵌 `Sort`/`Proxy` 两结构体已平铺）
//! 与存储处理模块为准；示例 JSON 取自 AList OpenAPI 规范的 `/api/admin/storage/*` 分组。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 存储驱动实例。
///
/// 对应 AList 服务端 model.Storage 数据模型；
/// 由 `GET /api/admin/storage/list`（`PageResponse<Storage>` 包裹）与
/// `GET /api/admin/storage/get` 返回。`POST /api/admin/storage/create`、
/// `/update` 的请求体也绑定该结构，端点层通过 [`crate::endpoint::admin`] 的
/// 请求构建器字段逐项构造。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Storage {
    /// 存储 ID。
    ///
    /// 对应服务端主键 ID。
    pub id: u64,
    /// 挂载路径。
    ///
    /// 创建时服务端必填。
    pub mount_path: String,
    /// 排序值。
    pub order: i32,
    /// 存储使用的驱动名称，例如 `Local`。
    pub driver: String,
    /// 缓存过期时间（秒）。
    pub cache_expiration: i32,
    /// 存储状态，例如 `work`。
    ///
    /// 驱动初始化失败时为错误信息。
    pub status: String,
    /// 驱动特定的附加信息，JSON 字符串（字段由各驱动自行定义）。
    pub addition: String,
    /// 备注名。
    pub remark: String,
    /// 最近修改时间。
    pub modified: DateTime<Utc>,
    /// 是否被禁用。
    pub disabled: bool,
    /// 是否禁止建立索引。
    ///
    /// 新版本服务端新增字段，老版本缺失时为 `false`。
    #[serde(default)]
    pub disable_index: bool,
    /// 是否启用签名。
    ///
    /// 新版本服务端新增字段，老版本缺失时为 `false`。
    #[serde(default)]
    pub enable_sign: bool,
    /// 对象排序字段。
    pub order_by: String,
    /// 对象排序方向。
    pub order_direction: String,
    /// 列目录时文件夹的展开时机，例如 `front`。
    pub extract_folder: String,
    /// 是否启用 Web 代理。
    pub web_proxy: bool,
    /// WebDAV 策略。
    ///
    /// 可选值包括 `302_redirect`、`use_proxy_url` 或 `native_proxy`。
    pub webdav_policy: String,
    /// 是否代理 Range 请求。
    ///
    /// 新版本服务端新增字段，老版本缺失时为 `false`。
    #[serde(default)]
    pub proxy_range: bool,
    /// 下载代理 URL。
    pub down_proxy_url: String,
    /// 下载代理 URL 是否附加签名。
    ///
    /// 新版本服务端新增字段，老版本缺失时为 `false`。
    #[serde(default)]
    pub down_proxy_sign: bool,
}

/// 存储创建/更新操作的响应数据。
///
/// 对应 AList 服务端存储处理模块中创建存储以
/// `gin.H{"id": id}` 返回的结构；更新端点成功时服务端实现
/// 返回 `data: null`，而 OpenAPI 规范的示例记载
/// `{ "id": N }`，因此更新端点以 `Option<StorageCreateResponse>` 建模，两种形状均可解码。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCreateResponse {
    /// 创建成功时由服务端分配的存储 ID。
    pub id: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::PageResponse;

    /// 正向钉扎：AList OpenAPI 规范 `/api/admin/storage/list`
    /// 的响应示例（Local 驱动存储）。
    #[test]
    fn storage_decodes_openapi_list_example() {
        // 示例来源：AList OpenAPI 规范 /api/admin/storage/list 200 响应示例
        let storage: Storage = serde_json::from_value(serde_json::json!({
            "id": 1,
            "mount_path": "/lll",
            "order": 0,
            "driver": "Local",
            "cache_expiration": 0,
            "status": "work",
            "addition": "{\"root_folder_path\":\"/root/www\",\"thumbnail\":false,\"thumb_cache_folder\":\"\",\"show_hidden\":true,\"mkdir_perm\":\"777\"}",
            "remark": "",
            "modified": "2023-07-19T09:46:38.868739912+08:00",
            "disabled": false,
            "enable_sign": false,
            "order_by": "name",
            "order_direction": "asc",
            "extract_folder": "front",
            "web_proxy": false,
            "webdav_policy": "native_proxy",
            "down_proxy_url": ""
        }))
        .unwrap();
        assert_eq!(storage.id, 1);
        assert_eq!(storage.mount_path, "/lll");
        assert_eq!(storage.order, 0);
        assert_eq!(storage.driver, "Local");
        assert_eq!(storage.cache_expiration, 0);
        assert_eq!(storage.status, "work");
        assert!(storage.addition.contains("root_folder_path"));
        assert_eq!(storage.remark, "");
        // "2023-07-19T09:46:38.868739912+08:00" → UTC 2023-07-19T01:46:38.868739912
        assert_eq!(storage.modified.timestamp(), 1_689_731_198);
        assert_eq!(storage.modified.timestamp_subsec_nanos(), 868_739_912);
        assert!(!storage.disabled);
        assert!(!storage.enable_sign);
        assert_eq!(storage.order_by, "name");
        assert_eq!(storage.order_direction, "asc");
        assert_eq!(storage.extract_folder, "front");
        assert!(!storage.web_proxy);
        assert_eq!(storage.webdav_policy, "native_proxy");
        assert_eq!(storage.down_proxy_url, "");
        // OpenAPI 规范示例尚未收录的新版本字段缺失 → 默认 false
        assert!(!storage.disable_index);
        assert!(!storage.proxy_range);
        assert!(!storage.down_proxy_sign);
    }

    /// 兼容钉扎：AList OpenAPI 规范 `/api/admin/storage/get`
    /// 的响应示例（Aliyundrive 驱动），并钉住服务端模型新增字段显式出现时的形状。
    #[test]
    fn storage_decodes_openapi_get_example_with_new_fields() {
        // 示例来源：AList OpenAPI 规范 /api/admin/storage/get 200 响应示例；
        // disable_index/proxy_range/down_proxy_sign 取自 AList 服务端 model.Storage 数据模型
        let storage: Storage = serde_json::from_value(serde_json::json!({
            "id": 2,
            "mount_path": "/aa",
            "order": 1,
            "driver": "Aliyundrive",
            "cache_expiration": 30,
            "status": "work",
            "addition": "{\"root_folder_id\":\"\",\"refresh_token\":\"\",\"order_by\":\"size\",\"order_direction\":\"ASC\",\"rapid_upload\":false}",
            "remark": "",
            "modified": "2022-11-26T21:50:44.142348853+08:00",
            "disabled": false,
            "order_by": "",
            "order_direction": "",
            "extract_folder": "front",
            "web_proxy": false,
            "webdav_policy": "302_redirect",
            "down_proxy_url": "",
            "disable_index": true,
            "proxy_range": false,
            "down_proxy_sign": true
        }))
        .unwrap();
        assert_eq!(storage.id, 2);
        assert_eq!(storage.driver, "Aliyundrive");
        assert_eq!(storage.cache_expiration, 30);
        assert_eq!(storage.webdav_policy, "302_redirect");
        assert!(storage.disable_index);
        assert!(!storage.proxy_range);
        assert!(storage.down_proxy_sign);
    }

    /// 兼容钉扎：`list` 端点响应 `data` 的 `PageResponse<Storage>` 分页包裹形态。
    #[test]
    fn list_response_decodes_page_wrapper() {
        // 示例来源：AList OpenAPI 规范 /api/admin/storage/list（content/total 包裹）
        let page: PageResponse<Storage> = serde_json::from_value(serde_json::json!({
            "content": [{
                "id": 1,
                "mount_path": "/lll",
                "order": 0,
                "driver": "Local",
                "cache_expiration": 0,
                "status": "work",
                "addition": "{}",
                "remark": "",
                "modified": "2023-07-19T09:46:38.868739912+08:00",
                "disabled": false,
                "enable_sign": false,
                "order_by": "name",
                "order_direction": "asc",
                "extract_folder": "front",
                "web_proxy": false,
                "webdav_policy": "native_proxy",
                "down_proxy_url": ""
            }],
            "total": 5
        }))
        .unwrap();
        assert_eq!(page.total, 5);
        assert_eq!(page.content.len(), 1);
        assert_eq!(page.content[0].mount_path, "/lll");
    }

    /// 正向钉扎 + 序列化键名：create 端点响应 `data` 为 `{"id": N}`。
    #[test]
    fn create_response_decodes_and_serializes_api_shape() {
        // 示例来源：AList OpenAPI 规范 /api/admin/storage/create 200 响应示例（data: {"id": 7}）
        let resp: StorageCreateResponse =
            serde_json::from_value(serde_json::json!({ "id": 7 })).unwrap();
        assert_eq!(resp.id, 7);
        assert_eq!(
            serde_json::to_value(&resp).unwrap(),
            serde_json::json!({ "id": 7 })
        );
    }
}
