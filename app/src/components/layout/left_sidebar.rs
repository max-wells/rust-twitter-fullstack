use icons::{Bell, House, Mail, Pen};
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::hooks::use_is_current_path::use_is_current_path;
use crate::components::ui::avatar::Avatar;
use crate::domain::home::routes::HomeRoutes;

#[component]
pub fn LeftSidebar() -> impl IntoView {
    let is_current_path = use_is_current_path();

    let home_active = {
        let icp = is_current_path.clone();
        move || icp(HomeRoutes::base_url()) == "page"
    };

    view! {
        <aside class="hidden overflow-y-auto flex-col py-3 px-2 h-full border-r sm:flex w-[72px] shrink-0 xl:w-[275px]">
            // Logo
            <div class="flex justify-center px-3 mb-4 xl:justify-start">
                <A href="/" attr:class="flex items-center gap-3 w-fit">
                    <svg
                        viewBox="0 0 1200 1227"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                        class="size-7 shrink-0"
                    >
                        <path
                            d="M714.163 519.284L1160.89 0H1055.03L667.137 450.887L357.328 0H0L468.492 681.821L0 1226.37H105.866L515.491 750.218L842.672 1226.37H1200L714.137 519.284H714.163ZM569.165 687.828L521.697 619.934L144.011 79.6944H306.615L611.412 515.685L658.88 583.579L1055.08 1150.3H892.476L569.165 687.854V687.828Z"
                            fill="currentColor"
                        />
                    </svg>
                    <span class="hidden text-xl font-bold tracking-tight xl:block">"Twitter Clone"</span>
                </A>
            </div>

            // Nav items
            <nav class="flex flex-col gap-1">
                <A
                    href=HomeRoutes::base_url()
                    attr:class=move || {
                        let bold = if home_active() { " font-bold" } else { "" };
                        format!(
                            "flex items-center justify-center xl:justify-start gap-4 px-3 py-3 rounded-full hover:bg-muted{bold}",
                        )
                    }
                >
                    <House class="size-6 shrink-0" />
                    <span class="hidden text-base xl:block">"Home"</span>
                </A>

                // Notifications — stub
                <div class="flex gap-4 justify-center items-center py-3 px-3 rounded-full cursor-pointer xl:justify-start hover:bg-muted">
                    <Bell class="size-6 shrink-0" />
                    <span class="hidden text-base xl:block">"Notifications"</span>
                </div>

                // Messages — stub
                <div class="flex gap-4 justify-center items-center py-3 px-3 rounded-full cursor-pointer xl:justify-start hover:bg-muted">
                    <Mail class="size-6 shrink-0" />
                    <span class="hidden text-base xl:block">"Messages"</span>
                </div>
            </nav>

            // Tweet button
            <div class="px-2 mt-4">
                // xl+: full text
                <A
                    href=HomeRoutes::base_url()
                    attr:class="hidden xl:flex w-full rounded-full bg-primary text-primary-foreground font-bold py-3 items-center justify-center hover:bg-primary/90"
                >
                    "Tweet"
                </A>
                // sm–xl: icon only
                <A
                    href=HomeRoutes::base_url()
                    attr:class="xl:hidden flex items-center justify-center rounded-full size-12 bg-primary text-primary-foreground hover:bg-primary/90 mx-auto"
                >
                    <Pen class="size-5" />
                </A>
            </div>

            // Spacer
            <div class="flex-1" />

            // Current user (Alice)
            <A
                href="/alice"
                attr:class="flex items-center justify-center xl:justify-start gap-3 p-3 rounded-full"
            >
                <Avatar
                    src=Some("https://api.dicebear.com/9.x/pixel-art/svg?seed=alice".to_string())
                    fallback="A"
                    class="size-9"
                />
                <div class="hidden min-w-0 xl:block">
                    <p class="font-semibold truncate">"Alice"</p>
                    <p class="text-sm text-muted-foreground truncate">"@alice"</p>
                </div>
            </A>
        </aside>
    }
}
