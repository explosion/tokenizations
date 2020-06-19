use crate::*;
#[cfg(test)]
struct EditPathFromGrid {
    // This struct is only for testing
    // Inefficient but simple algorithm
    d: Vec<Vec<usize>>,
    cur: Point,
    exhausted: bool,
}

#[cfg(test)]
impl Iterator for EditPathFromGrid {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        if self.cur == (0, 0) {
            self.exhausted = true;
            return Some((0, 0));
        }
        let (i, j) = self.cur;
        let ncur = if i > 0 && j > 0 {
            let ncur = *[(i, j - 1), (i - 1, j)]
                .iter()
                .min_by_key(|x| self.d[x.0][x.1])
                .unwrap();
            let ul = self.d[i - 1][j - 1];
            if self.d[ncur.0][ncur.1] > ul && self.d[i][j] == ul {
                (i - 1, j - 1)
            } else {
                ncur
            }
        } else if i > 0 {
            (i - 1, j)
        } else {
            (i, j - 1)
        };
        self.cur = ncur;
        Some((i, j))
    }
}

#[allow(clippy::many_single_char_names)]
#[cfg(test)]
fn get_shortest_edit_path_grid(a: &str, b: &str) -> EditPathFromGrid {
    let n = a.chars().count();
    let m = b.chars().count();
    let mut d = vec![vec![std::usize::MAX; m + 1]; n + 1];
    d[0] = (0..=m).collect();
    for (i, e) in d.iter_mut().enumerate().take(n + 1) {
        e[0] = i;
    }
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            if ca == cb {
                d[i + 1][j + 1] = d[i][j];
            } else {
                let mut u = (i + 1, j);
                let l = (i, j + 1);
                if d[u.0][u.1] > d[l.0][l.1] {
                    u = l;
                }
                d[i + 1][j + 1] = d[u.0][u.1] + 1;
            }
        }
    }

    EditPathFromGrid {
        d,
        cur: (n, m),
        exhausted: false,
    }
}

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
fn randomcheck_myers_with_dp(a: String, b: String) {
    let v = path_to_charmap(get_shortest_edit_path_grid(&a, &b));
    let w = path_to_charmap(get_shortest_edit_path_myers(&a, &b));
    assert_eq!(v, w)
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
