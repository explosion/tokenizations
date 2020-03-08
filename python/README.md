# Installation

```bash
$ pip install pytokenizations
```

## Install from source

This library uses [maturin](https://github.com/PyO3/maturin) to build.

```
$ git clone https://github.com/tamuhey/tokenizations
$ cd python
$ pip install maturin
$ maturin build
```

Now wheel is built in `python/target/wheels` directory. You can install it with `pip install *whl`.

# Usage

Get an alignment map for two different tokenizations:

```python
import tokenizations
tokens_a = ["New York"]
tokens_b = ["New", "York"]
a2b = [[0, 1]]
b2a = [[0], [0]]
assert tokenizations.get_alignments(tokens_a, tokens_b) == (a2b, b2a)
```

`a2b[i]` is tokens_a list representing the alignment from `tokens_a` to `tokens_b`.   
You can get the alignments for "dirty" tokens:

```python
tokens_a = ["げん", "ご"]
tokens_b = ["けんこ"] # all accents are dropped (が -> か, ご -> こ)
a2b = [[0], [0]]
b2a = [[0, 1]]
assert tokenizations.get_alignments(tokens_a, tokens_b) == (a2b, b2a)
```
