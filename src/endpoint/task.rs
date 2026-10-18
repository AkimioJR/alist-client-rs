//! 任务管理端点句柄。
//!
//! 覆盖 AList 后台任务管理端点：同一套任务路由在服务端同时挂载于
//! 用户级 `/api/task` 与管理员级 `/api/admin/task` 两个前缀下，
//! 包含 7 种子类别（`upload`, `copy`, `offline_download`, `offline_download_transfer`,
//! `s3_transition`, `decompress`, `decompress_upload`），
//! 每种子类别均支持 12 种控制与查询操作：
//! 任务详情、已完成/未完成列表、删除、取消、重试、清空已完成、清空已成功、重试全部失败任务，
//! 以及批量取消、批量删除与批量重试。
//!
//! ## 路由前缀与访问前提
//!
//! 两个前缀由同一组服务端任务路由处理器（AList 服务端 `handles.SetupTaskRoute`）
//! 提供服务，请求方法、路径动作与响应模型完全同形，差异仅在访问控制与可见任务范围：
//!
//! - 用户级 `/api/task`：路由中间件为认证与非访客校验（AList 服务端
//!   `middlewares.Auth` 与 `middlewares.AuthNotGuest`），登录非访客用户可用，
//!   且非管理员仅能查询与操作本人创建的任务（管理员调用时不受此限制）；
//! - 管理员级 `/api/admin/task`：路由中间件为认证与管理员校验（AList 服务端
//!   `middlewares.AuthAdmin`），仅管理员凭据可用（非管理员收到 403 错误），
//!   可查看与操作全部用户的任务。
//!
//! 通过 [`Client::task`](crate::Client::task) 获取用户级句柄（默认）；
//! 管理员级句柄则由管理端句柄通过 `client.admin().task()` 获取（需启用 `admin-task` feature）；
//! 前缀选择以 [`TaskScope`] 表达，可用 [`Task::scope`] 查询。
//!
//! ## 使用模式
//!
//! 1. 语义化分类链式调用（用户级前缀）：
//!    ```no_run
//!    # use alist_client::Client;
//!    # #[cfg(feature = "task")]
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
//! 2. 通用动态分类入口（用户级前缀）：
//!    ```no_run
//!    # use alist_client::Client;
//!    # use alist_client::endpoint::task::TaskCategory;
//!    # #[cfg(feature = "task")]
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

/// 任务路由前缀作用域。
///
/// AList 服务端将同一套任务路由（`handles.SetupTaskRoute`）同时挂载在
/// 两个前缀下，本枚举决定任务请求构建器生成的路径前缀：
///
/// - [`TaskScope::User`]：用户级 `/api/task` 前缀。路由中间件为认证与非访客校验
///   （AList 服务端 `middlewares.Auth` 与 `middlewares.AuthNotGuest`），登录非访客用户
///   即可访问；非管理员仅能查询与操作本人创建的任务，管理员调用时不受此限制。
/// - [`TaskScope::Admin`]：管理员级 `/api/admin/task` 前缀。路由中间件为认证与管理员
///   校验（AList 服务端 `middlewares.AuthAdmin`），必须携带管理员凭据（非管理员收到
///   403 错误），可查询与操作全部用户的任务。
///
/// 两前缀由同一组服务端处理器提供服务，请求方法、路径动作与响应模型完全同形。
/// 管理员前缀自 AList `v3.0.0` 起即存在；从 AList `v3.39.1` 起，任务路由新增
/// 用户级 `/api/task` 前缀并保留管理员前缀作为兼容别名（老版本仅提供管理员前缀）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskScope {
    /// 用户级前缀 `/api/task`（登录非访客用户可用）。
    User,
    /// 管理员级前缀 `/api/admin/task`（仅管理员可用）。
    Admin,
}

impl TaskScope {
    /// 获取作用域对应的 API 路径前缀标识字符串。
    ///
    /// 相对 `/api` 而言：[`TaskScope::User`] 为 `"task"`，
    /// [`TaskScope::Admin`] 为 `"admin/task"`。
    #[inline]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "task",
            Self::Admin => "admin/task",
        }
    }
}

/// 任务分类子句柄。
///
/// 绑定了具体 [`TaskCategory`] 与路由前缀作用域（[`TaskScope`]）的任务操作句柄，
/// 提供该类别下的全部 12 个 API 端点访问器；请求路径前缀随作用域为
/// `/api/task`（用户级）或 `/api/admin/task`（管理员级）。
pub struct TaskCategoryHandle<'a> {
    client: &'a crate::Client,
    scope: TaskScope,
    category: TaskCategory,
}

