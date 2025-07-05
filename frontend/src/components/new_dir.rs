use crate::model::file::NewFolderRequest;
use dioxus::prelude::*;

const NEW_DIR_CSS: Asset = asset!("/assets/styling/new_dir.css");

#[component]
pub fn New_dir(
    owner_id: String,
    parent_id: String,
    on_success: Option<EventHandler<()>>
) -> Element {
    let mut new_dir_name = use_signal(|| String::new());
    let mut fetch = use_signal(|| "".to_string());
    let mut is_creating = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: NEW_DIR_CSS }
        div { id: "new_dir",
            h3 { "Create New Folder" }
            input {
                class: "folder-input",
                value: "{new_dir_name}",
                oninput: move |e| new_dir_name.set(e.value().clone()),
                placeholder: "Enter folder name",
                disabled: *is_creating.read(),
            }
            button {
                class: "create-button",
                disabled: *is_creating.read() || new_dir_name.read().trim().is_empty(),
                onclick: move |_| {
                    let value1 = owner_id.clone();
                    let value2 = parent_id.clone();
                    let name = new_dir_name.read().trim().to_string();
                    let on_success = on_success.clone();
                    if name.is_empty() {
                        fetch.set("Please enter a folder name.".into());
                        return;
                    }
                    spawn(async move {
                        is_creating.set(true);
                        fetch.set("Creating folder...".into());
                        let resp = reqwest::Client::new()
                            .post("http://localhost:8080/directory/")
                            .json(
                                &NewFolderRequest {
                                    parent_id: value2,
                                    owner_id: value1,
                                    name,
                                },
                            )
                            .send()
                            .await;
                        match resp {
                            Ok(response) if response.status().is_success() => {
                                fetch.set("Folder created successfully!".into());
                                new_dir_name.set(String::new());
                                if let Some(callback) = on_success {
                                    callback.call(());
                                }
                            }
                            Ok(_) => {
                                fetch.set("Failed to create folder.".into());
                                is_creating.set(false);
                            }
                            Err(_) => {
                                fetch.set("Network error. Please try again.".into());
                                is_creating.set(false);
                            }
                        }
                    });
                },
                if *is_creating.read() {
                    "Creating..."
                } else {
                    "Create Folder"
                }
            }
            if !fetch.read().is_empty() {
                p { class: "feedback", "{fetch}" }
            }
        }
    }
}
