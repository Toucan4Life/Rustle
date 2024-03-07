use std::str::FromStr;

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
            .filter(|test| test.0.chars().all(|c| c.is_alphabetic()))
            .ok_or(ParsePointError)?;
        let y_fromstr = freq.parse::<f32>().map_err(|_| ParsePointError)?;

        Ok(WordleEntity {
            word: word_fromstr.to_string(),
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
    pub fn new(word_length: usize, first_char: String) -> Self {
        let mut test: Vec<WordleEntity> = include_str!("Lexique381.csv")
            .lines()
            .filter_map(|line| line.parse::<WordleEntity>().ok())
            .filter(|entity| {
                entity.word.len() == word_length
                    && (first_char.len() == 0
                        || entity.word.starts_with(first_char.chars().nth(0).unwrap()))
            })
            .collect::<Vec<WordleEntity>>();
        test.sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());
        let rec = test.clone();
        test.sort_by(|a, b| b.frequency.partial_cmp(&a.frequency).unwrap());
        let pos = test.clone();
        Self {
            recommended_word: rec,
            possible_word: pos,
        }
    }

    pub fn wordle_solver_step(self,
        word: String,
        pattern: String,
        words: Vec<WordleEntity>,
    ) -> Vec<WordleEntity> {
        vec![WordleEntity {
            word: word.to_string(),
            entropy: 0.0,
            frequency: 0.0,
        }]
    }
}

#[test]
fn Parse_Wordle_entity() {
    let parsed: WordleEntity = "coucou;1.32".parse().unwrap();
    assert_eq!(parsed.word, "coucou");
    assert_eq!(parsed.frequency, 1.32);
}

#[test]
fn Parse_Wordle_entity_special_char() {
    let parsed: WordleEntity = "à;1.32".parse().unwrap();
    assert_eq!(parsed.word, "à");
    assert_eq!(parsed.frequency, 1.32);
}

#[test]
fn Parse_Wordle_entity_space() {
    assert!("à priori;1.32".parse::<WordleEntity>().is_err());
}
