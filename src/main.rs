mod components;
mod game;

use components::pannable_space::ComponentPannableSpace;

use dioxus::prelude::*;
use dioxus_desktop::{WindowBuilder, WindowCloseBehaviour};
use dioxus_desktop::tao::monitor::MonitorHandle;
use dioxus_desktop::tao::window::Fullscreen;
use crate::components::pannable_space::PannableSpaceProps;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Desktop close window is broken and needs to be tinkered with so close is available
    // let config = dioxus_desktop::Config::new().with_close_behaviour(WindowCloseBehaviour::CloseWindow).with_window(WindowBuilder::new().with_closable(true));
    #[cfg(feature = "desktop")]
    let config = dioxus_desktop::Config::new();
    #[cfg(feature = "web")]
    let config = dioxus_web::Config::new();
    #[cfg(feature = "mobile")]
    let config = dioxus_mobile::Config::new();
    LaunchBuilder::new().with_cfg(config).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        ComponentPannableSpace {width: 24.5}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
