use crate::{
    model::{
        auth::{use_auth, AuthState},
        user::{LoginRequest, UserResponse},
    },
    Route,
};
use dioxus::prelude::*;

const CREATE_NEW_USER: Asset = asset!("/assets/styling/new_user.css");

#[component]
pub fn login_component() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    let nav = use_navigator();
    let auth = match try_use_context::<Signal<AuthState>>() {
        Some(auth) => auth,
        None => {
            return rsx! {
                div { class: "error",
                    "Authentication context not available. Please refresh the page."
                }
            };
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: CREATE_NEW_USER }
        div { class: "new-user-form",
            h2 { "Login" }
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
                        println!("Email: {}", email());
                        println!("Password: {}", password());
                        let login = LoginRequest {
                            email: email(),
                            password: password(),
                        };
                        let nav = nav.clone();
                        let mut auth = auth.clone();
                        spawn(async move {
                            let client = reqwest::Client::new();
                            let resp = client
                                .post("http://localhost:8080/login/")
                                .json(&login)
                                .send()
                                .await;
                            match resp {
                                Ok(response) => {
                                    if response.status().is_success() {
                                        match response.json::<UserResponse>().await {
                                            Ok(user_response) => {
                                                println!("Login successful!");
                                                auth.write().login(UserResponse {
                                                    id: user_response.id,
                                                    root_id: user_response.root_id.clone()
                                                });
                                                nav.push(Route::Dashboard {
                                                    root_id: user_response.root_id.clone()
                                                });
                                            }
                                            Err(e) => {
                                                eprintln!("Failed to parse response JSON: {}", e);
                                            }
                                        }
                                    } else {
                                        eprintln!("Login failed with status: {}", response.status());
                                        if let Ok(error_text) = response.text().await {
                                            eprintln!("Error details: {}", error_text);
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
