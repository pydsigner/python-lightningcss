from pathlib import Path
import pytest
import lightningcss as lcss

TEST_DATA_DIR = Path(__file__).parent / "data"
MAIN_CSS_PATH = TEST_DATA_DIR / "main.css"
NORMALIZE_CSS_PATH = TEST_DATA_DIR / "normalize.css"
MISSING_CSS_PATH = TEST_DATA_DIR / "not-there.css"
INVALID_CSS_PATH = TEST_DATA_DIR / "invalid.css"


def test_minify():
    expected = "@layer normalize{*{box-sizing:border-box}}a{color:#00f}"

    result = lcss.bundle_css(str(MAIN_CSS_PATH))
    assert result == expected

def test_pretty_print():
    expected = "* {\n  box-sizing: border-box;\n}\n"

    result = lcss.bundle_css(str(NORMALIZE_CSS_PATH), minify=False)
    assert result == expected

def test_file_not_found():
    with pytest.raises(OSError):
        lcss.bundle_css(str(MISSING_CSS_PATH))

def test_parse_error():
    with pytest.raises(ValueError):
        lcss.bundle_css(str(INVALID_CSS_PATH))
