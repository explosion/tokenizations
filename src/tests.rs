use crate::*;

#[test]
fn test_get_alignment() {
    let testcases = vec![
        (
            (vec!["fあo①が", "bar"], vec!["fあo1かb", "ar"]),
            (vec![vec![0], vec![0, 1]], vec![vec![0, 1], vec![1]]),
        ),
        (
            (vec!["New York"], vec!["New", "York"]),
            (vec![vec![0, 1]], vec![vec![0], vec![0]]),
        ),
        (
            (vec!["A'B"], vec!["A", "B"]),
            (vec![vec![0, 1]], vec![vec![0], vec![0]]),
        ),
        (
            (vec!["A'b"], vec!["a", "b"]),
            (vec![vec![0, 1]], vec![vec![0], vec![0]]),
        ),
        (
            (vec![""], vec!["", ""]),
            (vec![vec![]], vec![vec![], vec![]]),
        ),
        (
            (vec!["à", "la", "gorge"], vec!["a", "la", "gorge"]),
            (
                vec![vec![0], vec![1], vec![2]],
                vec![vec![0], vec![1], vec![2]],
            ),
        ),
    ];
    for (input, expected) in testcases {
        assert_eq!(get_alignments(&input.0, &input.1), expected);
    }
}
#[quickcheck]
fn randomcheck_get_original_spans_for_clean_text(tokens: Vec<String>) -> bool {
    let spans = get_span_indices(&tokens);
    let output = get_original_spans(&tokens, &join(&tokens))
        .iter()
        .map(|&x| x.unwrap())
        .collect::<Vec<_>>();
    spans == output
}

#[test]
fn test_get_original_spans() {
    let testcases = vec![
        (
            (vec!["fあo①が", "bar"], "fあo1かbar"),
            (vec![Some((0, 5)), Some((5, 8))]),
        ),
        ((vec!["New York"], "NewYork"), (vec![Some((0, 7))])),
        (
            (vec!["A'B", "", ""], "A B"),
            (vec![Some((0, 3)), Some((3, 3)), Some((3, 3))]),
        ),
        ((vec!["A'b", ""], "a b"), (vec![Some((0, 3)), Some((3, 3))])),
        (
            (vec!["", "", ""], ""),
            (vec![Some((0, 0)), Some((0, 0)), Some((0, 0))]),
        ),
        (
            (vec!["hello", "``world``"], "Hello \"world\""),
            vec![Some((0, 5)), Some((7, 12))],
        ),
        (
            (vec!["à", " ", "", "la", "gorge", ""], "a     lagorge"),
            (vec![
                Some((0, 1)),
                Some((1, 2)),
                Some((6, 6)),
                Some((6, 8)),
                Some((8, 13)),
                Some((13, 13)),
            ]),
        ),
    ];
    for (input, expected) in testcases.into_iter() {
        assert_eq!(
            get_original_spans(&input.0, input.1),
            expected,
            "{:?}",
            input
        );
    }
}

#[test]
fn test_get_char2token() {
    let testcases = vec![(vec!["a", "bc"], vec![0, 1, 1])];
    for (input, expected) in testcases.into_iter() {
        assert_eq!(get_char2token(&input), expected);
    }
}
#[test]
fn test_get_charmap() {
    let testcases = vec![
        ("å", "å", vec![Some(0)], vec![Some(0), Some(0)]),
        (
            "あがさ",
            "あかさ",
            vec![Some(0), Some(1), Some(2)],
            vec![Some(0), Some(1), Some(2)],
        ),
        ("", "a", vec![], vec![None]),
        ("", "", vec![], vec![]),
        (
            "å\tb",
            "a b",
            vec![Some(0), None, Some(2)],
            vec![Some(0), None, Some(2)],
        ),
        (
            "a\tb",
            "a b",
            vec![Some(0), None, Some(2)],
            vec![Some(0), None, Some(2)],
        ),
        (
            "２０００",
            "2000",
            vec![Some(0), Some(1), Some(2), Some(3)],
            vec![Some(0), Some(1), Some(2), Some(3)],
        ),
        ("¨", "", vec![None], vec![]),
        (
            "hello``world``",
            "Hello \"world\"",
            vec![
                Some(0),
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
                Some(7),
                Some(8),
                Some(9),
                Some(10),
                Some(11),
                None,
                None,
            ],
            vec![
                Some(0),
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
                Some(7),
                Some(8),
                Some(9),
                Some(10),
                Some(11),
                None,
            ],
        ),
    ];
    for (a, b, e_a2b, e_b2a) in testcases {
        let (a2b, b2a) = get_charmap(a, b);
        assert_eq!(a2b.len(), a.chars().count(), "a2b {:?}", a2b);
        assert_eq!(b2a.len(), b.chars().count(), "b2a {:?}", b2a);
        assert_eq!(a2b, e_a2b);
        assert_eq!(b2a, e_b2a);
    }
}
