use crate::Route;
use dioxus::prelude::*;
use crate::model::file::{File, NewFolderRequest};

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Dashboard(id: i32) -> Element {
    let file_id = 0;
    let mut fetch_directory = use_resource(|| async move {
        reqwest::get("http://localhost:8080/directoryy/Files:documents_bartek")
            .await
            .unwrap()
            .json::<Vec<File>>()
            .await
    });

    let mut new_dir_name = use_signal(|| String::new());
    let parent = "Files:documents_bartek".to_string();
    let owner = "User:bartek".to_string();
    let mut fetch = use_signal(|| "Click to start a request".to_string());
    match &*fetch_directory.read_unchecked() {
        Some(Ok(response)) => rsx! {
        div {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        div {
            h2 { "My Files" }
                button {
                        onclick: move |_| {
                                let value1 = owner.clone();
                                let value2 = parent.clone();
                                async move {
                                    fetch_directory.restart();
                                    let resp = reqwest::Client::new()
                                        .post("http://localhost:8080/directory/")
                                        .json(&NewFolderRequest{
                                            parent_id: value2,
                                            owner_id: value1,
                                            name: new_dir_name.to_string(),
                                        })
                                        .send()
                                        .await;

                                    if resp.is_ok() {
                                        fetch.set("ehh".into());
                                    } else  {
                                        fetch.set("failed to fetch response!".into());
                                    }
                                }
                            },
                            "{fetch}"
                    }
               input {
                    value: "{new_dir_name}",
                    oninput: move |e| new_dir_name.set(e.value().clone()),
                    placeholder: "New folder name",
                }
            ul {
                for file in response {
                    li {
                        Link { to: Route::File { id: file_id }, "{file.name}" }
                    }
                }
            }
        }
        }
    },
        Some(Err(eee)) => rsx! {
        div { "Loading dogs failed {eee}" }
    },
        None => rsx! {
        div { "Loading dogs..." }
    },
    }
}
