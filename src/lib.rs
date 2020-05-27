#![deny(warnings)]
//! Tokenizations alignment functions.
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

extern crate unicode_normalization;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

pub type Alignment = Vec<Vec<usize>>;

fn normalize(text: &str) -> String {
    text.to_lowercase().nfkd().collect()
}

type Point = (usize, usize);

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Node {
    P(Point),
    Root,
}

struct EditPathFromHashMap {
    nodes_map: HashMap<Node, Node>,
    cur: Node,
}

impl<'a> Iterator for EditPathFromHashMap {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Node::Root => None,
            Node::P(cur) => {
                self.cur = *self.nodes_map.get(&Node::P(cur)).unwrap();
                Some(cur)
            }
        }
    }
}

/// Returns an iterator over the shotest path of the edit graph based on Myers' diff algorithm.
///
/// See [An O(ND) Difference Algorithm and Its Variations](http://www.xmailserver.org/diff2.pdf)
#[allow(clippy::many_single_char_names)]
fn get_shortest_edit_path_myers(a: &str, b: &str) -> EditPathFromHashMap {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let n = a.len();
    let m = b.len();
    let bound = n + m;
    let get_y = |x, k| x + bound - k;
    let mut v = vec![0; 2 * bound + 1];
    let mut nodes_map = HashMap::new();
    'outer: for d in 0..=bound {
        for k in ((bound - d)..=bound + d).step_by(2) {
            let (mut x, parent) = if d == 0 {
                (0, Node::Root)
            } else if k == (bound - d) || k != (bound + d) && v[k - 1] < v[k + 1] {
                let px = v[k + 1];
                (px, Node::P((px, get_y(px, k + 1))))
            } else {
                let px = v[k - 1];
                (px + 1, Node::P((px, get_y(px, k - 1))))
            };
            let mut y = get_y(x, k);
            nodes_map.insert(Node::P((x, y)), parent);
            while x < n && y < m && a[x] == b[y] {
                nodes_map.insert(Node::P((x + 1, y + 1)), Node::P((x, y)));
                x += 1;
                y += 1;
            }
            v[k] = x;
            if x >= n && y >= m {
                break 'outer;
            }
        }
    }

    EditPathFromHashMap {
        nodes_map,
        cur: Node::P((n, m)),
    }
}

pub type CharMap = Vec<Option<usize>>;

fn path_to_charmap(mut path: impl Iterator<Item = (usize, usize)>) -> (CharMap, CharMap) {
    let (mut i, mut j) = path.next().unwrap();
    let mut a2b = vec![None; i];
    let mut b2a = vec![None; j];
    for (pi, pj) in path {
        if (i - pi) + (j - pj) == 2 {
            a2b[pi] = Some(pj);
            b2a[pj] = Some(pi);
        }
        i = pi;
        j = pj;
    }
    (a2b, b2a)
}

fn get_char2token<T: AsRef<str>>(tokens: &[T]) -> Vec<usize> {
    let token_lengths = tokens
        .iter()
        .map(|s| s.as_ref().chars().count())
        .collect::<Vec<_>>();
    let mut ret = vec![0; token_lengths.iter().sum()];
    let mut cur = 0;
    for (i, &l) in token_lengths.iter().enumerate() {
        for _ in 0..l {
            ret[cur] = i;
            cur += 1;
        }
    }
    ret
}

// Returns tokenization alignment from ta to tb.
fn get_alignment(
    num_tokens: usize,
    a2b: &[Option<usize>],
    ac2t: &[usize],
    bc2t: &[usize],
) -> Vec<Vec<usize>> {
    let mut at2bt = vec![vec![]; num_tokens];
    for (ti, a2bi) in ac2t.iter().zip(a2b) {
        if let Some(i) = a2bi {
            if let Some(j) = at2bt[*ti].last() {
                if *j == bc2t[*i] {
                    continue;
                }
            }
            at2bt[*ti].push(bc2t[*i])
        }
    }
    at2bt
}

