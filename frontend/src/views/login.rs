use crate::components::Create_user;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        Create_user {}
    }
}
