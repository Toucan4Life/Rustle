#![allow(non_snake_case)]

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

struct WordleEntity{
    word : String,
    frequency : f32,
    entropy : f32
}

fn wordle_solver_new_game()-> Vec<WordleEntity>{
    vec![WordleEntity{word :"coucou".to_string(),entropy:4.32, frequency:1.32}]
}

fn wordle_solver_step(word : String, pattern : String)-> Vec<WordleEntity>{
    vec![WordleEntity{word :word.to_string(),entropy:0.0, frequency:0.0}]
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {}
}

#[component]
fn Home(cx: Scope) -> Element {
    let started = use_state(cx, || false);
    let word_length = use_state(cx, || 0);
    let first_char = use_state(cx, || "".to_string());
    let word = use_state(cx, || "".to_string());
    let pattern = use_state(cx, || "".to_string());
    let mut wordle_entities =use_state(cx, || vec![WordleEntity{word:"Loading".to_string(),entropy:0.0,frequency:0.0}]);
    cx.render(rsx! {
        div {            
            h1 { "Welcome to rustle !" }
            div{
                div{ 
                    class:"form-group",
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
                button { class:"btn btn-primary", onclick: move |_| {started.set(true);wordle_entities.set(wordle_solver_new_game());}, "New Game"}
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
                        button { class:"btn btn-primary", onclick: move |_| wordle_entities.set(wordle_solver_step(word.to_string(),pattern.to_string())), "Apply Step"}
                    }
                }
             }

            table { class :"table", thead {
                tr { 
                    th {"Word" }
                    th {"Frequency" }
                    th {"Entropy" }
                }
                wordle_entities.iter().map(|we| {
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
