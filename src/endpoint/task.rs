//! 任务管理端点句柄。
//!
//! 覆盖 `/api/task` 下的后台任务管理端点：
//! 包含 7 种子类别（`upload`, `copy`, `offline_download`, `offline_download_transfer`,
//! `s3_transition`, `decompress`, `decompress_upload`），
//! 每种子类别均支持 12 种控制与查询操作：
//! 任务详情、已完成/未完成列表、删除、取消、重试、清空已完成、清空已成功、重试全部失败任务，
//! 以及批量取消、批量删除与批量重试。
//!
//! ## 使用模式
//!
//! 1. 语义化分类链式调用：
//!    ```no_run
//!    # use alist_client::Client;
//!    # async fn example(client: &Client) -> alist_client::Result<()> {
//!    let uploads = client.task().upload().undone().await?;
//!    let copy_tasks = client.task().copy().done().await?;
//!    let info = client.task().offline_download().info("id").await?;
//!    client.task().copy().retry("id").await?;
//!    client.task().upload().clear_done().await?;
//!    client.task().upload().retry_failed().await?;
//!    let failed_map = client.task().upload().cancel_some(["t1", "t2"]).await?;
//!    # Ok(())
//!    # }
//!    ```
//!
//! 2. 通用动态分类入口：
//!    ```no_run
//!    # use alist_client::Client;
//!    # use alist_client::endpoint::task::TaskCategory;
//!    # async fn example(client: &Client) -> alist_client::Result<()> {
//!    let tasks = client.task().category(TaskCategory::Copy).done().await?;
//!    # Ok(())
//!    # }
//!    ```
//!

pub mod action;
pub mod batch;
pub mod empty;
pub mod info;
pub mod list;

pub use crate::schema::task::{BatchTaskResult, TaskCategory, TaskInfo, TaskInfoList};

/// 任务分类子句柄。
///
/// 绑定了具体 [`TaskCategory`] 的任务操作句柄，提供该类别下的全部 12 个 API 端点访问器。
pub struct TaskCategoryHandle<'a> {
    client: &'a crate::Client,
    category: TaskCategory,
}

