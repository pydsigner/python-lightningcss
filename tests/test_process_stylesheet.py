import lightningcss as lcss
import pytest


def test_minify():
    code = ".content { margin-inline: 3vw; padding: 1rem 2rem 1rem 2rem; }"
    expected = ".content{margin-inline:3vw;padding:1rem 2rem}"

    result = lcss.process_stylesheet(code)
    assert result == expected

def test_pretty_print():
    code = ".content { margin: 3vw; }"
    expected = ".content {\n  margin: 3vw;\n}\n"

    result = lcss.process_stylesheet(code, minify=False)
    assert result == expected

def test_parse_error():
    code = "img { display: block; } nonsense }{ non-terminated"

    with pytest.raises(ValueError):
        lcss.process_stylesheet(code)

def test_error_recovery():
    code = "img { display: block; } nonsense }{ non-terminated"
    expected = "img{display:block}"

    with pytest.raises(ValueError):
        lcss.process_stylesheet(code, error_recovery=False)

    result = lcss.process_stylesheet(code, error_recovery=True)
    assert result == expected
