//! 客户端认证凭据模型与构造方法。

/// 用于刷新当前 token 的认证凭据。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authentication {
    /// 当前 token 缺失或被拒绝时，使用用户名密码重新登录。
    UsernamePassword {
        /// 用户名。
        username: String,
        /// 密码。
        password: String,
        /// 可选的两步验证码。
        otp_code: Option<String>,
    },
    /// 当前 token 缺失或被拒绝时，重新套用该 token（不会自动登录）。
    Token(String),
}

impl Authentication {
    /// 构造用户名密码认证。
    pub fn username_password(
        username: impl Into<String>,
        password: impl Into<String>,
        otp_code: impl Into<Option<String>>,
    ) -> Self {
        Self::UsernamePassword {
            username: username.into(),
            password: password.into(),
            otp_code: otp_code.into(),
        }
    }

    /// 构造 token 认证。
    pub fn token(token: impl Into<String>) -> Self {
        Self::Token(token.into())
    }
}

impl<T: Into<String>> From<T> for Authentication {
    fn from(token: T) -> Self {
        Self::Token(token.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authentication_constructors_produce_expected_variants() {
        let auth = Authentication::username_password("admin", "password", None);
        assert_eq!(
            auth,
            Authentication::UsernamePassword {
                username: "admin".to_string(),
                password: "password".to_string(),
                otp_code: None,
            }
        );

        let auth_with_otp =
            Authentication::username_password("admin", "password", Some("123456".to_string()));
        assert_eq!(
            auth_with_otp,
            Authentication::UsernamePassword {
                username: "admin".to_string(),
                password: "password".to_string(),
                otp_code: Some("123456".to_string()),
            }
        );

        let token_auth = Authentication::token("token-xyz");
        assert_eq!(token_auth, Authentication::Token("token-xyz".to_string()));

        let from_auth: Authentication = "token-abc".into();
        assert_eq!(from_auth, Authentication::Token("token-abc".to_string()));
    }
}
