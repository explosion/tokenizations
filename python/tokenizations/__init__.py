import warnings
from .tokenizations import (
    get_alignments,
    get_charmap,
    get_original_spans as _get_original_spans,
    __version__,
)


def get_original_spans(tokens, original_text):
    warnings.warn(
        "get_original_spans is deprecated, use `textspan.get_original_spans` instead.",
        DeprecationWarning,
    )
    return _get_original_spans(tokens, original_text)


__all__ = ["get_charmap", "get_alignments", "get_original_spans", "__version__"]
