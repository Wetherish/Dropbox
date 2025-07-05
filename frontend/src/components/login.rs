use crate::{model::user::LoginRequest, Route};
use dioxus::prelude::*;

const CREATE_NEW_USER: Asset = asset!("/assets/styling/new_user.css");

#[component]
pub fn login_component() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut root_id = use_signal(|| String::new());

    let nav = use_navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: CREATE_NEW_USER }
        div {
            class: "new-user-form",
            h2 { "Login" }
            div {
                class: "input-group",
                label {
                    r#for: "email",
                    "Email:"
                }
                input {
                    id: "email",
                    r#type: "email",
                    placeholder: "Enter email address",
                    value: "{email}",
                    oninput: move |evt| email.set(evt.value()),
                }
            }
            div {
                class: "input-group",
                label {
                    r#for: "password",
                    "Password:"
                }
                input {
                    id: "password",
                    r#type: "password",
                    placeholder: "Enter password",
                    value: "{password}",
                    oninput: move |evt| password.set(evt.value()),
                }
            }
            div {
                class: "button-group",
                button {
                    r#type: "submit",
                    onclick: move |_| {
                        println!("Email: {}", email());
                        println!("Password: {}", password());

                        let login = LoginRequest {
                            email: email(),
                            password: password(),
                        };

                        let nav = nav.clone();
                        spawn(async move {
                            let client = reqwest::Client::new();
                            let resp = client
                                .post("http://localhost:8080/login/")
                                .json(&login)
                                .send()
                                .await;

                            match resp {
                                Ok(response) => {
                                    println!("User created successfully: {:?}", response.status());
                                    match response.text().await {
                                        Ok(root_id_value) => {
                                            // Fetch the root directory using the received root_id
                                            let client = reqwest::Client::new();
                                            match client
                                                .get(format!("http://localhost:8080/get_root_dir/{}", root_id_value))
                                                .send()
                                                .await
                                            {
                                                Ok(resp) => {
                                                    match resp.text().await {
                                                        Ok(root_dir) => {
                                                            root_id.set(root_dir.clone());
                                                            let mut id = String::from("Files:");
                                                            id.push_str(&root_dir);
                                                            nav.push(Route::Dashboard {
                                                                root_id: id
                                                            });
                                                        }
                                                        Err(e) => {
                                                            println!("Error reading root dir response: {:?}", e);
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    println!("Error fetching root dir: {:?}", e);
                                                }
                                            }
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
                    "Login"
                }
                button {
                    r#type: "button",
                    onclick: move |_| {
                        email.set(String::new());
                        password.set(String::new());
                    },
                    "Clear"
                }
            }
        }
    }
}
