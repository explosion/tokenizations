# Tokenizations alignment library for Rust and Python, written in Rust

## Installation

```bash
$ pip install pytokenizations
```

## Usage

Get an alignment map for two different tokenizations:

```python
import tokenizations
a = ["New York"]
b = ["New", "York"]
a2b = [[0, 1]]
b2a = [[0], [0]]
assert tokenizations.get_alignments(a, b) == (a2b, b2a)
```

`a2b[i]` is a list representing the alignment from `a` to `b`.   
You can get the alignments for "dirty" tokens:

```python
a = ["げん", "ご"]
b = ["けんこ"] # all accents are dropped (が -> か, ご -> こ)
a2b = [[0], [0]]
b2a = [[0, 1]]
assert tokenizations.get_alignments(a, b) == (a2b, b2a)
```

## Algorithm

Let $A = a_{11}a_{12}..a_{1k_1},a_{21}..a_{Nk_N}$ and $B = b_{11}b_{12}..b_{1l_1},b_{21}..b_{Ml_M}$ be tokens of length N and M respectively. Each token $A_i$ in $A$ and $B_j$ in $B$ have length $k_i$ and $l_j$ respectively.
The *alignment* $AL_{AB}$ of $A$ to $B$ is such that $ \forall j \in AL_{AB,i} => B_j \cap A_i $. ($t \cap s$ means t partially matches s.)
For example, $a=["f","o","o"], b=["fo","o"] => AL_{AB} = [[1],[1],[2]], AL_{BA} = [[1, 2], [3]]$.
The goal of this algorithm is to find such $AL_{AB}$ and $AL_{BA}$

1. Normalize tokens in the unicode normalization form "NFKD"
2. Concatenate all tokens $A$ and $B$ to generate $TA$ and $TB$ respectively
3. Calculate shortest path on edit graph of $TA$ and $TB$
4. Get character mapping $C_{AB}$ and $C_{BA}$ from the edit graph
5. Get $AL_{AB}$ and $AL_{BA}$ from the character alignments $C_{AB}$ and $C_{BA}$

Details:

1. Normalize tokens in the unicode normalization form "NFKD"

To compare the token positions, we must compare each characters in tokens. Because the two tokenizations may be partially different, we normalize them in "NFKD" first.

2. Concatenate all tokens $A$ and $B$ to generate $TA$ and $TB$ respectively 
 
Before calculating the edit graph, we combine tokens into text. For example, if we have tokens `["Foo", "bar"]`, we concatenate them into one text `Foobar`. 

3. Calculate shortest path on edit graph from $TA$ and $TB$

We calculate the shortest path on edit graph from texts $TA$ and $TB$ to get character map between them.  The path can be calculated, for example, by [Myers' algorighm](http://www.xmailserver.org/diff2.pdf)

4. Get character alignments $C_{AB}$ and $C_{BA}$ from the edit graph

Let $TA_i$ and $TB_j$ be the i-th and j-th character in the text $TA$ and $TB$, respectively. $C_{AB}$ is a mapping from $TA$ to $TB$ such that $C_{AB},i \neq -1 \land C_{AB,i} = j \Rightarrow TA_i = TA_j$. For example, $TA = f0o, TB = fboo$ then $C_{AB} = [1,-1,3], C_{BA} = [1,-1,3,-1]$.
We can calculate $C_{AB}$ and $C_{BA}$ from the shortest path on the edit graph. If there exists diagonal edge $(i-1,j-1) -> (i, j)$ in the path, $C_{AB,i} = j$ and $C_{BA,j} = i$. If there doesn't exist any diagonal edge to $\forall j (i, j)$ then $C_{AB,i} = -1$.

5. Get $AL_{AB}$ and $AL_{BA}$ from the character alignments $C_{AB}$ and $C_{BA}$

Now we can calculate the desired $AL_{AB}$ and $AL_{BA}$ from the previous calculated $C_{AB}$ and $C_{BA}$. 
