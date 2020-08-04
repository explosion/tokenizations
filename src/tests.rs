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
        ("å", "å", vec![vec![0, 1]], vec![vec![0], vec![0]]),
        (
            "あがさ",
            "あかさ",
            vec![vec![0], vec![1], vec![2]],
            vec![vec![0], vec![1], vec![2]],
        ),
        ("", "a", vec![], vec![vec![]]),
        ("", "", vec![], vec![]),
        (
            "å\tb",
            "a b",
            vec![vec![0], vec![], vec![2]],
            vec![vec![0], vec![], vec![2]],
        ),
        (
            "a\tb",
            "a b",
            vec![vec![0], vec![], vec![2]],
            vec![vec![0], vec![], vec![2]],
        ),
        (
            "２０００",
            "2000",
            vec![vec![0], vec![1], vec![2], vec![3]],
            vec![vec![0], vec![1], vec![2], vec![3]],
        ),
        ("¨", "", vec![vec![]], vec![]),
        (
            "hello``world``",
            "Hello \"world\"",
            vec![
                vec![0],
                vec![1],
                vec![2],
                vec![3],
                vec![4],
                vec![],
                vec![],
                vec![7],
                vec![8],
                vec![9],
                vec![10],
                vec![11],
                vec![],
                vec![],
            ],
            vec![
                vec![0],
                vec![1],
                vec![2],
                vec![3],
                vec![4],
                vec![],
                vec![],
                vec![7],
                vec![8],
                vec![9],
                vec![10],
                vec![11],
                vec![],
            ],
        ),
    ];
    for (a, b, e_a2b, e_b2a) in testcases {
        let (a2b, b2a) = get_charmap(a, b);
        assert_eq!(a2b.len(), a.chars().count(), "a2b {:?}", a2b);
        assert_eq!(b2a.len(), b.chars().count(), "b2a {:?}", b2a);
        assert_eq!(
            a2b, e_a2b,
            "check a2b
             a: {:?}
             b: {:?}
        ",
            a, b
        );
        assert_eq!(
            b2a, e_b2a,
            "check b2a
             a: {:?}
             b: {:?}
        ",
            a, b
        );
    }
}
