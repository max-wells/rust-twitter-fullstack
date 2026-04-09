use icons::{Image, MapPin, Smile};
use leptos::prelude::*;

use crate::components::ui::avatar::Avatar;
use crate::components::ui::button::Button;
use crate::components::ui::textarea::Textarea;
use crate::domain::tweet::tweet_services::PostTweet;

const ALICE_AVATAR: &str = "https://api.dicebear.com/9.x/pixel-art/svg?seed=alice";

#[component]
pub fn TweetForm(post_tweet: ServerMultiAction<PostTweet>) -> impl IntoView {
    let content = RwSignal::new(String::new());
    let char_count = move || content.get().chars().count();
    let is_over_limit = move || char_count() > 260;

    // Reset textarea after successful submission
    Effect::new(move |_| {
        post_tweet.version().get();
        content.set(String::new());
    });

    view! {
        <MultiActionForm action=post_tweet>
            <div class="flex gap-3 py-3 px-4 border-b">
                <Avatar src=Some(ALICE_AVATAR.to_string()) alt="Your avatar" class="mt-1 shrink-0" />

                <div class="flex flex-col flex-1 gap-3 min-w-0">
                    // Twitter-style borderless textarea
                    <Textarea
                        name="content"
                        placeholder="What's happening?"
                        rows=2u32
                        maxlength=280u32
                        bind_value=content
                        class="py-0 px-0 min-h-0 text-lg rounded-none border-0 shadow-none resize-none focus-visible:border-0 focus-visible:ring-0"
                    />

                    // Bottom row: icon strip + char count + Post button
                    <div class="flex justify-between items-center">
                        <div class="flex gap-3 items-center text-primary">
                            <Image class="cursor-pointer size-5" />
                            <Smile class="cursor-pointer size-5" />
                            <MapPin class="cursor-pointer size-5" />
                        </div>
                        <div class="flex gap-3 items-center">
                            <span class=move || {
                                if is_over_limit() { "text-xs text-red-500" } else { "text-xs text-muted-foreground" }
                            }>{move || format!("{}/280", char_count())}</span>
                            <Button attr:r#type="submit" class="px-4 rounded-full">
                                "Post"
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </MultiActionForm>
    }
}
