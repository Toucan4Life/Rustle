use itertools::Itertools;
use core::{cmp, str::FromStr};
use std::collections::HashMap;
use deunicode::deunicode;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

#[derive(Clone)]
pub struct WordleEntity {
    pub word: String,
    pub frequency: f32,
    pub entropy: f32,
}

impl FromStr for WordleEntity {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (word_fromstr, freq) = s
            .split_once(';')
            .filter(|test| test.0.chars().all(char::is_alphabetic))
            .ok_or(ParsePointError)?;
        let y_fromstr = freq.parse::<f32>().map_err(|_| ParsePointError)?;

        Ok(Self {
            word: deunicode(word_fromstr),
            frequency: y_fromstr,
            entropy: 0.0,
        })
    }
}
#[derive(Clone)]
pub struct WordleSolver {
    pub recommended_word: Vec<WordleEntity>,
    pub possible_word: Vec<WordleEntity>,
}

impl WordleSolver {
    pub fn new(word_length: usize, first_char: &str) -> Self {
        let test = retrieve_words(word_length, first_char);
        let (pos,rec)=retrieve_recommended_words(&[], &test);

        Self {
            recommended_word: rec,
            possible_word: pos,
        }
    }
}
pub fn wordle_solver_step(
    patterns: &[(String, String)],
    word_length: usize,
    first_char: &str,
) -> (Vec<WordleEntity>, Vec<WordleEntity>) {
    let dico = retrieve_words(word_length, first_char);
    retrieve_recommended_words(patterns, &dico)
}
fn retrieve_recommended_words(
    patterns: &[(String, String)],
    word_dictionary: &[WordleEntity],
) -> (Vec<WordleEntity>, Vec<WordleEntity>) {

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
    }).collect_vec();

    let possible_words = word_dictionary
        .iter()
        .filter(|word|rules.iter().all(|rule| rule.Is_Word_Conform(&word.word)))
        .map(|word| &word.word)        
        .collect_vec();

    let mut recommended_words : Vec<WordleEntity> = word_dictionary
        //.iter()
        .into_par_iter()
        .map(|d| {
            WordleEntity {
                entropy: entropy_by_word(&d.word,&possible_words),
                word: d.word.clone(),
                frequency: d.frequency,
            }
        }).collect();

    recommended_words.sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());

    let mut t = recommended_words.iter().filter(|entity| possible_words.contains(&&entity.word)).cloned().collect_vec();
       
    t.sort_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());

    (t, recommended_words)
}

