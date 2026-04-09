use leptos::context::Provider;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::hooks::use_random_id::use_random_id_for;

mod components {
    use super::*;
    clx! {DropdownMenuGroup, ul, ""}
    clx! {DropdownMenuItem, li, "inline-flex gap-2 items-center w-full rounded-sm px-2 py-1.5 text-sm transition-colors duration-200 text-popover-foreground hover:bg-accent hover:text-accent-foreground [&_svg:not([class*='size-'])]:size-4"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone)]
pub struct DropdownMenuContext {
    pub target_id: String,
}

#[component]
pub fn DropdownMenu(children: Children) -> impl IntoView {
    let target_id = use_random_id_for("dropdown");
    let ctx = DropdownMenuContext { target_id };

    view! {
        <Provider value=ctx>
            <div data-name="DropdownMenu">{children()}</div>
        </Provider>
    }
}

#[component]
pub fn DropdownMenuTrigger(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<DropdownMenuContext>();
    let button_class = tw_merge!(
        "inline-flex items-center justify-center rounded-full size-8 \
         text-muted-foreground hover:bg-muted transition-colors",
        class
    );

    view! {
        <button
            type="button"
            class=button_class
            data-name="DropdownMenuTrigger"
            data-dropdown-trigger=ctx.target_id
            tabindex="0"
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<DropdownMenuContext>();

    let class = tw_merge!(
        "z-50 p-1 rounded-md border bg-card shadow-md w-[180px] h-fit fixed \
         transition-all duration-200 \
         data-[state=closed]:opacity-0 data-[state=closed]:scale-95 \
         data-[state=open]:opacity-100 data-[state=open]:scale-100",
        class
    );

    let target_id = ctx.target_id.clone();

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <div
            data-name="DropdownMenuContent"
            class=class
            id=ctx.target_id
            data-target="target__dropdown"
            data-state="closed"
            data-align="end"
            style="pointer-events: none;"
        >
            {children()}
        </div>

        <script>
            {format!(
                r#"
                (function() {{
                    const setupDropdown = () => {{
                        const dropdown = document.querySelector('#{id}');
                        const trigger = document.querySelector('[data-dropdown-trigger="{id}"]');

                        if (!dropdown || !trigger) {{
                            setTimeout(setupDropdown, 50);
                            return;
                        }}
                        if (dropdown.hasAttribute('data-initialized')) return;
                        dropdown.setAttribute('data-initialized', 'true');

                        let isOpen = false;

                        const updatePosition = () => {{
                            const tr = trigger.getBoundingClientRect();
                            const dr = dropdown.getBoundingClientRect();
                            const spaceBelow = window.innerHeight - tr.bottom;
                            const shouldAbove = tr.top >= dr.height && spaceBelow < dr.height;
                            if (shouldAbove) {{
                                dropdown.style.top = `${{tr.top - dr.height - 6}}px`;
                                dropdown.style.transformOrigin = 'right bottom';
                            }} else {{
                                dropdown.style.top = `${{tr.bottom + 6}}px`;
                                dropdown.style.transformOrigin = 'right top';
                            }}
                            dropdown.style.left = `${{tr.right - dr.width}}px`;
                        }};

                        const openDropdown = () => {{
                            isOpen = true;
                            dropdown.setAttribute('data-state', 'open');
                            dropdown.style.visibility = 'hidden';
                            dropdown.style.pointerEvents = 'auto';
                            dropdown.offsetHeight;
                            updatePosition();
                            dropdown.style.visibility = 'visible';
                            window.ScrollLock.lock();
                            setTimeout(() => document.addEventListener('click', handleOutside), 0);
                        }};

                        const closeDropdown = () => {{
                            isOpen = false;
                            dropdown.setAttribute('data-state', 'closed');
                            dropdown.style.pointerEvents = 'none';
                            document.removeEventListener('click', handleOutside);
                            window.ScrollLock.unlock(200);
                        }};

                        const handleOutside = (e) => {{
                            if (!dropdown.contains(e.target) && !trigger.contains(e.target)) closeDropdown();
                        }};

                        trigger.addEventListener('click', (e) => {{
                            e.stopPropagation();
                            document.querySelectorAll('[data-target="target__dropdown"]').forEach(dd => {{
                                if (dd !== dropdown && dd.getAttribute('data-state') === 'open') {{
                                    dd.setAttribute('data-state', 'closed');
                                    dd.style.pointerEvents = 'none';
                                    if (window.ScrollLock) window.ScrollLock.unlock(200);
                                }}
                            }});
                            isOpen ? closeDropdown() : openDropdown();
                        }});

                        dropdown.querySelectorAll('[data-dropdown-close]').forEach(el => {{
                            el.addEventListener('click', closeDropdown);
                        }});

                        document.addEventListener('keydown', (e) => {{
                            if (e.key === 'Escape' && isOpen) {{ e.preventDefault(); closeDropdown(); }}
                        }});
                    }};

                    document.readyState === 'loading'
                        ? document.addEventListener('DOMContentLoaded', setupDropdown)
                        : setupDropdown();
                }})();
                "#,
                id = target_id,
            )}
        </script>
    }
}

/// A clickable action item that auto-closes the dropdown.
#[component]
pub fn DropdownMenuAction(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!(
        "inline-flex gap-2 items-center w-full text-sm text-left px-2 py-1.5 rounded-sm \
         transition-colors cursor-pointer text-popover-foreground \
         hover:bg-accent hover:text-accent-foreground \
         [&_svg:not([class*='size-'])]:size-4",
        class
    );

    view! {
        <button type="button" data-name="DropdownMenuAction" class=class data-dropdown-close="true">
            {children()}
        </button>
    }
}
