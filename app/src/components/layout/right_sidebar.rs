use icons::Search;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::layout::theme_toggle::ThemeToggle;
use crate::components::ui::avatar::Avatar;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::domain::follow::follow_services::ToggleFollow;
use crate::domain::user::user_db::UserProfile;
use crate::domain::user::user_services::get_users_to_follow;

#[component]
pub fn RightSidebar() -> impl IntoView {
    let toggle_follow = ServerAction::<ToggleFollow>::new();

    let users_resource =
        Resource::new(move || toggle_follow.version().get(), |_| get_users_to_follow());

    view! {
        <aside class="hidden overflow-y-auto flex-col gap-6 py-3 px-4 h-full border-l lg:flex w-[350px] shrink-0">
            // Search + theme toggle
            <div class="flex gap-4 items-center">
                <div class="relative flex-1">
                    <Search class="absolute top-2.5 left-3 pointer-events-none size-4 text-muted-foreground" />
                    <input
                        type="text"
                        placeholder="Search"
                        class="py-2 pr-4 pl-9 w-full text-sm rounded-full border-0 outline-none bg-muted"
                    />
                </div>
                <ThemeToggle />
            </div>

            // Trending
            <div class="flex flex-col gap-3 p-4 rounded-2xl border">
                <h2 class="text-lg font-bold">"What's happening"</h2>
                <TrendingItem tag="#leptos" count="1.2K Tweets" />
                <TrendingItem tag="#rust" count="45.3K Tweets" />
                <TrendingItem tag="#tauri" count="8.9K Tweets" />
            </div>

            // Who to follow
            <div class="flex flex-col gap-3 p-4 rounded-2xl border">
                <h2 class="text-lg font-bold">"Who to follow"</h2>
                <Transition fallback=|| {
                    view! { <p class="text-sm text-muted-foreground">"Loading..."</p> }
                }>
                    {move || {
                        users_resource
                            .and_then(|users| {
                                users
                                    .iter()
                                    .map(|u| {
                                        view! { <WhoToFollowItem user=u.clone() toggle_follow=toggle_follow /> }
                                    })
                                    .collect_view()
                            })
                    }}
                </Transition>
            </div>
        </aside>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn TrendingItem(tag: &'static str, count: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-0.5 p-2 -mx-2 rounded-lg cursor-pointer hover:bg-muted">
            <p class="text-xs text-muted-foreground">"Trending"</p>
            <p class="text-sm font-semibold">{tag}</p>
            <p class="text-xs text-muted-foreground">{count}</p>
        </div>
    }
}

#[component]
fn WhoToFollowItem(user: UserProfile, toggle_follow: ServerAction<ToggleFollow>) -> impl IntoView {
    let following_id = user.id;
    let is_following = user.is_following;
    let avatar_url = user.avatar_url.clone();
    let display_name = user.display_name.clone();
    let username = user.username.clone();
    let initial = display_name.chars().next().unwrap_or('?').to_string();

    let on_follow = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        toggle_follow.dispatch(ToggleFollow { following_id });
    };

    view! {
        <div class="flex gap-3 items-center">
            <A href=format!("/{username}") attr:class="shrink-0">
                <Avatar src=avatar_url fallback=initial />
            </A>

            <div class="flex-1 min-w-0">
                <p class="text-sm font-semibold truncate">{display_name}</p>
                <p class="text-xs text-muted-foreground truncate">"@"{username}</p>
            </div>

            <Button
                on:click=on_follow
                variant=if is_following { ButtonVariant::Outline } else { ButtonVariant::Default }
                class="shrink-0"
                size=ButtonSize::Sm
            >
                {if is_following { "Unfollow" } else { "Follow" }}
            </Button>
        </div>
    }
}
