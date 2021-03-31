from __future__ import annotations
from typing import Optional, Sequence, Tuple

def get_alignments(
    a: Sequence[str], b: Sequence[str]
) -> Tuple[list[list[int]], list[list[int]]]: ...
def get_charmap(a: str, b: str) -> Tuple[list[list[int]], list[list[int]]]: ...
def get_original_spans(
    tokens: Sequence[str], original_text: str
) -> list[Optional[Tuple[int, int]]]: ...

