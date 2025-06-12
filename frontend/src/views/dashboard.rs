use crate::components::{File_item, New_dir};
use crate::model::file::File;
use dioxus::prelude::*;

const DASHBOARD: Asset = asset!("/assets/styling/dashboard.css");

#[component]
pub fn Dashboard(root_id: String) -> Element {
    dbg!(&root_id);
    let mut show_new_dir_modal = use_signal(|| false);
    let mut dashboard_link = use_signal(|| root_id);

    let open_directory: Resource<Result<Vec<File>, reqwest::Error>> = use_resource(move || {
        let current_id = dashboard_link.read().to_string();
        async move {
            reqwest::get(&format!(
                "http://localhost:8080/open_directory/{}",
                current_id
            ))
            .await?
            .json::<Vec<File>>()
            .await
        }
    });

    let fetch_directory: Resource<Result<File, reqwest::Error>> = use_resource(move || {
        let current_id = dashboard_link.read().to_string();
        async move {
            reqwest::get(&format!("http://localhost:8080/get_dir/{}", current_id))
                .await?
                .json::<File>()
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
                match &*fetch_directory.read() {
                    Some(Ok(response)) => rsx! {
                        div { class: "toolbar",
                            button { "Create" }
                            button { "Upload" }
                            button {
                                onclick: move |_| show_new_dir_modal.set(true),
                                "Create folder"
                            }
                        }
                        {(*show_new_dir_modal.read()).then(|| rsx! {
                            div { class: "modal-overlay",
                                div { class: "modal",
                                    button {
                                        class: "close-button",
                                        onclick: move |_| show_new_dir_modal.set(false),
                                        "×"
                                    }
                                    New_dir {
                                        owner_id: response.owner.clone(),
                                        parent_id: response.id.clone()
                                    }
                                }
                            }
                        })}
                    },
                    Some(Err(err)) => rsx! {
                        div { "Loading files failed: {err}" }
                    },
                    None => rsx! {
                        div { "Loading files..." }
                    },
                }
                match &*open_directory.read() {
                    Some(Ok(response)) => rsx! {
                        div { class: "folder-grid",
                            div {"elems: "{response.len();}  }
                            for file in response {
                                File_item {
                                    file: file.clone(),
                                    on_folder_click: move |file_id: String| {
                                        dashboard_link.set(file_id);
                                    }
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
