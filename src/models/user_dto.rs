use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entities::{oauth_account::OAuthProvider, user};

#[derive(Debug, Deserialize, Clone, ToSchema, Validate)]
pub struct OAuthLoginRequest {
    pub provider: OAuthProvider,
    #[schema(example = "google_123456789")]
    #[validate(length(
        min = 1,
        max = 100,
        message = "Provider user ID must be 1-100 characters"
    ))]
    pub provider_user_id: String,
    #[schema(example = "user@example.com")]
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(min = 5, max = 255, message = "Email must be 5-255 characters"))]
    pub email: String,
    #[schema(example = "홍길동")]
    #[validate(length(min = 2, max = 50, message = "Username must be 2-50 characters"))]
    pub username: String,
}

#[derive(Debug, Serialize, Clone, PartialEq, ToSchema)]
pub struct UserResponse {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "홍길동")]
    pub username: String,
    #[schema(example = "user@example.com")]
    pub email: String,
    #[schema(example = "2024-01-15T10:30:00")]
    pub created_at: NaiveDateTime,
}

impl From<user::Model> for UserResponse {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub user: UserResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    #[schema(example = "Successfully logged out")]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_login_request_valid() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "google_123456789".to_string(),
            email: "user@example.com".to_string(),
            username: "홍길동".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_oauth_login_request_empty_provider_user_id() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "".to_string(),
            email: "user@example.com".to_string(),
            username: "홍길동".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_oauth_login_request_provider_user_id_too_long() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "a".repeat(101),
            email: "user@example.com".to_string(),
            username: "홍길동".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_oauth_login_request_invalid_email_format() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "google_123456789".to_string(),
            email: "invalid-email".to_string(),
            username: "홍길동".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_oauth_login_request_empty_email() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "google_123456789".to_string(),
            email: "".to_string(),
            username: "홍길동".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_oauth_login_request_email_too_long() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "google_123456789".to_string(),
            email: format!("{}@example.com", "a".repeat(245)),
            username: "홍길동".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_oauth_login_request_empty_username() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "google_123456789".to_string(),
            email: "user@example.com".to_string(),
            username: "".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_oauth_login_request_username_too_short() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "google_123456789".to_string(),
            email: "user@example.com".to_string(),
            username: "a".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_oauth_login_request_username_too_long() {
        let request = OAuthLoginRequest {
            provider: OAuthProvider::Google,
            provider_user_id: "google_123456789".to_string(),
            email: "user@example.com".to_string(),
            username: "a".repeat(51),
        };
        assert!(request.validate().is_err());
    }
}
