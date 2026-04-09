use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub created_at: OffsetDateTime,
    pub is_following: bool,
    pub is_current_user: bool,
}

#[cfg(feature = "ssr")]
mod db {
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{User, UserProfile};

    impl User {
        pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
            sqlx::query_as!(User, "SELECT * FROM users ORDER BY username")
                .fetch_all(pool)
                .await
        }

        pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, sqlx::Error> {
            sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
                .fetch_optional(pool)
                .await
        }

        pub async fn create(pool: &PgPool, username: String, display_name: String) -> Result<User, sqlx::Error> {
            sqlx::query_as!(
                User,
                "INSERT INTO users (username, display_name) VALUES ($1, $2) RETURNING *",
                username,
                display_name
            )
            .fetch_one(pool)
            .await
        }
    }

    impl User {
        pub async fn get_others_with_follow_state(
            pool: &PgPool,
            current_user_id: Uuid,
        ) -> Result<Vec<UserProfile>, sqlx::Error> {
            sqlx::query_as!(
                UserProfile,
                r#"
                SELECT
                    u.id,
                    u.username,
                    u.display_name,
                    u.bio,
                    u.avatar_url,
                    u.banner_url,
                    u.created_at,
                    EXISTS(
                        SELECT 1 FROM follows
                        WHERE follower_id = $1 AND following_id = u.id
                    ) AS "is_following!: bool",
                    (u.id = $1) AS "is_current_user!: bool"
                FROM users u
                WHERE u.id != $1
                ORDER BY u.username
                "#,
                current_user_id,
            )
            .fetch_all(pool)
            .await
        }
    }

    impl UserProfile {
        pub async fn get_by_username(
            pool: &PgPool,
            username: &str,
            current_user_id: Uuid,
        ) -> Result<Option<UserProfile>, sqlx::Error> {
            sqlx::query_as!(
                UserProfile,
                r#"
                SELECT
                    u.id,
                    u.username,
                    u.display_name,
                    u.bio,
                    u.avatar_url,
                    u.banner_url,
                    u.created_at,
                    EXISTS(
                        SELECT 1 FROM follows
                        WHERE follower_id = $2 AND following_id = u.id
                    ) AS "is_following!: bool",
                    (u.id = $2) AS "is_current_user!: bool"
                FROM users u
                WHERE u.username = $1
                "#,
                username,
                current_user_id,
            )
            .fetch_optional(pool)
            .await
        }
    }
}
