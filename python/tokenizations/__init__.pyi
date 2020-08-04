from typing import Optional, Sequence, List, Tuple

def get_alignments(
    a: Sequence[str], b: Sequence[str]
) -> Tuple[List[List[int]], List[List[int]]]: ...
def get_charmap(a: str, b: str) -> Tuple[List[List[int]], List[List[int]]]: ...
def get_original_spans(
    tokens: Sequence[str], original_text: str
) -> List[Optional[Tuple[int, int]]]: ...

