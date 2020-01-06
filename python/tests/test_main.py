import pytest
from hypothesis import strategies as st, given
import tokenizations


@given(st.lists(st.text()), st.lists(st.text()))
def test_random(a, b):
    tokenizations.get_alignments(a, b)


@given(st.lists(st.text()))
def test_equality(a):
    a2b, b2a = tokenizations.get_alignments(a, a)
    assert a2b == b2a
    assert a2b == [[i] if len(aa) else [] for i, aa in enumerate(a)]


@pytest.mark.parametrize(
    "input_,expected",
    [
        ((["fo", "o"], ["foo"]), ([[0], [0]], [[0, 1]])),
        ((["fø", "o"], ["foo"]), ([[0], [0]], [[0, 1]])),
        ((["New", "York"], ["New York"]), ([[0], [0]], [[0, 1]])),
        (
            (["今日は", "\t", "いい", "天気だ", "。"], ["今日", "は", "いい", "天気", "た", "。"]),
            ([[0, 1], [], [2], [3, 4], [5]], [[0], [0], [2], [3], [3], [4]]),
        ),
    ],
)
def test_equality(input_, expected):
    output = tokenizations.get_alignments(*input_)
    assert output == expected