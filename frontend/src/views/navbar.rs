use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar",
            div { id: "nav-left",
                Link { to: Route::Home {}, "Home" }
                Link {
                    to: Route::Dashboard {
                        root_id: "Files:documents_bartek".to_string(),
                    },
                    "Dashboard"
                }
                Link { to: Route::File { id: 1 }, "File" }
            }

            div { id: "login",
                Link { to: Route::Login {}, "Login" }
            }
        }

        Outlet::<Route> {}
    }
}
