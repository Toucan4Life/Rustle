#![allow(non_snake_case)]
#![warn(clippy::pedantic)]
mod WordleSolver;
use dioxus::prelude::*;
use itertools::Itertools;
use WordleSolver::WordleEntity;

fn main() {
    launch(app);
}
#[derive(Clone)]
pub struct WordleStartEntity {
    pub steps:Vec<(String,String)>,
    pub word_length: usize,
    pub first_char: String,
}

fn app() -> Element {
    let rec = use_signal(|| WordleStartEntity{steps:Vec::new(),first_char:String::new(),word_length:0});
    
    rsx! {
        div {
            link { href:"https://cdn.jsdelivr.net/npm/bootstrap/dist/css/bootstrap.css", rel:"stylesheet" }
            h1 { "Welcome to rustle !" }
            WordleInput{rec}
            Recommendations{rec}
       }
    }
}

#[component]
fn WordleInput(rec: Signal<WordleStartEntity>) -> Element {
    rsx! {
        form {
            onsubmit: move |event| {
                rec.set(WordleStartEntity{
                    first_char:event.data.values()["First Char"].as_value(),
                    word_length:event.data.values()["Word Length"].as_value().parse().unwrap(),
                    steps:vec![]})
            },
            input { class:"form-control", name: "Word Length",placeholder: "Word Length" }
            input { class:"form-control", name: "First Char",placeholder: "First Char" }
            input { class:"btn btn-primary", r#type: "submit" }
        }    
        if rec.read().word_length != 0{
        {
            rsx!{
                form {
                    onsubmit: move |event| {
                        rec.write().steps.push((event.data.values()["Word"].as_value(), event.data.values()["Pattern"].as_value()));
                        rec.set(rec.cloned())
                    },
                    input { class:"form-control", name: "Word",placeholder: "Word"}
                    input { class:"form-control", name: "Pattern",placeholder: "Pattern" }
                    small { class:"form-text text-muted", "0/1/2 => Incorrect/Misplaced/Correct" }
                    input { class:"btn btn-primary", r#type: "submit" }
                }
            }
        }        
        }
    }
}

#[component]
fn Recommendations(rec: ReadOnlySignal<WordleStartEntity>) -> Element {
    let se = rec.read();
    let mut recommended = WordleSolver::retrieve_recommended_words(&se.steps, se.word_length, &se.first_char);
    recommended.sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());
    let mut possible = recommended
        .iter()
        .filter(|entity| entity.is_candidate)
        .cloned()
        .collect_vec();
    possible.sort_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());
    rsx! {
        h3 { "Recommended words" }
        label{"{recommended.len()} words, {WordleSolver::get_uniform_entropy(recommended.len().try_into().unwrap())} total entropy"}
        WordleTable{words:recommended, size:5}
        h3 { "Possible words" }
        label{"{possible.len()} words"}
        WordleTable{words:possible, size:5}
    }
}

#[component]
fn WordleTable(words: Vec<WordleEntity>, size : usize) -> Element {
    rsx! {
        table { class :"table", thead {
        tr {
            th {"Word" }
            th {"Frequency" }
            th {"Entropy" }
        }
        {
            words.iter().take(size).map(|we| {
                rsx!{
                    tr {
                        td {"{we.word}" }
                        td {"{we.frequency}" }
                        td {"{we.entropy}" }
                    }
                }
            })
        }}}
    }
}