impl<'a> TaskCategoryHandle<'a> {
    /// 创建任务分类子句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client, scope: TaskScope, category: TaskCategory) -> Self {
        Self {
            client,
            scope,
            category,
        }
    }

    /// 查询单个任务的详细信息。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/info?tid={tid}`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        info::Request::new(self.client, self.scope, self.category, tid)
    }

    /// 获取该类别下的已完成任务列表。
    ///
    /// 对应 AList 任务接口 `GET /api/task/{category}/done`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        list::Request::new(self.client, self.scope, self.category, "done")
    }

    /// 获取该类别下的未完成任务列表。
    ///
    /// 对应 AList 任务接口 `GET /api/task/{category}/undone`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        list::Request::new(self.client, self.scope, self.category, "undone")
    }

    /// 删除单个任务记录。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/delete?tid={tid}`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        action::Request::new(self.client, self.scope, self.category, "delete", tid)
    }

    /// 取消单个任务。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/cancel?tid={tid}`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        action::Request::new(self.client, self.scope, self.category, "cancel", tid)
    }

    /// 清空所有已完成（包括取消、失败、成功）的任务。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/clear_done`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        empty::Request::new(self.client, self.scope, self.category, "clear_done")
    }

    /// 清空所有已成功的任务。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/clear_succeeded`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        empty::Request::new(self.client, self.scope, self.category, "clear_succeeded")
    }

    /// 重试单个任务。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/retry?tid={tid}`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        action::Request::new(self.client, self.scope, self.category, "retry", tid)
    }

    /// 重试所有处于失败状态的任务。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/retry_failed`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        empty::Request::new(self.client, self.scope, self.category, "retry_failed")
    }

    /// 批量取消指定 ID 列表的任务。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/cancel_some`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        batch::Request::new(self.client, self.scope, self.category, "cancel_some", tids)
    }

    /// 批量删除指定 ID 列表的任务。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/delete_some`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        batch::Request::new(self.client, self.scope, self.category, "delete_some", tids)
    }

    /// 批量重试指定 ID 列表的任务。
    ///
    /// 对应 AList 任务接口 `POST /api/task/{category}/retry_some`；
    /// 管理员句柄下路径前缀为 `/api/admin/task`（见 [`TaskScope`]）。
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
    /// # #[cfg(feature = "task")]
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
        batch::Request::new(self.client, self.scope, self.category, "retry_some", tids)
    }
}

/// 任务管理句柄。
///
/// 作为任务端点方法的顶层命名空间路由句柄。
/// 通过 [`Client::task`](crate::Client::task)（用户级 `/api/task` 前缀）或
/// 管理端的 `client.admin().task()`（管理员级 `/api/admin/task` 前缀，需启用 `admin-task` feature）
/// 获取，推荐即建即用；前缀作用域可用 [`Task::scope`] 查询。
pub struct Task<'a> {
    client: &'a crate::Client,
    scope: TaskScope,
}

