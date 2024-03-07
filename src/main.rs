#![allow(non_snake_case)]

mod WordleSolver;
use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

use WordleSolver::WordleEntity;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
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
    let mut solver : &UseState<Option<WordleSolver::WordleSolver>> = use_state(cx, || None);
    let started = use_state(cx, || false);
    let word_length = use_state(cx, || 0 as usize);
    let first_char = use_state(cx, || "".to_string());
    let word = use_state(cx, || "".to_string());
    let pattern = use_state(cx, || "".to_string());
    let recommended_words =use_state(cx, || vec![WordleEntity{word:"Loading".to_string(),entropy:0.0,frequency:0.0}]);
    let possible_words =use_state(cx, || vec![WordleEntity{word:"Loading".to_string(),entropy:0.0,frequency:0.0}]);
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
                        let sol = WordleSolver::WordleSolver::new(*word_length.current(),first_char.current().to_string());
                        recommended_words.set(sol.recommended_word[0..5].to_vec());
                        possible_words.set(sol.possible_word[0..5].to_vec());
                        solver.set(Some(sol));
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
                                match solver.current().as_ref(){
                                    Some(sol) => { 
                                        let sol2 =sol.clone();
                                        sol2.wordle_solver_step(word.current().to_string(),pattern.current().to_string(),recommended_words.current().to_vec());
                                        let sol3 =sol.clone();
                                        recommended_words.set(sol3.recommended_word[0..5].to_vec());                                       
                                        possible_words.set(sol3.possible_word[0..5].to_vec());},
                                    None => {},
                                }                                                                
                            },
                            "Apply Step"}
                    }
                }
             }

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
