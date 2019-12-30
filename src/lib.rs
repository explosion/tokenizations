extern crate unicode_normalization;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

fn normalize(text: &str) -> String {
    text.nfkd().collect()
}

struct EditPathFromGrid {
    grid: Vec<Vec<(usize, usize)>>,
    cur: (usize, usize),
    exhausted: bool,
}
impl EditPathFromGrid {
    fn new(grid: Vec<Vec<(usize, usize)>>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        EditPathFromGrid {
            grid: grid,
            cur: (n - 1, m - 1),
            exhausted: false,
        }
    }
}

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
        let prev = self.cur;
        let next = self.grid[self.cur.0][self.cur.1];
        self.cur = next;
        Some(prev)
    }
}

fn get_shortest_edit_path_dp(a: &str, b: &str) -> EditPathFromGrid {
    let n = a.chars().count();
    let m = b.chars().count();
    let mut d = vec![vec![std::usize::MAX; m + 1]; n + 1];
    d[0] = (0..(m + 1)).collect();
    for i in 0..(n + 1) {
        d[i][0] = i;
    }
    let mut prev = vec![vec![(std::usize::MAX, std::usize::MAX); m + 1]; n + 1];
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            if ca == cb {
                d[i + 1][j + 1] = d[i][j];
                prev[i + 1][j + 1] = (i, j);
            } else {
                let mut u = (i + 1, j);
                let l = (i, j + 1);
                if d[u.0][u.1] > d[l.0][l.1] {
                    u = l;
                }
                d[i + 1][j + 1] = d[u.0][u.1] + 1;
                prev[i + 1][j + 1] = u;
            }
        }
    }
    EditPathFromGrid::new(prev)
}

struct EditPathFromHashMap {
    nodes_map: HashMap<(usize, usize), (usize, usize)>,
    cur: (usize, usize),
}

impl<'a> Iterator for EditPathFromHashMap {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == (0, 0) {
            return None;
        }
        match self.nodes_map.get(&self.cur) {
            Some(x) => {
                self.cur = *x;
                Some(*x)
            }
            None => None,
        }
    }
}

/// Myers' diff algorithm. See [An O(ND) Difference Algorithm and Its Variations](http://www.xmailserver.org/diff2.pdf)
fn get_shortest_edit_path_myers(a: &str, b: &str) -> EditPathFromHashMap {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let n = a.len();
    let m = b.len();
    let bound = n + m;
    let get_y = |x, k| x + bound - k;
    let mut v = vec![0; 2 * bound + 1];
    let mut nodes_map = HashMap::new();
    'outer: for d in 0..bound {
        for k in ((bound - d)..(bound + d + 1)).step_by(2) {
            let (mut x, px, py) = if d == 0 {
                (0, 0, 0)
            } else if k == (bound - d) || k != (bound + d) && v[k - 1] < v[k + 1] {
                let px = v[k + 1];
                println!("{:?}", (bound, k, px, &a, &b));
                (px, px, get_y(px, k + 1))
            } else {
                let px = v[k - 1] + 1;
                (px + 1, px, get_y(px, k - 1))
            };
            let mut y = get_y(x, k);
            nodes_map.insert((x, y), (px, py));
            while x < n && y < m && a[x] == b[y] {
                nodes_map.insert((x + 1, y + 1), (x, y));
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
        cur: (n, m),
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
    #[test]
    fn test_get_charmap() {
        let testcases = vec![
            ("a", "a", vec![Some(0)], vec![Some(0)]),
            (
                "あがさ",
                "あかさ",
                vec![Some(0), Some(1), None, Some(2)],
                vec![Some(0), Some(1), Some(3)],
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
            get_shortest_edit_path_myers(&a, &b);
            let path = get_shortest_edit_path_dp(&a, &b);
            let (a2b, b2a) = path_to_charmap(path);
            assert_eq!(a2b, e_a2b);
            assert_eq!(b2a, e_b2a);
        }
    }
}
