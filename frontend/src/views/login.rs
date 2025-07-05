use crate::components::{login_component, Create_user};
use dioxus::prelude::*;

const LOGIN: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Login() -> Element {
    let mut switch = use_signal(|| false);
    rsx! {
        div { class: "login-container",
            document::Link { rel: "stylesheet", href: LOGIN }
            h1 { "Welcome to the App" }
            if switch() {
                Create_user {}
            } else {
                login_component {}
            }
        }
        button { class: "switch-button", onclick: move |_| switch.set(!switch()),
            if switch() {
                "Switch to Login"
            } else {
                "Switch to Create User"
            }
        }
    }
}
