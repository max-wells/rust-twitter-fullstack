use leptos::prelude::*;
use time::OffsetDateTime;

use crate::components::layout::back_button::BackButton;
use crate::components::ui::avatar::Avatar;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::skeleton::Skeleton;
use crate::domain::follow::follow_services::ToggleFollow;
use crate::domain::tweet::components::tweet_card::TweetCard;
use crate::domain::tweet::tweet_services::{DeleteTweet, get_tweets_by_username};
use crate::domain::user::user_db::UserProfile;
use crate::domain::user::user_services::get_user_profile_by_username;
use crate::utils::param::UsernameParam;

#[component]
pub fn PageProfile() -> impl IntoView {
    let username = UsernameParam::extract_username();
    let delete_tweet = ServerAction::<DeleteTweet>::new();
    let toggle_follow = ServerAction::<ToggleFollow>::new();

    let profile_resource = Resource::new(
        move || (username.get(), toggle_follow.version().get()),
        move |(u, _)| get_user_profile_by_username(u),
    );

    let tweets_resource = Resource::new(
        move || (username.get(), delete_tweet.version().get()),
        move |(u, _)| get_tweets_by_username(u),
    );

    view! {
        <div class="flex flex-col mx-auto w-full h-full max-w-[600px] bg-background">
            // Header
            <header class="flex items-center px-4 pt-4 pb-3">
                <BackButton />
                <Transition fallback=|| {
                    view! { <Skeleton class="flex-1 mx-4 h-6" /> }
                }>
                    {move || {
                        profile_resource
                            .and_then(|p| {
                                p.as_ref()
                                    .map(|p| {
                                        view! {
                                            <h1 class="flex-1 text-lg font-semibold text-center">
                                                {p.display_name.clone()}
                                            </h1>
                                        }
                                    })
                            })
                    }}
                </Transition>
                <div class="size-8" />
            </header>

            // Profile info
            <Transition fallback=|| {
                view! { <ProfileSkeleton /> }
            }>
                {move || {
                    match profile_resource.get() {
                        Some(Ok(Some(p))) => {
                            view! { <ProfileHeader profile=p toggle_follow=toggle_follow /> }.into_any()
                        }
                        Some(Ok(None)) | Some(Err(_)) => {
                            view! { <p class="py-2 px-4 text-muted-foreground">"User not found."</p> }.into_any()
                        }
                        None => ().into_any(),
                    }
                }}
            </Transition>

            // Tweets
            <div class="overflow-y-auto flex-1 px-4 pb-4">
                <Transition fallback=|| {
                    view! { <ProfileTweetsSkeleton /> }
                }>
                    {move || {
                        tweets_resource
                            .and_then(|tweets| {
                                if tweets.is_empty() {
                                    view! { <p class="py-8 text-center text-muted-foreground">"No tweets yet."</p> }
                                        .into_any()
                                } else {
                                    view! {
                                        <div class="flex flex-col gap-2">
                                            {tweets
                                                .iter()
                                                .map(|tweet| {
                                                    view! {
                                                        <TweetCard tweet=tweet.clone() delete_tweet=delete_tweet />
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Transition>
            </div>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn ProfileHeader(profile: UserProfile, toggle_follow: ServerAction<ToggleFollow>) -> impl IntoView {
    let joined = format_joined(&profile.created_at);
    let is_following = profile.is_following;
    let banner_url = profile.banner_url.clone();
    let avatar_url = profile.avatar_url.clone();
    let initial = profile.display_name.chars().next().unwrap_or('?').to_string();

    let on_follow = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        toggle_follow.dispatch(ToggleFollow { following_id: profile.id });
    };

    view! {
        <div class="border-b">
            // Banner
            {match banner_url {
                Some(url) => view! { <img src=url alt="banner" class="object-cover w-full h-32" /> }.into_any(),
                None => view! { <div class="w-full h-32 bg-muted" /> }.into_any(),
            }} // Avatar + follow button row
            <div class="flex justify-between items-end px-4 mb-3 -mt-8">
                <Avatar src=avatar_url fallback=initial class="border-4 size-16 border-background" />

                {(!profile.is_current_user)
                    .then(move || {
                        view! {
                            <Button
                                on:click=on_follow
                                variant=if is_following { ButtonVariant::Outline } else { ButtonVariant::Default }
                                class="mb-1"
                            >
                                {if is_following { "Unfollow" } else { "Follow" }}
                            </Button>
                        }
                    })}
            // User info
            </div> <div class="px-4 pb-4">
                <p class="text-xl font-bold">{profile.display_name.clone()}</p>
                <p class="text-sm text-muted-foreground">{"@"}{profile.username.clone()}" · joined "{joined}</p>
                {(!profile.bio.is_empty())
                    .then(|| {
                        view! { <p class="mt-2 text-sm">{profile.bio.clone()}</p> }
                    })}
            </div>
        </div>
    }
}

fn format_joined(dt: &OffsetDateTime) -> String {
    format!("{} {}", dt.month(), dt.year())
}

#[component]
fn ProfileSkeleton() -> impl IntoView {
    view! {
        <div class="border-b">
            <div class="w-full h-32 bg-muted" />
            <div class="px-4 pb-4 -mt-8">
                <div class="mb-3 rounded-full border-4 size-16 bg-muted border-background" />
                <Skeleton class="mb-1 w-32 h-6" />
                <Skeleton class="w-48 h-4" />
            </div>
        </div>
    }
}

#[component]
pub fn ProfileTweetsSkeleton() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 pt-2">
            <Skeleton class="w-full h-20" />
            <Skeleton class="w-full h-20" />
        </div>
    }
}
