# Robust and Fast tokenizations alignment library for Rust and Python
[![creates.io](https://img.shields.io/crates/v/tokenizations.svg)](https://crates.io/crates/tokenizations)
[![pypi](https://img.shields.io/pypi/v/pytokenizations.svg)](https://pypi.org/project/pytokenizations/)
[![Actions Status](https://github.com/tamuhey/tokenizations/workflows/Test/badge.svg)](https://github.com/tamuhey/tokenizations/actions)

![sample](./img/demo.png)

Demo: [demo](https://tamuhey.github.io/tokenizations/)  
Rust document: [docs.rs](https://docs.rs/tokenizations/0.2.2/tokenizations/)  
Python document: [python/README.md](./python/README.md)  
Blog post: [How to calculate the alignment between BERT and spaCy tokens effectively and robustly](https://gist.github.com/tamuhey/af6cbb44a703423556c32798e1e1b704)

## Overview

Get an alignment map for two different and *noisy* tokenizations:

```python
>>> tokens_a = ["げん", "ご"]
>>> tokens_b = ["けんこ"] # all accents are dropped (が -> か, ご -> こ)
>>> a2b, b2a = tokenizations.get_alignments(tokens_a, tokens_b)
>>> print(a2b)
[[0], [0]]
>>> print(b2a)
[[0, 1]]
```

`a2b[i]` is tokens_a list representing the alignment from `tokens_a` to `tokens_b`.   

## Algorithm

- [Algorithm overview](./note/algorithm.md)  
- [Blog post](./note/blog_post.md)  
