use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Dashboard { id: 1 }, "Dashboard" }

            div { id: "login",
                Link { to: Route::Login {}, "Login" }
            }
        }

        Outlet::<Route> {}
    }
}
