#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

extern crate unicode_normalization;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

fn normalize(text: &str) -> String {
    text.nfkd().collect()
}

type Point = (usize, usize);

#[cfg(test)]
struct EditPathFromGrid {
    d: Vec<Vec<usize>>,
    cur: Point,
    size: (usize, usize),
    exhausted: bool,
}

#[cfg(test)]
impl Iterator for EditPathFromGrid {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        if self.cur == (0, 0) {
            self.exhausted = true;
            return Some((0, 0));
        }
        let (i, j) = self.cur;
        let mut ncur = (0, 0);
        if i > 0 && j > 0 {
            ncur = *[(i, j - 1), (i - 1, j)]
                .iter()
                .min_by_key(|x| self.d[x.0][x.1])
                .unwrap();
            let ul = self.d[i - 1][j - 1];
            if self.d[ncur.0][ncur.1] > ul && self.d[i][j] == ul {
                ncur = (i - 1, j - 1)
            }
        } else if i > 0 {
            ncur = (i - 1, j);
        } else {
            ncur = (i, j - 1);
        }
        self.cur = ncur;
        Some((i, j))
    }
}

#[cfg(test)]
fn get_shortest_edit_path_dp(a: &str, b: &str) -> EditPathFromGrid {
    let n = a.chars().count();
    let m = b.chars().count();
    let mut d = vec![vec![std::usize::MAX; m + 1]; n + 1];
    d[0] = (0..(m + 1)).collect();
    for i in 0..(n + 1) {
        d[i][0] = i;
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
        d: d,
        cur: (n, m),
        size: (n + 1, m + 1),
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

/// Calculate shotest edit path based on Myers' diff algorithm.
/// See [An O(ND) Difference Algorithm and Its Variations](http://www.xmailserver.org/diff2.pdf)
fn get_shortest_edit_path_myers(a: &str, b: &str) -> EditPathFromHashMap {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let n = a.len();
    let m = b.len();
    let bound = n + m;
    let get_y = |x, k| x + bound - k;
    let mut v = vec![0; 2 * bound + 1];
    let mut nodes_map = HashMap::new();
    'outer: for d in 0..(bound + 1) {
        for k in ((bound - d)..(bound + d + 1)).step_by(2) {
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
                x = x + 1;
                y = y + 1;
            }
            v[k] = x;
            if x >= n && y >= m {
                break 'outer;
            }
        }
    }

    EditPathFromHashMap {
        nodes_map: nodes_map,
        cur: Node::P((n, m)),
    }
}

fn path_to_charmap(
    mut path: impl Iterator<Item = (usize, usize)>,
) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[quickcheck]
    fn randomcheck_myers_with_dp(a: String, b: String) {
        let v = path_to_charmap(get_shortest_edit_path_dp(&a, &b));
        let w = path_to_charmap(get_shortest_edit_path_myers(&a, &b));
        assert_eq!(v, w)
    }

    #[test]
    fn test_get_charmap() {
        let testcases = vec![
            (
                "あがさ",
                "あかさ",
                vec![Some(0), Some(1), None, Some(2)],
                vec![Some(0), Some(1), Some(3)],
            ),
            ("", "a", vec![], vec![None]),
            ("", "", vec![], vec![]),
            (
                "å\tb",
                "a b",
                vec![Some(0), None, None, Some(2)],
                vec![Some(0), None, Some(3)],
            ),
            (
                "a\tb",
                "a b",
                vec![Some(0), None, Some(2)],
                vec![Some(0), None, Some(2)],
            ),
        ];
        for (a, b, e_a2b, e_b2a) in testcases {
            let a = normalize(a);
            let b = normalize(b);
            let path = get_shortest_edit_path_myers(&a, &b);
            let (a2b, b2a) = path_to_charmap(path);
            assert_eq!(a2b, e_a2b);
            assert_eq!(b2a, e_b2a);
        }
    }
}
