#![allow(non_snake_case)]
#![warn(
    clippy::pedantic
)]
mod WordleSolver;
use axum::{extract::ws::WebSocketUpgrade, response::Html, routing::get, Router};
use dioxus::prelude::*;
use itertools::Itertools;
use WordleSolver::WordleEntity;

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move || async move {
                Html(format!(
                    r#"
                <!DOCTYPE html>
                <html>
                <head> <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap/dist/css/bootstrap.min.css"> <title>Dioxus LiveView with Axum</title>  </head>
                <body> <div id="main"></div> </body>
                {glue}
                </html>
                "#,
                    // Create the glue code to connect to the WebSocket on the "/ws" route
                    glue = dioxus_liveview::interpreter_glue(&format!("ws://{addr}/ws"))
                ))
            }),
        )
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    _ = view.launch(dioxus_liveview::axum_socket(socket), app).await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


fn app(cx: Scope) -> Element {
    let started = use_state(cx, || false);
    let word_length = use_state(cx, || 0);
    let first_char = use_state(cx,  String::new);
    let word = use_state(cx, String::new);
    let pattern = use_state(cx, String::new);
    
    let steps = use_state(cx, Vec::new);
    let recommended_words =use_state(cx, || vec![WordleEntity{word: String::from("Loading"),entropy:0.0,frequency:0.0, is_candidate:false}]);
    let possible_words =use_state(cx, || vec![WordleEntity{word: String::from("Loading"),entropy:0.0,frequency:0.0, is_candidate:false}]);
    let possible_counts = use_state(cx, || 0);
    let recommended_counts = use_state(cx, || 0);
    let entropy = use_state(cx, || 0.0);
    cx.render(rsx! {
        div {            
            h1 { "Welcome to rustle !" }
            div{
                div{
                    label {"Word Length"}
                    input {
                        placeholder: "Word Length",
                        class:"form-control",
                        oninput: move |evt| word_length.set(evt.value.parse().unwrap()),
                    }
                }
                div{ 
                    class:"form-group",
                    label {"First Char"}
                    input {
                        placeholder: "First Char",
                        class:"form-control",
                        oninput: move |evt| first_char.set(evt.value.clone()),
                    }
                    small {class:"form-text text-muted", "Optional"}
                }
                button {
                    class:"btn btn-primary",
                    onclick: move |_| {
                        started.set(true);
                        let mut rec = WordleSolver::retrieve_recommended_words(&[], *word_length.current(),&first_char.current().to_string());
                        rec.sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());
                        let mut pos = rec.iter().filter(|entity| entity.is_candidate).cloned().collect_vec();
                        pos.sort_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());
                        recommended_words.set(rec.iter().take(5).cloned().collect_vec());
                        possible_words.set(pos.iter().take(5).cloned().collect_vec());       
                        possible_counts.set(pos.len());  
                        recommended_counts.set(rec.len());                        
                        entropy.set(WordleSolver::get_uniform_entropy(rec.len().try_into().unwrap()));
                    },
                    "New Game"}
            }
            if *started.get() {
                rsx! {
                    div{
                        div{ 
                            class:"form-group",
                            label {"Word"}
                            input {
                                placeholder: "Word",
                                class:"form-control",
                                oninput: move |evt| word.set(evt.value.clone()),
                            }
                        }
                        div{ 
                            class:"form-group",
                            label {"Pattern"}
                            input {
                                placeholder: "Pattern",
                                class:"form-control",
                                oninput: move |evt| pattern.set(evt.value.clone()),
                            }
                            small {class:"form-text text-muted", "0/1/2 => Incorrect/Misplaced/Correct"}
                        }
                        button { 
                            class:"btn btn-primary",
                            onclick: move |_| {
                                let mut test = steps.current().to_vec();
                                test.push((word.current().to_string(),pattern.current().to_string()));
                                steps.set(test.clone());
                                let mut rec=WordleSolver::retrieve_recommended_words(&test.clone(),*word_length.current(),&first_char.current().to_string());
                                rec.sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());
                                let mut pos = rec.iter().filter(|entity| entity.is_candidate).cloned().collect_vec();
                                pos.sort_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());
                                recommended_words.set(rec.iter().take(5).cloned().collect_vec());
                                possible_words.set(pos.iter().take(5).cloned().collect_vec());   
                                possible_counts.set(pos.len());  
                                recommended_counts.set(rec.len());
                                entropy.set(WordleSolver::get_uniform_entropy(rec.len().try_into().unwrap()));
                            },
                            "Apply Step"}
                    }
                }
             }
            h3 { "Recommended words" }
            label{"{recommended_counts} words, {entropy} total entropy"}
            table { class :"table", thead {
                tr { 
                    th {"Word" }
                    th {"Frequency" }
                    th {"Entropy" }
                }
                recommended_words.iter().map(|we| {
                    rsx!{
                        tr { 
                            td {"{we.word}" }
                            td {"{we.frequency}" }
                            td {"{we.entropy}" }
                        }
                    }
                })              
            }}
            h3 { "Possible words" }
            label{"{possible_counts} words"}
            table { class :"table", thead {
                tr { 
                    th {"Word" }
                    th {"Frequency" }
                    th {"Entropy" }
                }
                possible_words.iter().map(|we| {
                    rsx!{
                        tr { 
                            td {"{we.word}" }
                            td {"{we.frequency}" }
                            td {"{we.entropy}" }
                        }
                    }
                })              
            }}
        }
    })
}
