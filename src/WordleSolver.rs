use core::cmp;
use deunicode::deunicode;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct WordleEntity {
    pub word: String,
    pub frequency: f32,
    pub entropy: f32,
    pub is_candidate: bool,
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Pattern {
    Incorrect,
    Misplaced,
    Correct,
}

pub fn get_uniform_entropy(count : i32) -> f32{
    let probabilities = (0..count).map(|_| 1.0 / (count as f32) ).collect_vec();
    get_entropy(&probabilities)
}

pub fn retrieve_recommended_words(
    patterns: &[(String, String)],
    word_length: usize,
    first_char: &str,
) -> Vec<WordleEntity> {
    let word_dictionary = dictionary(word_length, first_char);

    let rules = patterns
        .iter()
        .map(|(word, pattern)| {
            Rule::new(
                word,
                &pattern
                    .chars()
                    .map(|c| match c {
                        '0' => Pattern::Incorrect,
                        '1' => Pattern::Misplaced,
                        '2' => Pattern::Correct,
                        _ => todo!(),
                    })
                    .collect_vec(),
            )
        })
        .collect_vec();

    let possible_words = word_dictionary
        .iter()
        .filter(|(word, _)| rules.iter().all(|rule| rule.Is_Word_Conform(word)))
        .map(|(word, _)| word)
        .cloned()
        .collect_vec();

    word_dictionary
        .par_iter()
        .map(|(word, freq)| WordleEntity {
            entropy: entropy_by_word(word, &possible_words),
            word: word.to_string(),
            frequency: *freq,
            is_candidate: possible_words.contains(word),
        })
        .collect()
}

fn parse_line(line: &str, word_length: usize, first_char: &str) -> Option<(String, f32)> {
    let (word, freq) = line.split_once(';')?;
    let decoded_word = deunicode(word);
    let parsed_freq = freq.parse::<f32>().ok()?;

    if word.chars().all(char::is_alphabetic)
        && word.chars().count() == word_length
        && (first_char.chars().count() == 0 || word.starts_with(first_char.chars().next()?))
    {
        return Some((decoded_word, parsed_freq));
    }
    None
}

fn dictionary(word_length: usize, first_char: &str) -> Vec<(String, f32)> {
    include_str!("Lexique381.csv")
        .lines()
        .filter_map(|line| parse_line(line, word_length, first_char))
        .into_group_map_by(|(word, _)| word.clone())
        .iter()
        .map(|(key, group)| (key.to_string(), group.iter().map(|(_, freq)| freq).sum()))
        .collect_vec()
}

struct Rule {
    character_count: HashMap<char, usize>,
    character_at_least_count: HashMap<char, usize>,
    character_position_to_match: HashMap<usize, char>,
    character_position_to_not_match: Vec<(usize, char)>,
}

impl Rule {
    pub fn new(word: &str, pattern: &[Pattern]) -> Self {
        let mut char_count: HashMap<char, usize> = HashMap::new();
        let mut char_at_least_count: HashMap<char, usize> = HashMap::new();
        let mut char_position_to_match: HashMap<usize, char> = HashMap::new();
        let mut char_position_to_not_match: Vec<(usize, char)> = Vec::new();
        for (key, group) in &pattern
            .iter()
            .enumerate()
            .map(|(index, pat)| (word.chars().nth(index).unwrap(), index, pat))
            .into_group_map_by(|test| test.0)
        {
            if group.iter().any(|test| test.2 == &Pattern::Incorrect) {
                let count = group
                    .iter()
                    .filter(|test| test.2 != &Pattern::Incorrect)
                    .count();
                if let Some(x) = char_count.get_mut(key) {
                    *x = cmp::max(*x, count);
                } else {
                    char_at_least_count.remove(key);
                    char_count.insert(*key, count);
                }
            } else {
                let count = group.len();
                if let Some(x) = char_at_least_count.get_mut(key) {
                    *x = cmp::max(*x, count);
                } else {
                    char_at_least_count.insert(*key, count);
                }
            }
            for grp in group {
                match grp.2 {
                    Pattern::Incorrect | Pattern::Misplaced => {
                        char_position_to_not_match.push((grp.1, grp.0));
                    }
                    Pattern::Correct => {
                        char_position_to_match.insert(grp.1, grp.0);
                    }
                }
            }
        }
        Self {
            character_count: char_count,
            character_at_least_count: char_at_least_count,
            character_position_to_match: char_position_to_match,
            character_position_to_not_match: char_position_to_not_match,
        }
    }

    fn Is_Word_Conform(&self, word: &str) -> bool {
        self.character_position_to_match
            .iter()
            .all(|(pos, char)| word.chars().nth(*pos) == Some(*char))
            && self
                .character_position_to_not_match
                .iter()
                .all(|(pos, char)| word.chars().nth(*pos) != Some(*char))
            && self
                .character_count
                .iter()
                .all(|(char, count)| word.chars().filter(|c| c == char).count() == *count)
            && self
                .character_at_least_count
                .iter()
                .all(|(char, count)| word.chars().filter(|c| c == char).count() >= *count)
    }
}

fn get_pattern(actual_word: &str, target_word: &str) -> Vec<Pattern> {
    let mut pattern_list = vec![Pattern::Incorrect; actual_word.chars().count()];
    for (k, i) in actual_word.chars().zip(target_word.chars()).zip(0..) {
        if k.0 == k.1 {
            pattern_list[i] = Pattern::Correct;
        }
    }
    for (key, group) in actual_word
        .char_indices()
        .filter(|t| pattern_list[t.0] != Pattern::Correct)
        .into_group_map_by(|test| test.1)
    {
        let count = cmp::min(
            target_word
                .char_indices()
                .filter(|t| pattern_list[t.0] != Pattern::Correct && key == t.1)
                .count(),
            group.len(),
        );
        for i in 0..count {
            pattern_list[group[i].0] = Pattern::Misplaced;
        }
    }
    pattern_list
}

fn get_entropy(probabilities: &[f32]) -> f32 {
    probabilities
        .iter()
        .map(|probability| -probability * probability.log2())
        .sum()
}

fn entropy_by_word(actual_word: &str, possible_words: &[String]) -> f32 {
    let patterns = possible_words
        .iter()
        .map(|word| get_pattern(actual_word, word))
        .collect_vec();
    let probabilities = patterns
        .iter()
        .map(|t| (t, t))
        .into_group_map()
        .values()
        .map(|group| (group.len() as f32 / patterns.len() as f32))
        .collect_vec();
    get_entropy(&probabilities)
}

#[cfg(test)]
mod tests;
