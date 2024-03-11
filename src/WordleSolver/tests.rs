use super::*;
#[test]
fn Parse_Wordle_entity() {
    let parsed = parse_line("coucou;1.32", 6, "");
    assert_eq!(parsed, Some(("coucou".to_string(),1.32)));
}

#[test]
fn Parse_Wordle_entity_special_char() {
    let parsed = parse_line("père;1.32", 4, "");
    assert_eq!(parsed, Some(("pere".to_string(),1.32)));
}

#[test]
fn Parse_Wordle_entity_space() {
    let parsed = parse_line("à priori;1.32", 8, "");
    assert_eq!(parsed, None);
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
    let actual = get_pattern("usurier", "usagers");
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
    let actual = get_pattern("usagers", "usurier");
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
    let actual = get_pattern("abregee", "feuille");
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
    let actual = get_pattern("aeriens", "feuille");
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
    let mut test = retrieve_recommended_words(&vec![], 5, "t");
    test.sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());
    let elu = &test[0];
    assert_eq!("tarie", elu.word);
}
//sudo apt-get update
//sudo apt install -y linux-perf
//ln -s /usr/bin/perf_5.10 /usr/bin/perf_5.15
//cargo install flamegraph
//cargo flamegraph --unit-test -- StressTests2
#[test]
fn StressTests2() {
    let mut test = retrieve_recommended_words(&vec![], 5, "");
    test.sort_by(|a, b| b.entropy.partial_cmp(&a.entropy).unwrap());
    let elu = &test[0];
    assert_eq!("tarie", elu.word)
}

#[test]
fn Get_uniform_entropy() {
    let test = get_uniform_entropy(5037);
    assert_eq!(12.297734, test)
}
