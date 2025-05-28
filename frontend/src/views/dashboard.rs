use crate::Route;
use dioxus::prelude::*;
use crate::model::file::File;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Dashboard(id: i32) -> Element {
    let file_id = 0;
    let mut files = vec!{"a.txt"};
    let mut future = use_resource(|| async move {
        reqwest::get("http://localhost:8080/directoryy/Files:documents_bartek")
            .await
            .unwrap()
            .json::<Vec<File>>()
            .await
    });

    match &*future.read_unchecked() {
        Some(Ok(response)) => rsx! {
        button { onclick: move |_| future.restart(), "Click to fetch another doggo" }
        div {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            h2 { "My Files" }
            ul {
                for file in response {
                    li {
                        Link { to: Route::File { id: file_id }, "{file.name}" }
                    }
                }
            }
                // Link { to: Route::Upload {}, "Upload New File" }
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

    // rsx! {
    //
    // }
}
