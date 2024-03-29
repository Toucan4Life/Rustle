#![allow(non_snake_case)]
#![warn(clippy::pedantic)]
mod WordleSolver;
use dioxus::prelude::*;
use itertools::Itertools;
use WordleSolver::WordleEntity;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut started = use_signal(|| false);
    let mut word_length = use_signal(|| 0);
    let mut first_char = use_signal(String::new);
    let mut word = use_signal(String::new);
    let mut pattern = use_signal(String::new);

    let mut steps = use_signal(Vec::new);
    let mut recommended_words = use_signal(|| {
        vec![WordleEntity {
            word: String::from("Loading"),
            entropy: 0.0,
            frequency: 0.0,
            is_candidate: false,
        }]
    });
    let mut possible_words = use_signal(|| {
        vec![WordleEntity {
            word: String::from("Loading"),
            entropy: 0.0,
            frequency: 0.0,
            is_candidate: false,
        }]
    });
    let mut possible_counts = use_signal(|| 0);
    let mut recommended_counts = use_signal(|| 0);
    let mut entropy = use_signal(|| 0.0);
    rsx! {
        div {
            link { href:"https://cdn.jsdelivr.net/npm/bootstrap/dist/css/bootstrap.css", rel:"stylesheet" }
            h1 { "Welcome to rustle !" }
            div{
                div{
                    label {"Word Length"}
                    input {
                        placeholder: "Word Length",
                        class:"form-control",
                        oninput: move |evt| word_length.set(evt.value().parse().unwrap()),
                    }
                }
                div{
                    class:"form-group",
                    label {"First Char"}
                    input {
                        placeholder: "First Char",
                        class:"form-control",
                        oninput: move |evt| first_char.set(evt.value().clone()),
                    }
                    small {class:"form-text text-muted", "Optional"}
                }
                button {
                    class:"btn btn-primary",
                    onclick: move |_| {
                        started.set(true);
                        let mut rec = WordleSolver::retrieve_recommended_words(&[], (*word_length)(),&first_char.to_string());
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
            if started() {
               { rsx!{
                    div{
                        div{
                            class:"form-group",
                            label {"Word"}
                            input {
                                placeholder: "Word",
                                class:"form-control",
                                oninput: move |evt| word.set(evt.value().clone()),
                            }
                        }
                        div{
                            class:"form-group",
                            label {"Pattern"}
                            input {
                                placeholder: "Pattern",
                                class:"form-control",
                                oninput: move |evt| pattern.set(evt.value().clone()),
                            }
                            small {class:"form-text text-muted", "0/1/2 => Incorrect/Misplaced/Correct"}
                        }
                        button {
                            class:"btn btn-primary",
                            onclick: move |_| {
                                let mut test = steps().to_vec();
                                test.push((word.to_string(),pattern.to_string()));
                                steps.set(test.clone());
                                let mut rec=WordleSolver::retrieve_recommended_words(&test.clone(),(*word_length)(),&first_char.to_string());
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
                }}
             }
            h3 { "Recommended words" }
            label{"{recommended_counts} words, {entropy} total entropy"}
            table { class :"table", thead {
                tr {
                    th {"Word" }
                    th {"Frequency" }
                    th {"Entropy" }
                }
                {
                recommended_words.iter().map(|we| {
                    rsx!{
                        tr {
                            td {"{we.word}" }
                            td {"{we.frequency}" }
                            td {"{we.entropy}" }
                        }
                    }
                })}
            }}
            h3 { "Possible words" }
            label{"{possible_counts} words"}
            table { class :"table", thead {
                tr {
                    th {"Word" }
                    th {"Frequency" }
                    th {"Entropy" }
                }
                {possible_words.iter().map(|we| {
                    rsx!{
                        tr {
                            td {"{we.word}" }
                            td {"{we.frequency}" }
                            td {"{we.entropy}" }
                        }
                    }
                })}
            }}
        }
    }
}
