use leptos::prelude::*;

use crate::components::layout::reload_button::ReloadButton;
use crate::components::layout::theme_toggle::ThemeToggle;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        // Mobile only: floating reload + theme buttons (hidden on sm+, LeftSidebar handles desktop)
        <div class="absolute right-8 sm:hidden top-[calc(env(safe-area-inset-top)+0.625rem)] z-100">
            <ReloadButton />
        </div>
        <div class="absolute right-4 sm:hidden top-[calc(env(safe-area-inset-top)+1rem)] z-100">
            <ThemeToggle />
        </div>
    }
}
