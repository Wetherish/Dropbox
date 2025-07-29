use dioxus::prelude::*;

use views::{Dashboard, File, Home, Login, Navbar, Signup};

use crate::model::auth::AuthState;

mod components;
mod model;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/dashboard/:root_id")]
        Dashboard { root_id: String },
        #[route("/file/:id")]
        File { id: i32 },
        #[route("/login")]
        Login {},
        #[route("/signup")]
        Signup {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AuthState::default()));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
