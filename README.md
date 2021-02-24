# Robust and Fast tokenizations alignment library for Rust and Python
[![creates.io](https://img.shields.io/crates/v/tokenizations.svg)](https://crates.io/crates/tokenizations)
[![pypi](https://img.shields.io/pypi/v/pytokenizations.svg)](https://pypi.org/project/pytokenizations/)
[![Actions Status](https://github.com/explosion/tokenizations/workflows/Test/badge.svg)](https://github.com/explosion/tokenizations/actions)

![sample](./img/demo.png)

Demo: [demo](https://tamuhey.github.io/tokenizations/)  
Rust document: [docs.rs](https://docs.rs/tokenizations)  
Blog post: [How to calculate the alignment between BERT and spaCy tokens effectively and robustly](https://gist.github.com/tamuhey/af6cbb44a703423556c32798e1e1b704)

## Usage (Python)

- Installation

```bash
$ pip install -U pip # update pip
$ pip install pytokenizations
```

- Install from source

This library uses [maturin](https://github.com/PyO3/maturin) to build the wheel.

```console
$ git clone https://github.com/tamuhey/tokenizations
$ cd tokenizations/python
$ pip install maturin
$ maturin build
```

Now the wheel is created in `python/target/wheels` directory, and you can install it with `pip install *whl`.

### `get_alignments`

```python
def get_alignments(a: Sequence[str], b: Sequence[str]) -> Tuple[List[List[int]], List[List[int]]]: ...
```

Returns alignment mappings for two different tokenizations:

```python
>>> tokens_a = ["å", "BC"]
>>> tokens_b = ["abc"] # the accent is dropped (å -> a) and the letters are lowercased(BC -> bc)
>>> a2b, b2a = tokenizations.get_alignments(tokens_a, tokens_b)
>>> print(a2b)
[[0], [0]]
>>> print(b2a)
[[0, 1]]
```

`a2b[i]` is a list representing the alignment from `tokens_a` to `tokens_b`.   

## Usage (Rust)

See here: [docs.rs](https://docs.rs/tokenizations)  

## Related

- [Algorithm overview](./note/algorithm.md)  
- [Blog post](./note/blog_post.md)  
- [seqdiff](https://github.com/tamuhey/seqdiff) is used for the diff process.
- [textspan](https://github.com/tamuhey/textspan)
- [explosion/spacy-alignments: 💫 A spaCy package for Yohei Tamura's Rust tokenizations library](https://github.com/explosion/spacy-alignments)
  - Python bindings for this library, maintained by Explosion, author of spaCy. If you feel difficult to install pytokenizations, please try this.
