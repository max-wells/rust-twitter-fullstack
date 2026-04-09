use leptos::prelude::*;
use tw_merge::tw_merge;

/// Circular avatar with image or initial fallback.
/// Default size: size-10. Override via `class` (e.g. `"size-16 border-4 border-background"`).
#[component]
pub fn Avatar(
    src: Option<String>,
    #[prop(into, optional)] alt: String,
    #[prop(into, optional)] fallback: String,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let container = tw_merge!("relative flex shrink-0 overflow-hidden rounded-full size-10", class);

    view! {
        <div class=container>
            {match src {
                Some(url) => {
                    view! { <img src=url alt=alt class="absolute inset-0 size-full object-cover" /> }
                        .into_any()
                }
                None => {
                    view! {
                        <div class="absolute inset-0 flex size-full items-center justify-center bg-muted text-muted-foreground font-semibold text-sm">
                            {fallback}
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
