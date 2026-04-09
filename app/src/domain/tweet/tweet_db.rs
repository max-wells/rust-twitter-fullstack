use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Tweet {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub author_username: String,
    pub author_display_name: String,
    pub author_avatar_url: Option<String>,
    pub is_mine: bool,
}

#[cfg(feature = "ssr")]
mod db {
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::Tweet;

    impl Tweet {
        pub async fn get_all(
            pool: &PgPool,
            current_user_id: Uuid,
        ) -> Result<Vec<Tweet>, sqlx::Error> {
            sqlx::query_as!(
                Tweet,
                r#"
                SELECT t.id, t.author_id, t.content, t.created_at,
                       u.username AS author_username,
                       u.display_name AS author_display_name,
                       u.avatar_url AS author_avatar_url,
                       (t.author_id = $1) AS "is_mine!: bool"
                FROM tweets t
                JOIN users u ON u.id = t.author_id
                ORDER BY t.created_at DESC
                "#,
                current_user_id
            )
            .fetch_all(pool)
            .await
        }

        pub async fn get_by_username(
            pool: &PgPool,
            username: &str,
            current_user_id: Uuid,
        ) -> Result<Vec<Tweet>, sqlx::Error> {
            sqlx::query_as!(
                Tweet,
                r#"
                SELECT t.id, t.author_id, t.content, t.created_at,
                       u.username AS author_username,
                       u.display_name AS author_display_name,
                       u.avatar_url AS author_avatar_url,
                       (t.author_id = $2) AS "is_mine!: bool"
                FROM tweets t
                JOIN users u ON u.id = t.author_id
                WHERE u.username = $1
                ORDER BY t.created_at DESC
                "#,
                username,
                current_user_id
            )
            .fetch_all(pool)
            .await
        }

        pub async fn get_feed(pool: &PgPool, user_id: Uuid) -> Result<Vec<Tweet>, sqlx::Error> {
            sqlx::query_as!(
                Tweet,
                r#"
                SELECT t.id, t.author_id, t.content, t.created_at,
                       u.username AS author_username,
                       u.display_name AS author_display_name,
                       u.avatar_url AS author_avatar_url,
                       (t.author_id = $1) AS "is_mine!: bool"
                FROM tweets t
                JOIN users u ON u.id = t.author_id
                JOIN follows f ON f.following_id = t.author_id AND f.follower_id = $1
                ORDER BY t.created_at DESC
                "#,
                user_id
            )
            .fetch_all(pool)
            .await
        }

        pub async fn create(
            pool: &PgPool,
            author_id: Uuid,
            content: String,
        ) -> Result<Tweet, sqlx::Error> {
            sqlx::query_as!(
                Tweet,
                r#"
                WITH inserted AS (
                    INSERT INTO tweets (author_id, content)
                    VALUES ($1, $2)
                    RETURNING *
                )
                SELECT i.id, i.author_id, i.content, i.created_at,
                       u.username AS author_username,
                       u.display_name AS author_display_name,
                       u.avatar_url AS author_avatar_url,
                       (i.author_id = $1) AS "is_mine!: bool"
                FROM inserted i
                JOIN users u ON u.id = i.author_id
                "#,
                author_id,
                content
            )
            .fetch_one(pool)
            .await
        }

        pub async fn delete(pool: &PgPool, id: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
            sqlx::query_scalar!("DELETE FROM tweets WHERE id = $1 RETURNING id", id)
                .fetch_optional(pool)
                .await
        }
    }
}
