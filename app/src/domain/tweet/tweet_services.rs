use leptos::prelude::*;
use leptos::server_fn::codec::{GetUrl, Json, PostUrl};
use uuid::Uuid;

use crate::domain::tweet::tweet_db::Tweet;

#[server(GetTweets, "/api", input = GetUrl, output = Json)]
pub async fn get_tweets() -> Result<Vec<Tweet>, ServerFnError> {
    use crate::common::app_state::use_app_state;
    let app_state = use_app_state()?;
    Tweet::get_all(&app_state.pool, app_state.current_user_id).await.map_err(ServerFnError::new)
}

#[server(GetTweetsByUsername, "/api", input = GetUrl, output = Json)]
pub async fn get_tweets_by_username(username: String) -> Result<Vec<Tweet>, ServerFnError> {
    use crate::common::app_state::use_app_state;
    let app_state = use_app_state()?;
    Tweet::get_by_username(&app_state.pool, &username, app_state.current_user_id).await.map_err(ServerFnError::new)
}

#[server(GetFollowingTweets, "/api", input = GetUrl, output = Json)]
pub async fn get_following_tweets() -> Result<Vec<Tweet>, ServerFnError> {
    use crate::common::app_state::use_app_state;
    let app_state = use_app_state()?;
    Tweet::get_feed(&app_state.pool, app_state.current_user_id).await.map_err(ServerFnError::new)
}

#[server(PostTweet, "/api", input = PostUrl, output = Json)]
pub async fn post_tweet(content: String) -> Result<Tweet, ServerFnError> {
    use crate::common::app_state::use_app_state;
    let app_state = use_app_state()?;
    Tweet::create(&app_state.pool, app_state.current_user_id, content)
        .await
        .map_err(ServerFnError::new)
}

#[server(DeleteTweet, "/api", input = GetUrl, output = Json)]
pub async fn delete_tweet(id: Uuid) -> Result<Option<Uuid>, ServerFnError> {
    use crate::common::app_state::use_app_state;
    let app_state = use_app_state()?;
    Tweet::delete(&app_state.pool, id).await.map_err(ServerFnError::new)
}
