use dioxus::prelude::*;
use dioxus_web::WebFileEngineExt;
use gloo_file::futures::read_as_bytes;
use gloo_file::Blob as GlooBlob;
use gloo_net::http::Request;
use serde::Serialize;
use web_sys::Blob;

const FILE_INPUT_CSS: Asset = asset!("/assets/styling/file_input.css");

#[derive(Debug, Serialize, Clone)]
pub struct FileUploadRequest {
    pub name: String,
    pub owner: String,
    pub parent: String,
    pub size: i64,
    pub file_type: String,
}

#[component]
pub fn File_input() -> Element {
    let upload_status = use_signal(|| String::new());

    let status_class = match upload_status.read().as_str() {
        s if s.is_empty() => "empty",
        s if s.contains("📤") => "uploading",
        s if s.contains("✅") => "success",
        s if s.contains("❌") => "error",
        _ => "",
    };

    rsx! {
        document::Link { rel: "stylesheet", href: FILE_INPUT_CSS }
        div { class: "file-upload-container",
            div { class: "file-upload-wrapper",
                input {
                    class: "file-upload-input",
                    r#type: "file",
                    onchange: move |event| {
                        let mut upload_status = upload_status.clone();
                        async move {
                            if let Some(file_engine) = event.files() {
                                let file_names = file_engine.files();
                                for name in file_names {
                                    if let Some(file) = file_engine.get_web_file(&name).await {
                                        let file_name = file.name();
                                        let size = file.size();
                                        let mime = file.type_();
                                        let blob_ref: &Blob = file.as_ref();
                                        let gloo_blob = GlooBlob::from(blob_ref.clone());

                                        upload_status.set("📤 Starting upload...".to_string());

                                        let upload_req = FileUploadRequest {
                                            name: file_name.clone(),
                                            owner: "User:pgpefl2i1fyp0qb7jefy".to_string(),
                                            parent: "Files:0urf5efvnd6z72gle94b".to_string(),
                                            size: size as i64,
                                            file_type: mime,
                                        };

                                        let resp = Request::post("http://localhost:8080/upload_url")
                                            .header("Content-Type", "application/json")
                                            .body(serde_json::to_string(&upload_req).unwrap())
                                            .unwrap()
                                            .send()
                                            .await;

                                        match resp {
                                            Ok(res) => {
                                                match res.text().await {
                                                    Ok(upload_url) => {
                                                        upload_status.set("📤 Uploading file...".to_string());

                                                        match read_as_bytes(&gloo_blob).await {
                                                            Ok(bytes) => {
                                                                let upload_resp = Request::put(upload_url.trim())
                                                                    .body(bytes)
                                                                    .unwrap()
                                                                    .send()
                                                                    .await;

                                                                match upload_resp {
                                                                    Ok(_) => {
                                                                        upload_status.set("✅ Upload successful".to_string());
                                                                    }
                                                                    Err(e) => {
                                                                        upload_status.set(format!("❌ Upload failed: {e}"));
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => {
                                                                upload_status.set(format!("❌ Failed to read file: {e}"));
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        upload_status.set(format!("❌ Failed to get upload URL: {e}"));
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                upload_status.set(format!("❌ Request failed: {e}"));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                span { class: "upload-icon", "📁" }
                span { class: "upload-text", "Choose File" }
            }
            if !upload_status.read().is_empty() {
                p { class: "upload-status {status_class}", "{upload_status}" }
            }
        }
    }
}
