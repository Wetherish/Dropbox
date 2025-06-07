use crate::components::{File_item, New_dir};
use crate::model::file::File;
use dioxus::prelude::*;

const DASHBOARD: Asset = asset!("/assets/styling/dashboard.css");

#[component]
pub fn Dashboard(root_id: String) -> Element {
    let parent = "Files:documents_bartek".to_string();
    let owner = "User:bartek".to_string();
    let mut show_new_dir_modal = use_signal(|| false);
    let dashboard_link = use_signal(|| root_id);

    let fetch_directory = use_resource(move || {
        let current_id = dashboard_link.read().to_string();
        async move {
            reqwest::get(format!("http://localhost:8080/directoryy/{}", current_id))
                .await
                .unwrap()
                .json::<Vec<File>>()
                .await
        }
    });

    rsx! {
        div { id: "dashboard",
            document::Link { rel: "stylesheet", href: DASHBOARD }
            div { class: "sidebar",
                h3 { "Folders" }
                a { href: "#", "All files" }
                a { href: "#", "Favorites" }
            }
            div { class: "main-content",
                div { class: "toolbar",
                    button { "Create" }
                    button { "Upload" }
                    button {
                        onclick: move |_| show_new_dir_modal.set(true),
                        "Create folder"
                    }
                }
                if *show_new_dir_modal.read() {
                    div { class: "modal-overlay",
                        div { class: "modal",
                            button {
                                class: "close-button",
                                onclick: move |_| show_new_dir_modal.set(false),
                                "×"
                            }
                            New_dir { owner_id: owner.clone(), parent_id: parent.clone() }
                        }
                    }
                }
                match &*fetch_directory.read_unchecked() {
                    Some(Ok(response)) => rsx! {
                        div { class: "folder-grid",
                            for file in response {
                                File_item {
                                    file: file.clone(),
                                    on_folder_click: Some(EventHandler::new({
                                        let mut dashboard_link = dashboard_link.clone();
                                        move |file_id: String| {
                                            dashboard_link.set(file_id);
                                        }
                                    }))
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div { "Loading files failed: {err}" }
                    },
                    None => rsx! {
                        div { "Loading files..." }
                    },
                }
            }
        }
    }
}
