"""Benchmark"""
import tokenizations
import pytest


@pytest.mark.benchmark(warmup=True, group="short", disable_gc=True)
def test_short(benchmark):
    args = ["今日は", "\t", "いい", "天気だ", "。"], ["今日", "は", "いい", "天気", "た", "。"]
    benchmark(tokenizations.get_alignments, *args)


@pytest.mark.benchmark(warmup=True, group="long", disable_gc=True)
def test_long(benchmark):
    a = list("abcde") * 1000
    b = list("abbde") * 1000
    benchmark(tokenizations.get_alignments, a, b)
