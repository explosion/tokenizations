from typing import Optional, Sequence, List, Tuple

Align = List[List[int]]
CharAlign = List[Optional[int]]

def get_alignments(a: Sequence[str], b: Sequence[str]) -> Tuple[Align, Align]: ...
def get_charmap(a: str, b: str) -> Tuple[CharAlign, CharAlign]: ...
def get_original_spans(
    tokens: Sequence[str], original_text: str
) -> List[Optional[Tuple[int, int]]]: ...