impl<'a> Task<'a> {
    /// 创建任务句柄。
    ///
    /// 默认使用用户级 `/api/task` 前缀（[`TaskScope::User`]），
    /// 与 [`Client::task`](crate::Client::task) 行为一致。
    #[cfg(feature = "task")]
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            scope: TaskScope::User,
        }
    }

    /// 创建指定路由前缀作用域的任务句柄。
    #[cfg(any(feature = "admin-task", test))]
    #[inline]
    pub(crate) fn with_scope(client: &'a crate::Client, scope: TaskScope) -> Self {
        Self { client, scope }
    }

    /// 返回当前句柄的路由前缀作用域。
    #[inline]
    #[must_use]
    pub const fn scope(&self) -> TaskScope {
        self.scope
    }

    /// 通过指定的 [`TaskCategory`] 获取通用任务分类子句柄。
    ///
    /// 适用于动态或可配置的任务类别场景；请求路径前缀随当前句柄作用域。
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
    /// # #[cfg(feature = "task")]
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
        TaskCategoryHandle::new(self.client, self.scope, category)
    }

    /// 获取上传任务 (`upload`) 子句柄。
    ///
    /// 对应 `/api/task/upload` 路由组；管理员句柄下为 `/api/admin/task/upload`。
    #[inline]
    #[must_use]
    pub fn upload(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::Upload)
    }

    /// 获取复制任务 (`copy`) 子句柄。
    ///
    /// 对应 `/api/task/copy` 路由组；管理员句柄下为 `/api/admin/task/copy`。
    #[inline]
    #[must_use]
    pub fn copy(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::Copy)
    }

    /// 获取离线下载任务 (`offline_download`) 子句柄。
    ///
    /// 对应 `/api/task/offline_download` 路由组；管理员句柄下为
    /// `/api/admin/task/offline_download`。
    #[inline]
    #[must_use]
    pub fn offline_download(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::OfflineDownload)
    }

    /// 获取离线下载中转任务 (`offline_download_transfer`) 子句柄。
    ///
    /// 对应 `/api/task/offline_download_transfer` 路由组；管理员句柄下为
    /// `/api/admin/task/offline_download_transfer`。
    #[inline]
    #[must_use]
    pub fn offline_download_transfer(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::OfflineDownloadTransfer)
    }

    /// 获取 S3 转移任务 (`s3_transition`) 子句柄。
    ///
    /// 对应 `/api/task/s3_transition` 路由组；管理员句柄下为
    /// `/api/admin/task/s3_transition`。
    #[inline]
    #[must_use]
    pub fn s3_transition(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::S3Transition)
    }

    /// 获取解压任务 (`decompress`) 子句柄。
    ///
    /// 对应 `/api/task/decompress` 路由组；管理员句柄下为 `/api/admin/task/decompress`。
    #[inline]
    #[must_use]
    pub fn decompress(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::Decompress)
    }

    /// 获取解压上传任务 (`decompress_upload`) 子句柄。
    ///
    /// 对应 `/api/task/decompress_upload` 路由组；管理员句柄下为
    /// `/api/admin/task/decompress_upload`。
    #[inline]
    #[must_use]
    pub fn decompress_upload(&self) -> TaskCategoryHandle<'a> {
        self.category(TaskCategory::DecompressUpload)
    }
}

#[cfg(all(test, feature = "task"))]
mod tests {
    use super::*;

    /// 断言指定类别句柄下全部 12 个动作的请求方法与 URL 路径
    /// （`base` 为类别路径公共前缀，如 `https://alist.example/api/task/upload`）。
    fn assert_twelve_actions(handle: &TaskCategoryHandle<'_>, base: &str) {
        // 1) undone: GET {base}/undone
        let req = handle.undone().build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::GET);
        assert_eq!(req.url().as_str(), format!("{base}/undone"));
        assert!(req.url().query().is_none());
        assert!(req.body().is_none());

