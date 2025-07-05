use crate::{model::user::NewUserRequest, Route};
use dioxus::prelude::*;

const CREATE_NEW_USER: Asset = asset!("/assets/styling/new_user.css");

#[component]
pub fn Create_user() -> Element {
    let mut username = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let nav = use_navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: CREATE_NEW_USER }
        div { class: "new-user-form",
            h2 { "Create New User" }
            div { class: "input-group",
                label { r#for: "username", "Username:" }
                input {
                    id: "username",
                    r#type: "text",
                    placeholder: "Enter username",
                    value: "{username}",
                    oninput: move |evt| username.set(evt.value()),
                }
            }
            div { class: "input-group",
                label { r#for: "email", "Email:" }
                input {
                    id: "email",
                    r#type: "email",
                    placeholder: "Enter email address",
                    value: "{email}",
                    oninput: move |evt| email.set(evt.value()),
                }
            }
            div { class: "input-group",
                label { r#for: "password", "Password:" }
                input {
                    id: "password",
                    r#type: "password",
                    placeholder: "Enter password",
                    value: "{password}",
                    oninput: move |evt| password.set(evt.value()),
                }
            }
            div { class: "button-group",
                button {
                    r#type: "submit",
                    onclick: move |_| {
                        println!("Username: {}", username());
                        println!("Email: {}", email());
                        println!("Password: {}", password());
                        let mut user_req = NewUserRequest::new();
                        user_req.password = password();
                        user_req.username = username();
                        user_req.email = email();
                        let nav = nav.clone();
                        spawn(async move {
                            let client = reqwest::Client::new();
                            let resp = client
                                .post("http://localhost:8080/new_user/")
                                .json(&user_req)
                                .send()
                                .await;
                            match resp {
                                Ok(response) => {
                                    println!("User created successfully: {:?}", response.status());
                                    match response.text().await {
                                        Ok(root_id) => {
                                            let mut id = String::from("Files:");
                                            id.push_str(&root_id);
                                            nav.push(Route::Dashboard { root_id: id });
                                        }
                                        Err(e) => {
                                            println!("Error reading response: {:?}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Error creating user: {:?}", e);
                                }
                            }
                        });
                    },
                    "Create User"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        username.set(String::new());
                        email.set(String::new());
                        password.set(String::new());
                    },
                    "Clear"
                }
            }
        }
    }
}
