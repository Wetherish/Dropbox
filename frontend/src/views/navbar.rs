use crate::{
    model::auth::{self, use_auth, AuthState},
    Route,
};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

pub fn Navbar() -> Element {
    let auth = try_use_context::<Signal<AuthState>>();

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { id: "navbar",
            div { id: "nav-left",
                Link { to: Route::Home {}, "Home" }

                if let Some(auth) = auth {
                    if auth.read().is_authenticated {
                        if let Some(user) = &auth.read().user {
                            Link {
                                to: Route::Dashboard {
                                    root_id: user.root_id.clone(),
                                },
                                "Dashboard"
                            }
                        }
                    }

                    if auth.read().is_authenticated {
                        Link { to: Route::File { id: 1 }, "File" }
                    }
                }
            }

            div { id: "login",
                if let Some(mut auth) = auth {
                    if auth.read().is_authenticated {
                        button {
                            onclick: move |_| {
                                auth.write().logout();
                            },
                            "Logout"
                        }
                    } else {
                        Link { to: Route::Login {}, "Login" }
                    }
                } else {
                    Link { to: Route::Login {}, "Login" }
                }
            }
        }

        Outlet::<Route> {}
    }
}