impl<'a> TaskCategoryHandle<'a> {
    /// 创建任务分类子句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client, category: TaskCategory) -> Self {
        Self { client, category }
    }

    /// 查询单个任务的详细信息。
    ///
    /// 对应 AList `POST /api/task/{category}/info?tid={tid}`。
    ///
    /// # Arguments
    ///
    /// * `tid` - 目标任务 ID，可传 `&str` 或任何 `Into<String>` 的值。
    ///
    /// # Returns
    ///
    /// 返回 [`info::Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`crate::schema::task::TaskInfo`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（如任务不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let info = client.task().offline_download().info("task_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn info(&self, tid: impl Into<String>) -> info::Request<'a> {
        info::Request::new(self.client, self.category, tid)
    }

    /// 获取该类别下的已完成任务列表。
    ///
    /// 对应 AList `GET /api/task/{category}/done`。
    ///
    /// # Returns
    ///
    /// 返回 [`list::Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`crate::schema::task::TaskInfoList`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let tasks = client.task().copy().done().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn done(&self) -> list::Request<'a> {
        list::Request::new(self.client, self.category, "done")
    }

    /// 获取该类别下的未完成任务列表。
    ///
    /// 对应 AList `GET /api/task/{category}/undone`。
    ///
    /// # Returns
    ///
    /// 返回 [`list::Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`crate::schema::task::TaskInfoList`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let tasks = client.task().upload().undone().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn undone(&self) -> list::Request<'a> {
        list::Request::new(self.client, self.category, "undone")
    }

    /// 删除单个任务记录。
    ///
    /// 对应 AList `POST /api/task/{category}/delete?tid={tid}`。
    ///
    /// # Arguments
    ///
    /// * `tid` - 目标任务 ID，可传 `&str` 或任何 `Into<String>` 的值。
    ///
    /// # Returns
    ///
    /// 返回 [`action::Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（如任务不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.task().copy().delete("task_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn delete(&self, tid: impl Into<String>) -> action::Request<'a> {
        action::Request::new(self.client, self.category, "delete", tid)
    }

    /// 取消单个任务。
    ///
    /// 对应 AList `POST /api/task/{category}/cancel?tid={tid}`。
    ///
    /// # Arguments
    ///
    /// * `tid` - 目标任务 ID，可传 `&str` 或任何 `Into<String>` 的值。
    ///
    /// # Returns
    ///
    /// 返回 [`action::Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（如任务不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.task().upload().cancel("task_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn cancel(&self, tid: impl Into<String>) -> action::Request<'a> {
        action::Request::new(self.client, self.category, "cancel", tid)
    }

    /// 清空所有已完成（包括取消、失败、成功）的任务。
    ///
    /// 对应 AList `POST /api/task/{category}/clear_done`。
    ///
    /// # Returns
    ///
    /// 返回 [`empty::Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.task().upload().clear_done().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn clear_done(&self) -> empty::Request<'a> {
        empty::Request::new(self.client, self.category, "clear_done")
    }

    /// 清空所有已成功的任务。
    ///
    /// 对应 AList `POST /api/task/{category}/clear_succeeded`。
    ///
    /// # Returns
    ///
    /// 返回 [`empty::Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.task().upload().clear_succeeded().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn clear_succeeded(&self) -> empty::Request<'a> {
        empty::Request::new(self.client, self.category, "clear_succeeded")
    }

    /// 重试单个任务。
    ///
    /// 对应 AList `POST /api/task/{category}/retry?tid={tid}`。
    ///
    /// # Arguments
    ///
    /// * `tid` - 目标任务 ID，可传 `&str` 或任何 `Into<String>` 的值。
    ///
    /// # Returns
    ///
    /// 返回 [`action::Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（如任务不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.task().copy().retry("task_123").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn retry(&self, tid: impl Into<String>) -> action::Request<'a> {
        action::Request::new(self.client, self.category, "retry", tid)
    }

    /// 重试所有处于失败状态的任务。
    ///
    /// 对应 AList `POST /api/task/{category}/retry_failed`。
    ///
    /// # Returns
    ///
    /// 返回 [`empty::Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.task().upload().retry_failed().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn retry_failed(&self) -> empty::Request<'a> {
        empty::Request::new(self.client, self.category, "retry_failed")
    }

    /// 批量取消指定 ID 列表的任务。
    ///
    /// 对应 AList `POST /api/task/{category}/cancel_some`。
    ///
    /// # Arguments
    ///
    /// * `tids` - 待取消的任务 ID 集合。
    ///
    /// # Returns
    ///
    /// 返回 [`batch::Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`BatchTaskResult`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let failed_map = client.task().upload().cancel_some(["t1", "t2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn cancel_some(
        &self,
        tids: impl IntoIterator<Item = impl Into<String>>,
    ) -> batch::Request<'a> {
        batch::Request::new(self.client, self.category, "cancel_some", tids)
    }

    /// 批量删除指定 ID 列表的任务。
    ///
    /// 对应 AList `POST /api/task/{category}/delete_some`。
    ///
    /// # Arguments
    ///
    /// * `tids` - 待删除的任务 ID 集合。
    ///
    /// # Returns
    ///
    /// 返回 [`batch::Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`BatchTaskResult`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let failed_map = client.task().upload().delete_some(["t1", "t2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn delete_some(
        &self,
        tids: impl IntoIterator<Item = impl Into<String>>,
    ) -> batch::Request<'a> {
        batch::Request::new(self.client, self.category, "delete_some", tids)
    }

    /// 批量重试指定 ID 列表的任务。
    ///
    /// 对应 AList `POST /api/task/{category}/retry_some`。
    ///
    /// # Arguments
    ///
    /// * `tids` - 待重试的任务 ID 集合。
    ///
    /// # Returns
    ///
    /// 返回 [`batch::Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`BatchTaskResult`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let failed_map = client.task().upload().retry_some(["t1", "t2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn retry_some(
        &self,
        tids: impl IntoIterator<Item = impl Into<String>>,
    ) -> batch::Request<'a> {
        batch::Request::new(self.client, self.category, "retry_some", tids)
    }
}