        // 2) done: GET {base}/done
        let req = handle.done().build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::GET);
        assert_eq!(req.url().as_str(), format!("{base}/done"));

        // 3) info: POST {base}/info?tid=…
        let req = handle.info("test_id").build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/info?tid=test_id"));
        assert!(req.body().is_none());

        // 4) cancel: POST {base}/cancel?tid=…
        let req = handle.cancel("test_id").build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/cancel?tid=test_id"));

        // 5) delete: POST {base}/delete?tid=…
        let req = handle.delete("test_id").build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/delete?tid=test_id"));

        // 6) retry: POST {base}/retry?tid=…
        let req = handle.retry("test_id").build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/retry?tid=test_id"));

        // 7) clear_done: POST {base}/clear_done
        let req = handle.clear_done().build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/clear_done"));
        assert!(req.url().query().is_none());
        assert!(req.body().is_none());

        // 8) clear_succeeded: POST {base}/clear_succeeded
        let req = handle.clear_succeeded().build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/clear_succeeded"));

        // 9) retry_failed: POST {base}/retry_failed
        let req = handle.retry_failed().build_request().build().unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/retry_failed"));

        // 10) cancel_some: POST {base}/cancel_some
        let req = handle
            .cancel_some(["t1", "t2"])
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/cancel_some"));
        assert!(req.body().is_some());

        // 11) delete_some: POST {base}/delete_some
        let req = handle
            .delete_some(["t1", "t2"])
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/delete_some"));

        // 12) retry_some: POST {base}/retry_some
        let req = handle
            .retry_some(["t1", "t2"])
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*req.method(), reqwest::Method::POST);
        assert_eq!(req.url().as_str(), format!("{base}/retry_some"));
    }

    /// 用户级与管理员级句柄共用的 7 种类别表。
    fn all_categories() -> [(TaskCategory, &'static str); 7] {
        [
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
        ]
    }

    /// 1. 验证用户级前缀（`client.task`）下所有 7 种类别生成的 URL 路径是否正确。
    #[test]
    fn test_all_seven_categories_url_paths() {
        let client = crate::Client::new("https://alist.example").unwrap();

        for (cat, slug) in all_categories() {
            assert_eq!(cat.as_str(), slug);
            let handle = client.task().category(cat);
            assert_twelve_actions(&handle, &format!("https://alist.example/api/task/{slug}"));
        }
    }

    /// 1b. 验证管理员级前缀（`TaskScope::Admin`）下所有 7 种类别生成的 URL 路径是否正确。
    #[test]
    fn test_admin_scope_all_categories_url_paths() {
        let client = crate::Client::new("https://alist.example").unwrap();

        for (cat, slug) in all_categories() {
            assert_eq!(cat.as_str(), slug);
            let handle = Task::with_scope(&client, TaskScope::Admin).category(cat);
            assert_twelve_actions(
                &handle,
                &format!("https://alist.example/api/admin/task/{slug}"),
            );
        }
    }

    /// 1c. 验证作用域访问器与两种前缀下的语义化子类别访问器。
    #[test]
    fn test_task_scope_accessor() {
        let client = crate::Client::new("https://alist.example").unwrap();
        assert_eq!(client.task().scope(), TaskScope::User);
        assert_eq!(
            Task::with_scope(&client, TaskScope::Admin).scope(),
            TaskScope::Admin
        );
        assert_eq!(TaskScope::User.as_str(), "task");
        assert_eq!(TaskScope::Admin.as_str(), "admin/task");

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
            Task::with_scope(&client, TaskScope::Admin)
                .decompress_upload()
                .undone()
                .build_request()
                .build()
                .unwrap()
                .url()
                .as_str(),
            "https://alist.example/api/admin/task/decompress_upload/undone"
        );
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
        let mut server = mockito::Server::new_async().await;
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": [
                {
                    "id": "t1",
                    "name": "upload task",
                    "state": 3,
                    "status": "succeeded",
                    "progress": 100,
                    "error": ""
                }
            ]
        }"#;
        let mock_undone = server
            .mock("GET", "/api/task/upload/undone")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let mock_done = server
            .mock("GET", "/api/task/copy/done")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let list1 = client.task().upload().undone().send().await.unwrap();
        assert_eq!(list1.len(), 1);
        let list2 = client.task().copy().done().await.unwrap();
        assert_eq!(list2.len(), 1);

        mock_undone.assert_async().await;
        mock_done.assert_async().await;
    }

    /// 4. Mock 服务器收发与反序列化测试：`info::Request`
    #[tokio::test]
    async fn test_task_info_mock_send_and_await() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": {
                "id": "t_info",
                "name": "download url",
                "state": 1,
                "status": "running",
                "progress": 42.5,
                "error": ""
            }
        }"#;
        let mock_upload = server
            .mock("POST", "/api/task/upload/info?tid=t1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let mock_copy = server
            .mock("POST", "/api/task/copy/info?tid=t2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let info1 = client.task().upload().info("t1").send().await.unwrap();
        assert_eq!(info1.id, "t_info");
        let info2 = client.task().copy().info("t2".to_string()).await.unwrap();
        assert_eq!(info2.id, "t_info");

        mock_upload.assert_async().await;
        mock_copy.assert_async().await;
    }

    /// 5. Mock 服务器收发与反序列化测试：`action::Request`
    #[tokio::test]
    async fn test_task_action_mock_send_and_await() {
        let mut server = mockito::Server::new_async().await;
        let mock_cancel = server
            .mock("POST", "/api/task/upload/cancel?tid=t1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let mock_delete = server
            .mock("POST", "/api/task/copy/delete?tid=t2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let mock_retry = server
            .mock("POST", "/api/task/s3_transition/retry?tid=t3")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        client.task().upload().cancel("t1").send().await.unwrap();
        client.task().copy().delete("t2").await.unwrap();
        client.task().s3_transition().retry("t3").await.unwrap();

        mock_cancel.assert_async().await;
        mock_delete.assert_async().await;
        mock_retry.assert_async().await;
    }

    /// 6. Mock 服务器收发与反序列化测试：`empty::Request`
    #[tokio::test]
    async fn test_task_empty_post_mock_send_and_await() {
        let mut server = mockito::Server::new_async().await;
        let mock_clear_done = server
            .mock("POST", "/api/task/upload/clear_done")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let mock_clear_succeeded = server
            .mock("POST", "/api/task/decompress/clear_succeeded")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let mock_retry_failed = server
            .mock("POST", "/api/task/s3_transition/retry_failed")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        client.task().upload().clear_done().send().await.unwrap();
        client.task().decompress().clear_succeeded().await.unwrap();
        client.task().s3_transition().retry_failed().await.unwrap();

        mock_clear_done.assert_async().await;
        mock_clear_succeeded.assert_async().await;
        mock_retry_failed.assert_async().await;
    }

    /// 7. Mock 服务器收发与反序列化测试：`batch::Request`
    #[tokio::test]
    async fn test_task_batch_mock_send_and_await() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": {
                "t1": "task not found"
            }
        }"#;
        let mock_cancel = server
            .mock("POST", "/api/task/upload/cancel_some")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::Json(serde_json::json!(["t1", "t2"])))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let mock_delete = server
            .mock("POST", "/api/task/copy/delete_some")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::Json(serde_json::json!(["t1"])))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let mock_retry = server
            .mock("POST", "/api/task/offline_download/retry_some")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::Json(serde_json::json!(["t1"])))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let res1 = client
            .task()
            .upload()
            .cancel_some(["t1", "t2"])
            .send()
            .await
            .unwrap();
        assert_eq!(res1.get("t1").map(String::as_str), Some("task not found"));
        assert!(!res1.contains_key("t2"));

        let res2 = client
            .task()
            .copy()
            .delete_some(vec!["t1".to_string()])
            .await
            .unwrap();
        assert_eq!(res2.len(), 1);

        let res3 = client
            .task()
            .offline_download()
            .retry_some(["t1"])
            .await
            .unwrap();
        assert_eq!(res3.len(), 1);

        mock_cancel.assert_async().await;
        mock_delete.assert_async().await;
        mock_retry.assert_async().await;
    }

    /// 8. Mock 服务器收发与反序列化测试：管理员级前缀（`TaskScope::Admin`）
    ///    覆盖全部 5 种请求形状。
    #[tokio::test]
    async fn test_task_admin_prefix_mock_send_and_await() {
        let mut server = mockito::Server::new_async().await;
        let list_body = r#"{
            "code": 200,
            "message": "success",
            "data": [
                {
                    "id": "admin_task_1",
                    "name": "upload item",
                    "state": 1,
                    "status": "running",
                    "progress": 10.0,
                    "error": ""
                }
            ]
        }"#;
        let info_body = r#"{
            "code": 200,
            "message": "success",
            "data": {
                "id": "t_info",
                "name": "download url",
                "state": 1,
                "status": "running",
                "progress": 42.5,
                "error": ""
            }
        }"#;

        let mock_list = server
            .mock("GET", "/api/admin/task/upload/undone")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(list_body)
            .create_async()
            .await;
        let mock_info = server
            .mock("POST", "/api/admin/task/copy/info?tid=t_info")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(info_body)
            .create_async()
            .await;
        let mock_cancel = server
            .mock("POST", "/api/admin/task/offline_download/cancel?tid=t2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let mock_empty = server
            .mock("POST", "/api/admin/task/decompress/clear_done")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let mock_batch = server
            .mock("POST", "/api/admin/task/s3_transition/retry_some")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::Json(serde_json::json!(["t3"])))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":{"t3":"task not found"}}"#)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();
        let admin_task = Task::with_scope(&client, TaskScope::Admin);

        let list = admin_task.upload().undone().send().await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "admin_task_1");

        let info = admin_task.copy().info("t_info").await.unwrap();
        assert_eq!(info.id, "t_info");

        admin_task.offline_download().cancel("t2").await.unwrap();

        admin_task.decompress().clear_done().await.unwrap();

        let batch = admin_task.s3_transition().retry_some(["t3"]).await.unwrap();
        assert_eq!(batch.get("t3").map(String::as_str), Some("task not found"));

        mock_list.assert_async().await;
        mock_info.assert_async().await;
        mock_cancel.assert_async().await;
        mock_empty.assert_async().await;
        mock_batch.assert_async().await;
    }
}
