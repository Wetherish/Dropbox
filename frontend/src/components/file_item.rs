use crate::model::file::File;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn File_item(file: File, on_folder_click: Option<EventHandler<String>>) -> Element {
    if file.file_type == "dir" {
        rsx! {
            Link {
                to: Route::Dashboard {
                    root_id: file.id.clone(),
                },
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
                onclick: move |_| {
                    let file_id = file.id.clone();
                    spawn(async move {
                        if let Ok(url) = get_file_url(&file_id).await {
                            if let Some(window) = web_sys::window() {
                                let _ = window
                                    .open_with_url(&url)
                                    .map_err(|e| {
                                        dbg!(e);
                                    });
                            }
                        }
                    });
                },
                div { class: "folder",
                    div { class: "folder-icon", "🖹" }
                    div { class: "folder-name", "{file.name}" }
                }
            }
        }
    }
}

async fn get_file_url(file_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:8080/file_url/{}", file_id);
    let response = reqwest::get(&url).await?;
    let file_url = response.text().await?;
    Ok(file_url)
}
