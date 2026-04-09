use leptos::prelude::*;
use leptos_meta::{Html, Title, provide_meta_context};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use crate::components::hooks::use_theme_mode::ThemeMode;
use crate::components::layout::app_bottom_nav::AppBottomNav;
use crate::components::layout::app_wrapper::AppWrapper;
use crate::components::layout::header::Header;
use crate::components::layout::left_sidebar::LeftSidebar;
use crate::components::layout::right_sidebar::RightSidebar;
use crate::domain::home::{HomeLayout, HomeRoutes};
use crate::domain::tweet::components::tweet_list::TweetList;
use crate::domain::user::routing::page_profile::PageProfile;
use crate::utils::param::PARAM;

#[component]
pub fn App() -> impl IntoView {
    let theme_mode = ThemeMode::init();

    provide_meta_context();

    view! {
        <Title text="Twitter Clone" />

        <Html {..} class=move || if theme_mode.is_dark() { "dark" } else { "" } />

        <Router>
            <AppWrapper>
                <Header />

                <div class="flex flex-1 min-h-0">
                    <LeftSidebar />
                    <main class="overflow-y-auto flex-1 overflow-x-clip min-w-0">
                        <Routes fallback=|| view! { <NotFoundPage /> }>
                            <ParentRoute path=StaticSegment(HomeRoutes::base_url()) view=HomeLayout>
                                <Route path=StaticSegment("") view=|| view! { <TweetList /> } />
                                <Route
                                    path=StaticSegment(HomeRoutes::following_segment())
                                    view=|| view! { <TweetList following=true /> }
                                />
                            </ParentRoute>
                            <Route path=ParamSegment(PARAM::USERNAME) view=PageProfile />
                        </Routes>
                    </main>
                    <RightSidebar />
                </div>
            </AppWrapper>

            <AppBottomNav />
        </Router>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! { <p>"Not Found."</p> }
}
