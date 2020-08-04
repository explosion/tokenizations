#![deny(warnings)]
//! Tokenizations alignment functions.
#[cfg(test)]
mod tests;
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate quickcheck_macros;
extern crate seqdiff;
extern crate unicode_normalization;
use seqdiff::Diff;
use unicode_normalization::UnicodeNormalization;

pub type Alignment = Vec<Vec<usize>>;
pub type CharMap = Vec<Vec<usize>>;

fn normalize(text: &str) -> String {
    text.to_lowercase().nfkd().collect()
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
    let (a2b, b2a) = seqdiff::diff(
        &a.join("").chars().collect::<Vec<_>>(),
        &b.join("").chars().collect::<Vec<_>>(),
    );
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
/// assert_eq!(c_a2b, vec![vec![0], vec![1], vec![2]]);
/// assert_eq!(c_b2a, vec![vec![0], vec![1], vec![2]]);
/// ```
pub fn get_charmap(a: &str, b: &str) -> (CharMap, CharMap) {
    let at: Vec<String> = a.chars().map(|x| x.to_string()).collect();
    let bt: Vec<String> = b.chars().map(|x| x.to_string()).collect();
    get_alignments(&at, &bt)
}

// Deprecated functions:

fn _get_charmap(a: &str, b: &str) -> (Diff, Diff) {
    let at: Vec<String> = a.chars().map(|x| x.to_string()).collect();
    let bt: Vec<String> = b.chars().map(|x| x.to_string()).collect();
    let (a2b, b2a) = get_alignments(&at, &bt);
    let c_a2b: Diff = a2b.into_iter().map(|x| x.into_iter().next()).collect();
    let c_b2a: Diff = b2a.into_iter().map(|x| x.into_iter().next()).collect();
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

#[deprecated(since = "0.5.0", note = "please use `textspan::align_spans` instead")]
pub fn get_original_spans<S: AsRef<str>>(
    tokens: &[S],
    original_text: &str,
) -> Vec<Option<(usize, usize)>> {
    let spans = get_span_indices(tokens);
    let text = join(tokens);
    let (a2b, b2a) = _get_charmap(&text, original_text);

    let mut ret = vec![];
    for (l, r) in spans {
        // get the leftmost corresponding char
        let mut origl = None;
        for &x in a2b[l..r].iter() {
            if x != None {
                origl = x;
                break;
            }
        }
        // get the rightmost corresponding char
        let mut origr = None;
        for x in a2b[l..r].iter().rev() {
            if let Some(j) = x {
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