/// 任务管理句柄。
///
/// 作为任务端点方法的顶层命名空间路由句柄。
/// 推荐通过 [`Client::task`](crate::Client::task) 链式调用直接使用。
pub struct Task<'a> {
    client: &'a crate::Client,
}

impl<'a> Task<'a> {
    /// 创建任务句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }

    /// 通过指定的 [`TaskCategory`] 获取通用任务分类子句柄。
    ///
    /// 适用于动态或可配置的任务类别场景。
    ///
    /// # Arguments
    ///
    /// * `category` - 任务类别枚举。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    /// use alist_client::endpoint::task::TaskCategory;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let tasks = client.task().category(TaskCategory::Copy).done().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn category(&self, category: TaskCategory) -> TaskCategoryHandle<'a> {
        TaskCategoryHandle::new(self.client, category)
    }

    /// 获取上传任务 (`upload`) 子句柄。
    ///
    /// 对应 `/api/task/upload` 路由组。
    #[inline]
    #[must_use]
    pub fn upload(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::Upload)
    }

    /// 获取复制任务 (`copy`) 子句柄。
    ///
    /// 对应 `/api/task/copy` 路由组。
    #[inline]
    #[must_use]
    pub fn copy(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::Copy)
    }

    /// 获取离线下载任务 (`offline_download`) 子句柄。
    ///
    /// 对应 `/api/task/offline_download` 路由组。
    #[inline]
    #[must_use]
    pub fn offline_download(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::OfflineDownload)
    }

    /// 获取离线下载中转任务 (`offline_download_transfer`) 子句柄。
    ///
    /// 对应 `/api/task/offline_download_transfer` 路由组。
    #[inline]
    #[must_use]
    pub fn offline_download_transfer(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::OfflineDownloadTransfer)
    }

    /// 获取 S3 转移任务 (`s3_transition`) 子句柄。
    ///
    /// 对应 `/api/task/s3_transition` 路由组。
    #[inline]
    #[must_use]
    pub fn s3_transition(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::S3Transition)
    }

    /// 获取解压任务 (`decompress`) 子句柄。
    ///
    /// 对应 `/api/task/decompress` 路由组。
    #[inline]
    #[must_use]
    pub fn decompress(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::Decompress)
    }

    /// 获取解压上传任务 (`decompress_upload`) 子句柄。
    ///
    /// 对应 `/api/task/decompress_upload` 路由组。
    #[inline]
    #[must_use]
    pub fn decompress_upload(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::DecompressUpload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1. 验证所有 7 种类别生成的 URL 路径是否正确。
    #[test]
    fn test_all_seven_categories_url_paths() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let categories = [
            (TaskCategory::Upload, "upload"),
            (TaskCategory::Copy, "copy"),
            (TaskCategory::OfflineDownload, "offline_download"),
            (
                TaskCategory::OfflineDownloadTransfer,
                "offline_download_transfer",
            ),
            (TaskCategory::S3Transition, "s3_transition"),
            (TaskCategory::Decompress, "decompress"),
            (TaskCategory::DecompressUpload, "decompress_upload"),
        ];

        for (cat, slug) in categories {
            assert_eq!(cat.as_str(), slug);

            // 1) undone: GET /api/task/{category}/undone
            let req = client
                .task()
                .category(cat)
                .undone()
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::GET);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/undone")
            );
            assert!(req.url().query().is_none());
            assert!(req.body().is_none());

            // 2) done: GET /api/task/{category}/done
            let req = client
                .task()
                .category(cat)
                .done()
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::GET);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/done")
            );

            // 3) info: POST /api/task/{category}/info?tid=123
            let req = client
                .task()
                .category(cat)
                .info("test_id")
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/info?tid=test_id")
            );
            assert!(req.body().is_none());

            // 4) cancel: POST /api/task/{category}/cancel?tid=123
            let req = client
                .task()
                .category(cat)
                .cancel("test_id")
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/cancel?tid=test_id")
            );

            // 5) delete: POST /api/task/{category}/delete?tid=123
            let req = client
                .task()
                .category(cat)
                .delete("test_id")
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/delete?tid=test_id")
            );

            // 6) retry: POST /api/task/{category}/retry?tid=123
            let req = client
                .task()
                .category(cat)
                .retry("test_id")
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/retry?tid=test_id")
            );

            // 7) clear_done: POST /api/task/{category}/clear_done
            let req = client
                .task()
                .category(cat)
                .clear_done()
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/clear_done")
            );
            assert!(req.url().query().is_none());
            assert!(req.body().is_none());

            // 8) clear_succeeded: POST /api/task/{category}/clear_succeeded
            let req = client
                .task()
                .category(cat)
                .clear_succeeded()
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/clear_succeeded")
            );

            // 9) retry_failed: POST /api/task/{category}/retry_failed
            let req = client
                .task()
                .category(cat)
                .retry_failed()
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/retry_failed")
            );

            // 10) cancel_some: POST /api/task/{category}/cancel_some
            let req = client
                .task()
                .category(cat)
                .cancel_some(["t1", "t2"])
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/cancel_some")
            );
            assert!(req.body().is_some());

            // 11) delete_some: POST /api/task/{category}/delete_some
            let req = client
                .task()
                .category(cat)
                .delete_some(["t1", "t2"])
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/delete_some")
            );

            // 12) retry_some: POST /api/task/{category}/retry_some
            let req = client
                .task()
                .category(cat)
                .retry_some(["t1", "t2"])
                .build_request()
                .build()
                .unwrap();
            assert_eq!(*req.method(), reqwest::Method::POST);
            assert_eq!(
                req.url().as_str(),
                format!("https://alist.example/api/task/{slug}/retry_some")
            );
        }
    }

    /// 2. 验证各个子类别访问器方法指向正确类别。
    #[test]
    fn test_handle_category_accessors() {
        let client = crate::Client::new("https://alist.example").unwrap();

        // 7 个子类别访问器分别指向正确类别
        assert_eq!(
            client
                .task()
                .upload()
                .done()
                .build_request()
                .build()
                .unwrap()
                .url()
                .as_str(),
            "https://alist.example/api/task/upload/done"
        );
        assert_eq!(
            client
                .task()
                .copy()
                .done()
                .build_request()
                .build()
                .unwrap()
                .url()
                .as_str(),
            "https://alist.example/api/task/copy/done"
        );
        assert_eq!(
            client
                .task()
                .offline_download()
                .done()
                .build_request()
                .build()
                .unwrap()
                .url()
                .as_str(),
            "https://alist.example/api/task/offline_download/done"
        );
        assert_eq!(
            client
                .task()
                .offline_download_transfer()
                .done()
                .build_request()
                .build()
                .unwrap()
                .url()
                .as_str(),
            "https://alist.example/api/task/offline_download_transfer/done"
        );
        assert_eq!(
            client
                .task()
                .s3_transition()
                .done()
                .build_request()
                .build()
                .unwrap()
                .url()
                .as_str(),
            "https://alist.example/api/task/s3_transition/done"
        );
        assert_eq!(
            client
                .task()
                .decompress()
                .done()
                .build_request()
                .build()
                .unwrap()
                .url()
                .as_str(),
            "https://alist.example/api/task/decompress/done"
        );
        assert_eq!(
            client
                .task()
                .decompress_upload()
                .done()
                .build_request()
                .build()
                .unwrap()
                .url()
                .as_str(),
            "https://alist.example/api/task/decompress_upload/done"
        );
    }

    /// 3. Mock 服务器收发与反序列化测试：`list::Request`
    #[tokio::test]
    async fn test_task_list_mock_send_and_await() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": [
                {
                    "id": "task_1",
                    "name": "copy item",
                    "state": 3,
                    "status": "succeeded",
                    "progress": 100,
                    "error": ""
                }
            ]
        }"#;
        let base_url = spawn_mock_server(
            vec![ok_json(body), ok_json(body)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        // 验证 .send().await
        let list1 = client.task().copy().done().send().await.unwrap();
        assert_eq!(list1.len(), 1);
        assert_eq!(list1[0].id, "task_1");

        // 验证直接 .await (IntoFuture)
        let list2 = client.task().copy().done().await.unwrap();
        assert_eq!(list2.len(), 1);
        assert_eq!(list2[0].id, "task_1");

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].starts_with("GET /api/task/copy/done "));
        assert!(recorded[1].starts_with("GET /api/task/copy/done "));
    }

    /// 4. Mock 服务器收发与反序列化测试：`info::Request`
    #[tokio::test]
    async fn test_task_info_mock_send_and_await() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": {
                "id": "task_123",
                "name": "download url",
                "state": 1,
                "status": "running",
                "progress": 42.5,
                "error": ""
            }
        }"#;
        let base_url = spawn_mock_server(
            vec![ok_json(body), ok_json(body)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        // 验证 .send().await
        let info1 = client
            .task()
            .offline_download()
            .info("task_123")
            .send()
            .await
            .unwrap();
        assert_eq!(info1.id, "task_123");
        assert_eq!(info1.progress, 42.5);

        // 验证直接 .await (IntoFuture)
        let info2 = client
            .task()
            .offline_download()
            .info("task_123")
            .await
            .unwrap();
        assert_eq!(info2.id, "task_123");

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].contains("POST /api/task/offline_download/info?tid=task_123 "));
        assert!(recorded[1].contains("POST /api/task/offline_download/info?tid=task_123 "));
    }

    /// 5. Mock 服务器收发与反序列化测试：`action::Request`
    #[tokio::test]
    async fn test_task_action_mock_send_and_await() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":null}"#;
        let base_url = spawn_mock_server(
            vec![ok_json(body), ok_json(body), ok_json(body)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        // 验证 cancel send()
        client.task().upload().cancel("t1").send().await.unwrap();
        // 验证 delete 直接 await
        client.task().copy().delete("t2").await.unwrap();
        // 验证 retry 直接 await
        client.task().s3_transition().retry("t3").await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].contains("POST /api/task/upload/cancel?tid=t1 "));
        assert!(recorded[1].contains("POST /api/task/copy/delete?tid=t2 "));
        assert!(recorded[2].contains("POST /api/task/s3_transition/retry?tid=t3 "));
    }

    /// 6. Mock 服务器收发与反序列化测试：`empty::Request`
    #[tokio::test]
    async fn test_task_empty_post_mock_send_and_await() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":null}"#;
        let base_url = spawn_mock_server(
            vec![ok_json(body), ok_json(body), ok_json(body)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        client.task().upload().clear_done().send().await.unwrap();
        client.task().decompress().clear_succeeded().await.unwrap();
        client
            .task()
            .decompress_upload()
            .retry_failed()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].starts_with("POST /api/task/upload/clear_done "));
        assert!(recorded[1].starts_with("POST /api/task/decompress/clear_succeeded "));
        assert!(recorded[2].starts_with("POST /api/task/decompress_upload/retry_failed "));
    }

    /// 7. Mock 服务器收发与反序列化测试：`batch::Request`
    #[tokio::test]
    async fn test_task_batch_mock_send_and_await() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": {
                "t1": "task not found"
            }
        }"#;
        let base_url = spawn_mock_server(
            vec![ok_json(body), ok_json(body), ok_json(body)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        // 验证 cancel_some .send().await
        let res1 = client
            .task()
            .upload()
            .cancel_some(["t1", "t2"])
            .send()
            .await
            .unwrap();
        assert_eq!(res1.get("t1").map(String::as_str), Some("task not found"));
        assert!(!res1.contains_key("t2"));

        // 验证 delete_some 直接 .await (IntoFuture)
        let res2 = client
            .task()
            .copy()
            .delete_some(vec!["t1".to_string()])
            .await
            .unwrap();
        assert_eq!(res2.len(), 1);

        // 验证 retry_some 直接 .await (IntoFuture)
        let res3 = client
            .task()
            .offline_download()
            .retry_some(["t1"])
            .await
            .unwrap();
        assert_eq!(res3.len(), 1);

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].starts_with("POST /api/task/upload/cancel_some "));
        assert!(recorded[0].contains(r#"["t1","t2"]"#));
        assert!(recorded[1].starts_with("POST /api/task/copy/delete_some "));
        assert!(recorded[1].contains(r#"["t1"]"#));
        assert!(recorded[2].starts_with("POST /api/task/offline_download/retry_some "));
        assert!(recorded[2].contains(r#"["t1"]"#));
    }
}