fn retrieve_words(word_length: usize, first_char: &str) -> Vec<WordleEntity> {
    let test: Vec<WordleEntity> = include_str!("Lexique381.csv")
        .lines()
        .filter_map(|line| line.parse::<WordleEntity>().ok())
        .filter(|entity| {
            entity.word.chars().count() == word_length
                && (first_char.chars().count() == 0
                    || entity.word.starts_with(first_char.chars().next().unwrap()))
        })
        .into_group_map_by(|entity| entity.word.clone())
        .iter()
        .map(|(key, group)| WordleEntity {
            entropy: 0.0,
            frequency: group.iter().map(|test| test.frequency).sum(),
            word: key.to_string(),
        })
        .collect::<Vec<WordleEntity>>();
    test
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
pub enum Pattern {
    Incorrect,
    Misplaced,
    Correct,
}
#[derive(Clone)]
pub struct Rule {
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
                    Pattern::Incorrect | Pattern::Misplaced => char_position_to_not_match.push((grp.1, grp.0)),
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

    fn get_pattern(actual_word: &str, target_word: &str) -> Vec<Pattern> {
        let mut pattern_list = vec![Pattern::Incorrect; actual_word.chars().count()];
        for (k, i) in actual_word.chars().zip(target_word.chars()).zip(0..) {
            if k.0 == k.1 {
                pattern_list[i] = Pattern::Correct;
            }
        }
        for (key, group) in actual_word
            .chars()
            .enumerate()
            .filter(|t| pattern_list[t.0] != Pattern::Correct)
            .into_group_map_by(|test| test.1)
        {
            let count = cmp::min(
                target_word
                    .chars()
                    .enumerate()
                    .filter(|t| pattern_list[t.0] != Pattern::Correct)
                    .filter(|t| key == t.1)
                    .count(),
                group.len(),
            );
            for i in 0..count {
                pattern_list[group[i].0] = Pattern::Misplaced;
            }
        }
        pattern_list
    }
}

fn get_entropy(probabilities: &[f32]) -> f32 {
    probabilities
        .iter()
        .map(|probability| -probability * probability.log2())
        .sum()
}

fn entropy_by_word(actual_word: &str, possible_words: &[&String]) -> f32 {
    let patterns = possible_words
        .iter()
        .map(|word| Rule::get_pattern(actual_word, word))
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

#[test]
fn Parse_Wordle_entity() {
    let parsed: WordleEntity = "coucou;1.32".parse().unwrap();
    assert_eq!(parsed.word, "coucou");
    assert_eq!(parsed.frequency, 1.32);
}

#[test]
fn Parse_Wordle_entity_special_char() {
    let parsed: WordleEntity = "père;1.32".parse().unwrap();
    assert_eq!(parsed.word, "pere");
    assert_eq!(4, parsed.word.chars().count());
    assert_eq!(parsed.frequency, 1.32);
}

#[test]
fn Parse_Wordle_entity_space() {
    assert!("à priori;1.32".parse::<WordleEntity>().is_err());
}

#[test]
fn Rule1() {
    let rule = Rule::new(
        "coucou",
        &vec![
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
        ],
    );
    assert!(rule.Is_Word_Conform(&"coucou".to_string()));
}

#[test]
fn Rule2() {
    let rule = Rule::new(
        "coucou",
        &vec![
            Pattern::Correct,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
        ],
    );
    assert!(!rule.Is_Word_Conform(&"toucan".to_string()));
}

#[test]
fn Rule3() {
    let rule = Rule::new(
        "boubbb",
        &vec![
            Pattern::Incorrect,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
        ],
    );
    assert!(rule.Is_Word_Conform(&"coucou".to_string()));
    assert!(rule.Is_Word_Conform(&"toucan".to_string()));
}

#[test]
fn Rule4() {
    let rule = Rule::new(
        "boubbb",
        &vec![
            Pattern::Incorrect,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
        ],
    );
    assert!(rule.Is_Word_Conform(&"coucou".to_string()));
    assert!(rule.Is_Word_Conform(&"toucan".to_string()));
}

#[test]
fn Rule5() {
    let rule = Rule::new(
        "doucat",
        &vec![
            Pattern::Incorrect,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Misplaced,
        ],
    );
    assert!(!rule.Is_Word_Conform(&"coucou".to_string()));
    assert!(rule.Is_Word_Conform(&"toucan".to_string()));
}

#[test]
fn Rule6() {
    let rule = Rule::new(
        "coucot",
        &vec![
            Pattern::Incorrect,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Incorrect,
            Pattern::Misplaced,
        ],
    );
    assert!(!rule.Is_Word_Conform(&"coucou".to_string()));
    assert!(rule.Is_Word_Conform(&"toucan".to_string()));
}

#[test]
fn Rule7() {
    let rule = Rule::new(
        "coucot",
        &vec![
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Incorrect,
        ],
    );
    assert!(rule.Is_Word_Conform(&"coucou".to_string()));
    assert!(!rule.Is_Word_Conform(&"toucan".to_string()));
    assert!(!rule.Is_Word_Conform(&"ehbahnon".to_string()));
    assert!(!rule.Is_Word_Conform(&"couchera".to_string()));
}

#[test]
fn Rule8() {
    let rule = Rule::new(
        "vivre",
        &vec![
            Pattern::Incorrect,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
        ],
    );
    assert!(rule.Is_Word_Conform(&"givre".to_string()));
    assert!(rule.Is_Word_Conform(&"livre".to_string()));
    assert!(!rule.Is_Word_Conform(&"vivre".to_string()));
}

#[test]
fn Rule9() {
    let rule = Rule::new(
        "vivre",
        &vec![
            Pattern::Misplaced,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Incorrect,
        ],
    );
    assert!(rule.Is_Word_Conform(&"tivrv".to_string()));
}

#[test]
fn Rule10() {
    let rule = Rule::new(
        "eeet",
        &vec![
            Pattern::Correct,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Incorrect,
        ],
    );
    assert!(rule.Is_Word_Conform(&"eaye".to_string()));
}

#[test]
fn Rule11() {
    let rule = Rule::new(
        "poursuivis",
        &vec![
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Misplaced,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Misplaced,
        ],
    );
    assert!(!rule.Is_Word_Conform(&"poursuivis".to_string()));
}

#[test]
fn Rule12() {
    let rule = Rule::new(
        "maintenant",
        &vec![
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Misplaced,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
        ],
    );
    assert!(rule.Is_Word_Conform(&"proportion".to_string()));
}

#[test]
fn Rule13() {
    let rule = Rule::new(
        "exactement",
        &vec![
            Pattern::Correct,
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Incorrect,
        ],
    );
    assert!(rule.Is_Word_Conform(&"encourager".to_string()));
    assert!(!rule.Is_Word_Conform(&"maintenant".to_string()));
    assert!(!rule.Is_Word_Conform(&"exactement".to_string()));
}

#[test]
fn Rule14() {
    let rule = Rule::new(
        "habitude",
        &vec![
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Correct,
        ],
    );
    assert!(!rule.Is_Word_Conform(&"mauvaise".to_string()));
}

#[test]
fn Rule15() {
    let rule = Rule::new(
        "ventilateur",
        &vec![
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Misplaced,
            Pattern::Correct,
            Pattern::Misplaced,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Correct,
        ],
    );
    assert!(!rule.Is_Word_Conform(&"realisateur".to_string()));
}

#[test]
fn Rule16() {
    let rule = Rule::new(
        "dansee",
        &vec![
            Pattern::Misplaced,
            Pattern::Misplaced,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Correct,
        ],
    );
    assert!(rule.Is_Word_Conform(&"grande".to_string()));
}

#[test]
fn Rule17() {
    let rule = Rule::new(
        "usurier",
        &vec![
            Pattern::Correct,
            Pattern::Misplaced,
            Pattern::Incorrect,
            Pattern::Correct,
            Pattern::Correct,
            Pattern::Incorrect,
            Pattern::Correct,
        ],
    );
    assert!(!rule.Is_Word_Conform(&"butoirs".to_string()));
}

#[test]
fn Rule18() {
    let rule = Rule::new(
        "abaisse",
        &vec![
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Correct,
            Pattern::Incorrect,
            Pattern::Incorrect,
            Pattern::Correct,
        ],
    );
    assert!(rule.Is_Word_Conform(&"feuille".to_string()));
}

#[test]
fn Entropy1() {
    assert_eq!(1.0, get_entropy(&[0.5, 0.5]))
}

#[test]
fn Entropy2() {
    assert_eq!(2.0, get_entropy(&[0.25, 0.25, 0.25, 0.25]))
}

#[test]
fn Pattern1() {
    let actual = Rule::get_pattern("usurier", "usagers");
    let expected = vec![
        Pattern::Correct,
        Pattern::Correct,
        Pattern::Incorrect,
        Pattern::Misplaced,
        Pattern::Incorrect,
        Pattern::Misplaced,
        Pattern::Incorrect,
    ];
    assert_eq!(actual, expected)
}

#[test]
fn Pattern2() {
    let actual = Rule::get_pattern("usagers", "usurier");
    let expected = vec![
        Pattern::Correct,
        Pattern::Correct,
        Pattern::Incorrect,
        Pattern::Incorrect,
        Pattern::Misplaced,
        Pattern::Misplaced,
        Pattern::Incorrect,
    ];
    assert_eq!(actual, expected)
}

#[test]
fn Pattern3() {
    let actual = Rule::get_pattern("abregee", "feuille");
    let expected = vec![
        Pattern::Incorrect,
        Pattern::Incorrect,
        Pattern::Incorrect,
        Pattern::Misplaced,
        Pattern::Incorrect,
        Pattern::Incorrect,
        Pattern::Correct,
    ];
    assert_eq!(actual, expected)
}

#[test]
fn Pattern4() {
    let actual = Rule::get_pattern("aeriens", "feuille");
    let expected = vec![
        Pattern::Incorrect,
        Pattern::Correct,
        Pattern::Incorrect,
        Pattern::Correct,
        Pattern::Misplaced,
        Pattern::Incorrect,
        Pattern::Incorrect,
    ];
    assert_eq!(actual, expected)
}

#[test]
fn StressTests() {
    let words = retrieve_words(5, "t");
    let mut test = retrieve_recommended_words(&vec![], &words);
    test.1
        .sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());
    let elu = &test.1[0];
    assert_eq!("tarie", elu.word);
}
//sudo apt-get update
//sudo apt install -y linux-perf
//ln -s /usr/bin/perf_5.10 /usr/bin/perf_5.15
//cargo install flamegraph
//cargo flamegraph --unit-test -- StressTests2
#[test]
fn StressTests2() {
    let words = retrieve_words(5, "");
    let mut test = retrieve_recommended_words(&vec![], &words);
    test.1
        .sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());
    let elu = &test.1[0];
    assert_eq!("tarie", elu.word)
}
