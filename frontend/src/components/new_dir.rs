use crate::model::file::NewFolderRequest;
use dioxus::prelude::*;

const NEW_DIR_CSS: Asset = asset!("/assets/styling/new_dir.css");

#[component]
pub fn New_dir(owner_id: String, parent_id: String) -> Element {
    let mut new_dir_name = use_signal(|| String::new());
    let mut fetch = use_signal(|| "".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: NEW_DIR_CSS }

        div { id: "new_dir",
            h3 { "Create New Folder" }

            input {
                class: "folder-input",
                value: "{new_dir_name}",
                oninput: move |e| new_dir_name.set(e.value().clone()),
                placeholder: "Enter folder name",
            }

            button {
                class: "create-button",
                onclick: move |_| {
                    let value1 = owner_id.clone();
                    let value2 = parent_id.clone();
                    let name = new_dir_name.to_string();
                    async move {
                        let resp = reqwest::Client::new()
                            .post("http://localhost:8080/directory/")
                            .json(&NewFolderRequest {
                                parent_id: value2,
                                owner_id: value1,
                                name,
                            })
                            .send()
                            .await;

                        if resp.is_ok() {
                            fetch.set("Folder created!".into());
                            new_dir_name.set(String::new());
                        } else {
                            fetch.set("Failed to create folder.".into());
                        }
                    }
                },
                "Create Folder"
            }

            if !fetch.read().is_empty() {
                p { class: "feedback", "{fetch}" }
            }
        }
    }
}
