use uuid::Uuid;

#[cfg(feature = "ssr")]
mod db {
    use sqlx::PgPool;
    use uuid::Uuid;

    pub struct Follow;

    impl Follow {
        pub async fn toggle(pool: &PgPool, follower_id: Uuid, following_id: Uuid) -> Result<bool, sqlx::Error> {
            sqlx::query_scalar!(
                r#"
                WITH deleted AS (
                    DELETE FROM follows
                    WHERE follower_id = $1 AND following_id = $2
                    RETURNING 1
                ),
                inserted AS (
                    INSERT INTO follows (follower_id, following_id)
                    SELECT $1, $2
                    WHERE NOT EXISTS (SELECT 1 FROM deleted)
                    RETURNING 1
                )
                SELECT EXISTS (SELECT 1 FROM inserted) AS "is_following!"
                "#,
                follower_id,
                following_id,
            )
            .fetch_one(pool)
            .await
        }
    }
}

#[cfg(feature = "ssr")]
pub use db::Follow;