/// Returns the tokenizations alignments `a2b` (from `a` to `b`) and `b2a` (from `b` to `a`) based on the shortest edit script (SES).
///
/// # Examples
///
/// ```
/// use tokenizations::get_alignments;
///
/// let a = vec!["New York"];
/// let b = vec!["New", "York"];
/// // calculate the two alignments `a2b` and `b2a` at the same time
/// let (a2b, b2a) = get_alignments(&a, &b);
///
/// // `a2b[i]` is a set that holds indices `j`s of `b` such that `a[i]` corresponds to `b[j]`
/// assert_eq!(a2b, vec![[0, 1]]);
/// // `b2a` is the inverse of `a2b`
/// assert_eq!(b2a, vec![[0], [0]]);
///
/// // `get_alignments` can be applied to noisy tokens.
/// let a = vec!["à", "la", "gorge"];
/// let b = vec!["a", "la", "gorge"]; // dropped accent
/// let (a2b, b2a) = get_alignments(&a, &b);
/// assert_eq!(a2b, vec![[0], [1], [2]]);
/// assert_eq!(a2b, vec![[0], [1], [2]]);
/// ```
pub fn get_alignments<S: AsRef<str>>(a: &[S], b: &[S]) -> (Alignment, Alignment) {
    let a: Vec<String> = a.iter().map(|x| normalize(x.as_ref())).collect();
    let b: Vec<String> = b.iter().map(|x| normalize(x.as_ref())).collect();
    let ac2t = get_char2token(&a);
    let bc2t = get_char2token(&b);
    let (a2b, b2a) = path_to_charmap(get_shortest_edit_path_myers(&a.join(""), &b.join("")));
    let at2bt = get_alignment(a.len(), &a2b, &ac2t, &bc2t);
    let bt2at = get_alignment(b.len(), &b2a, &bc2t, &ac2t);
    (at2bt, bt2at)
}

/// Returns the character mappings `c_a2b` (from `a` to `b`) and `c_b2a` (from `b` to `a`) based on the shortest edit script (SES).
///
/// `a` and `b` can be noisy. For example, `bar` and `bår` can be properly compared.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use tokenizations::get_charmap;
/// let a = "bar";
/// let b = "bår";
/// let (c_a2b, c_b2a) = get_charmap(a, b);
/// assert_eq!(c_a2b, vec![Some(0), Some(1), Some(2)]);
/// assert_eq!(c_b2a, vec![Some(0), Some(1), Some(2)]);
/// ```
pub fn get_charmap(a: &str, b: &str) -> (CharMap, CharMap) {
    let at: Vec<String> = a.chars().map(|x| x.to_string()).collect();
    let bt: Vec<String> = b.chars().map(|x| x.to_string()).collect();
    let (a2b, b2a) = get_alignments(&at, &bt);
    let c_a2b: CharMap = a2b.into_iter().map(|x| x.into_iter().next()).collect();
    let c_b2a: CharMap = b2a.into_iter().map(|x| x.into_iter().next()).collect();
    (c_a2b, c_b2a)
}

fn get_span_indices<S: AsRef<str>>(tokens: &[S]) -> Vec<(usize, usize)> {
    tokens
        .iter()
        .scan(0, |state, token| {
            let l = *state;
            let r = l + token.as_ref().chars().count();
            *state = r;
            Some((l, r))
        })
        .collect()
}

fn join<S: AsRef<str>>(tokens: &[S]) -> String {
    let mut text = "".to_owned();
    for token in tokens.iter() {
        text.push_str(token.as_ref());
    }
    text
}

/// Returns the span indices in original_text from the tokens based on the shortest edit script (SES).
/// This is useful, for example, when a processed result is mapped to the original text that is not normalized yet.
///
/// # Examples
///
/// ```
/// use tokenizations::get_original_spans;
/// let tokens = vec!["a", "la", "gorge"];
/// let original_text = "à  LA    gorge";
/// let spans = get_original_spans(&tokens, original_text);
/// assert_eq!(spans, vec![Some((0,1)), Some((3,5)), Some((9,14))]);
/// ```
pub fn get_original_spans<S: AsRef<str>>(
    tokens: &[S],
    original_text: &str,
) -> Vec<Option<(usize, usize)>> {
    let spans = get_span_indices(tokens);
    let text = join(tokens);
    let (a2b, b2a) = get_charmap(&text, original_text);

    let mut ret = vec![];
    for (l, r) in spans {
        // get the leftmost corresponding char
        let mut origl = None;
        for i in l..r {
            if a2b[i] != None {
                origl = a2b[i];
                break;
            }
        }
        // get the rightmost corresponding char
        let mut origr = None;
        for i in (l..r).rev() {
            if let Some(j) = a2b[i] {
                origr = Some(j + 1);
                break;
            }
        }
        // edge case: a token with empty string
        if l == r {
            if l >= a2b.len() {
                origl = Some(b2a.len());
            } else {
                origl = a2b[l];
            }
            origr = origl;
        }
        ret.push(match (origl, origr) {
            (Some(l), Some(r)) => Some((l, r)),
            (None, None) => None,
            _ => unreachable!(
                "Internal error occured in get_original_span\ntokens: {:?}\noriginal_text: {:?}",
                tokens.iter().map(|x| x.as_ref()).collect::<Vec<_>>(),
                original_text
            ),
        })
    }
    ret
}

#[cfg(test)]
mod tests {
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
        ];
        for (a, b, e_a2b, e_b2a) in testcases {
            let (a2b, b2a) = get_charmap(a, b);
            assert_eq!(a2b, e_a2b);
            assert_eq!(b2a, e_b2a);
        }
    }
}
