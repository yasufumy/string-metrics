import pytest

from text_string_metrics import levenshtein


@pytest.mark.parametrize(
    ("x", "y", "expected"),
    [
        ("", "", 0),
        ("python", "python", 0),
        ("python", "", 6),
        ("", "python", 6),
        ("rust", "cargo", 5),
        ("cargo", "rust", 5),
    ],
)
def test_levenshtein(x: str, y: str, expected: int) -> None:
    assert levenshtein(x, y) == expected
