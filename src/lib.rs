extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;

fn normalize(text: &str) -> String {
    text.nfkd().collect()
}

fn get_charmap(a: &str, b: &str) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
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

    let mut a2b = vec![None; n];
    let mut b2a = vec![None; m];
    let mut cur = (n, m);
    loop {
        let parent = prev[cur.0][cur.1];
        if (cur.0 - parent.0) + (cur.1 - parent.1) == 2 {
            let (i, j) = parent;
            a2b[i] = Some(j);
            b2a[j] = Some(i);
        }
        cur = parent;
        if cur == (0, 0) {
            break;
        }
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
        ];
        for (a, b, e_a2b, e_b2a) in testcases {
            let a = normalize(a);
            let b = normalize(b);
            let (a2b, b2a) = get_charmap(&a, &b);
            assert_eq!(a2b, e_a2b);
            assert_eq!(b2a, e_b2a);
        }
    }
}
