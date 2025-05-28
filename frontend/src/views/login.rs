use crate::components::{Echo, Hero};
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
