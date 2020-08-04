import pytest
import tokenizations
from hypothesis import given
from hypothesis import strategies as st


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
def test_get_alignments(input_, expected):
    output = tokenizations.get_alignments(*input_)
    assert output == expected


@pytest.mark.parametrize(
    "input_,expected", [(("foo", "fo0"), ([[0], [1], []], [[0], [1], []]))]
)
def test_get_charmap(input_, expected):
    assert tokenizations.get_charmap(*input_) == expected


@given(st.text(), st.text())
def test_random_charmap(a, b):
    tokenizations.get_charmap(a, b)


@given(st.text())
def test_equality_charmap(a):
    a2b, b2a = tokenizations.get_charmap(a, a)
    assert a2b == b2a
    assert a2b == [[x] for x in range(len(a))]


@given(st.lists(st.text()), st.text())
def test_random_get_original_spans(tokens, text):
    tokenizations.get_original_spans(tokens, text)
    ret = tokenizations.get_original_spans(tokens, "".join(tokens))
    assert all(x is not None for x in ret)


@pytest.mark.parametrize(
    "tokens,text,expected",
    [
        (["Hello", "world"], "Hello world", [(0, 5), (6, 11)]),
        (["hello", "``world``"], 'Hello "world"', [(0, 5), (7, 12)]),
    ],
)
def test_random_get_original_spans(tokens, text, expected):
    ret = tokenizations.get_original_spans(tokens, text)
    assert ret == expected, (tokens, text)
