use leptos::html;
use leptos::prelude::*;
use tw_merge::tw_merge;

/// Registry Textarea — styled with border/focus-ring by default.
/// For Twitter-style borderless use, override via `class`:
/// `"border-0 shadow-none focus-visible:ring-0 px-0 py-0 min-h-0 rounded-none"`
#[component]
pub fn Textarea(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] readonly: bool,
    #[prop(optional)] required: bool,
    #[prop(optional)] autofocus: bool,
    #[prop(into, optional)] rows: Option<u32>,
    #[prop(into, optional)] maxlength: Option<u32>,
    #[prop(into, optional)] bind_value: Option<RwSignal<String>>,
    #[prop(optional)] node_ref: NodeRef<html::Textarea>,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "border-input placeholder:text-muted-foreground focus-visible:border-ring \
         focus-visible:ring-ring/50 dark:bg-input/30 flex field-sizing-content min-h-16 \
         w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs \
         transition-[color,box-shadow] outline-none focus-visible:ring-2 \
         disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        class
    );

    match bind_value {
        Some(signal) => view! {
            <textarea
                class=merged_class
                placeholder=placeholder
                name=name
                id=id
                disabled=disabled
                readonly=readonly
                required=required
                autofocus=autofocus
                rows=rows
                maxlength=maxlength
                bind:value=signal
                node_ref=node_ref
            />
        }
        .into_any(),
        None => view! {
            <textarea
                class=merged_class
                placeholder=placeholder
                name=name
                id=id
                disabled=disabled
                readonly=readonly
                required=required
                autofocus=autofocus
                rows=rows
                maxlength=maxlength
                node_ref=node_ref
            />
        }
        .into_any(),
    }
}
