use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

pub fn shell(leptos_options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover"
                />
                <link rel="shortcut icon" type_="image/x-icon" href="/favicon.ico" />
                <link rel="stylesheet" href="/pkg/twitter_clone_fullstack.css" />

                // Prevent dark mode flash - must run before page renders
                <script>
                    "if(localStorage.getItem('darkmode')==='true'||(localStorage.getItem('darkmode')===null&&window.matchMedia('(prefers-color-scheme:dark)').matches)){document.documentElement.classList.add('dark')}"
                </script>

                <AutoReload options=leptos_options.clone() />
                <HydrationScripts options=leptos_options />
                <MetaTags />
            </head>

            <body>
                <App />
            </body>
        </html>
    }
}
