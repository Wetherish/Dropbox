use crate::model::file::File;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn File_item(file: File, on_folder_click: Option<EventHandler<String>>) -> Element {
    if file.file_type == "dir" {
        rsx! {
            Link {
                to: Route::Dashboard { root_id: file.id.clone() },
                class: "folder-btn",
                onclick: {
                    let file_id = file.id.clone();
                    move |_| {
                        if let Some(handler) = on_folder_click {
                            handler.call(file_id.clone());
                        }
                    }
                },
                div { class: "folder",
                    div { class: "folder-icon", "📁" }
                    div { class: "folder-name", "{file.name}" }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "folder-btn",
                div { class: "folder",
                    div { class: "folder-icon", "🖹" }
                    div { class: "folder-name", "{file.name}" }
                }
            }
        }
    }
}
