use leptos::prelude::*;

use crate::components::ui::skeleton::Skeleton;
use crate::domain::tweet::components::tweet_card::TweetCard;
use crate::domain::tweet::components::tweet_form::TweetForm;
use crate::domain::tweet::tweet_services::{
    DeleteTweet, PostTweet, get_following_tweets, get_tweets,
};

#[component]
pub fn TweetList(#[prop(optional)] following: bool) -> impl IntoView {
    let post_tweet = ServerMultiAction::<PostTweet>::new();
    let delete_tweet = ServerAction::<DeleteTweet>::new();

    let tweets = Resource::new(
        move || (post_tweet.version().get(), delete_tweet.version().get()),
        move |_| async move {
            if following { get_following_tweets().await } else { get_tweets().await }
        },
    );

    let empty_msg = if following {
        "Follow people to see their tweets here."
    } else {
        "No tweets yet. Say something!"
    };

    view! {
        <div class="overflow-y-auto flex-1">
            <TweetForm post_tweet=post_tweet />

            <Transition fallback=|| {
                view! { <TweetListSkeleton count=3 /> }
            }>
                {move || {
                    tweets
                        .and_then(|tweets| {
                            if tweets.is_empty() {
                                view! { <p class="py-8 text-center text-muted-foreground">{empty_msg}</p> }.into_any()
                            } else {
                                view! {
                                    <div class="flex flex-col gap-2 px-4 pt-2 pb-4">
                                        {tweets
                                            .iter()
                                            .map(|tweet| {
                                                view! { <TweetCard tweet=tweet.clone() delete_tweet=delete_tweet /> }
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
    }
}

#[component]
fn TweetListSkeleton(#[prop()] count: usize) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 px-4 pt-2">
            {(0..count)
                .map(|_| {
                    view! {
                        <div class="flex gap-3 p-4 rounded-lg border">
                            <div class="rounded-full size-10 bg-muted shrink-0" />
                            <div class="flex flex-col flex-1 gap-2">
                                <div class="flex gap-2 items-center">
                                    <Skeleton class="w-24 h-4" />
                                    <Skeleton class="w-16 h-4" />
                                </div>
                                <Skeleton class="w-full h-4" />
                                <Skeleton class="w-3/4 h-4" />
                            </div>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
