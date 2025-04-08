use std::thread::Scope;
use sycamore::{prelude::*, web::rt::web_sys};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    id: u32,
    comment: String,
    groups: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Group {
    id: u32,
    name: String,
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let logged_in = create_signal(cx, false);
    let password = create_signal(cx, String::new());
    let client_status = create_signal(cx, String::new());
    let show_password = create_signal(cx, false);
    let selected_client = create_signal(cx, String::new());

    // Check existing session on mount
    spawn_local(async move {
        if let Ok(resp) = reqwest::get("/api/check_auth").await {
            logged_in.set(resp.status().is_success());
        }
    });

    let handle_login = move |_| {
        spawn_local(async move {
            let resp = reqwest::Client::new()
                .post("/api/login")
                .json(&serde_json::json!({ "password": password.get().as_ref() }))
                .send()
                .await;

            if let Ok(resp) = resp {
                if resp.status().is_success() {
                    logged_in.set(true);
                }
            }
        });
    };

    let handle_logout = move |_| {
        spawn_local(async move {
            let _ = reqwest::get("/api/logout").await;
            logged_in.set(false);
        });
    };

    let toggle_client_status = move |_| {
        spawn_local(async move {
            if let Ok(_) = reqwest::Client::new()
                .post("/api/toggle_client")
                .json(&serde_json::json!({ "client": selected_client.get().as_ref() }))
                .send()
                .await
            {
                client_status.set("Status updated".to_string());
            }
        });
    };

    view! { cx,
        div(class="container") {
            h1 { "Pi-hole Client Manager" }
            
            // Login/Logout section
            div(class="auth-section") {
                (if *logged_in.get() {
                    view! { cx,
                        button(on:click=handle_logout) { "Logout" }
                    }
                } else {
                    view! { cx,
                        div(class="password-entry") {
                            input(
                                type=if *show_password.get() { "text" } else { "password" },
                                bind:value=password,
                                placeholder="Enter password"
                            )
                            button(on:click=move |_| show_password.set(!*show_password.get())) {
                                (if *show_password.get() { "🙈" } else { "👁️" })
                            }
                            button(on:click=handle_login) { "Login" }
                        }
                    }
                })
            }

            // Client control section (only shown when logged in)
            (if *logged_in.get() {
                view! { cx,
                    div(class="client-control") {
                        h2 { "Client Management" }
                        
                        // Client selection
                        select(bind:value=selected_client) {
                            option(value="Fire TV cube") { "Fire TV cube" }
                            option(value="Smart Fridge") { "Smart Fridge" }
                        }
                        
                        // Status toggle
                        button(on:click=toggle_client_status) { "Toggle Internet Access" }
                        
                        p { (client_status.get().as_ref()) }
                    }
                }
            } else {
                view! { cx, }
            })
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    sycamore::render(|cx| view! { cx, App {} });
}
