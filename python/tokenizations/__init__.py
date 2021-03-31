import warnings
from .tokenizations import (
    get_alignments,
    get_charmap,
    __version__,
)


def get_original_spans(tokens, original_text):
    raise ValueError(
        f"{get_original_spans.__name__} was deprecated. Please use `textspan.get_original_spans` instead."
    )


__all__ = ["get_charmap", "get_alignments", "get_original_spans", "__version__"]
