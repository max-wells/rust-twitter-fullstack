use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
use leptos_router::hooks::use_location;

use crate::domain::home::routes::HomeRoutes;

#[component]
pub fn HomeLayout() -> impl IntoView {
    let location = use_location();

    let is_following = Memo::new(move |_| location.pathname.get() == HomeRoutes::following_url());

    view! {
        <div class="flex relative flex-col mx-auto w-full h-full max-w-[600px]">
            <header class="sticky top-0 z-10 border-b bg-background shrink-0">
                <div class="flex">
                    <A
                        href=HomeRoutes::base_url()
                        attr:class=move || {
                            if !is_following() {
                                "flex-1 py-4 text-sm text-center font-bold relative \
                                 after:absolute after:bottom-0 after:left-1/4 after:w-1/2 \
                                 after:h-0.5 after:bg-primary after:rounded-full"
                            } else {
                                "flex-1 py-4 text-sm text-center text-muted-foreground \
                                 hover:bg-muted/50 transition-colors"
                            }
                        }
                    >
                        "For you"
                    </A>
                    <A
                        href=HomeRoutes::following_url()
                        attr:class=move || {
                            if is_following() {
                                "flex-1 py-4 text-sm text-center font-bold relative \
                                 after:absolute after:bottom-0 after:left-1/4 after:w-1/2 \
                                 after:h-0.5 after:bg-primary after:rounded-full"
                            } else {
                                "flex-1 py-4 text-sm text-center text-muted-foreground \
                                 hover:bg-muted/50 transition-colors"
                            }
                        }
                    >
                        "Following"
                    </A>
                </div>
            </header>

            <Outlet />
        </div>
    }
}
