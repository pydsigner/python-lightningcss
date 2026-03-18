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


def test_bundle_css_visitor_multiple_mutations(tmp_path: Path):
    main_css = tmp_path / "main.css"
    imported_css = tmp_path / "normalize.css"

    imported_css.write_text(".content{margin-top:10px}")
    main_css.write_text(
        "@import \"normalize.css\";a{color:red;background-image:url('a.png')}"
    )

    class MutatingVisitor(lcss.Visitor):
        def __init__(self):
            super().__init__(
                lcss.VISIT_URLS | lcss.VISIT_LENGTHS | lcss.VISIT_COLORS
            )

        def visit_url(self, value: lcss.Url):
            return lcss.Url("b.png")

        def visit_length(self, value: lcss.LengthValue):
            return lcss.LengthValue(value.value + 5, value.unit)

        def visit_color(self, value: lcss.CssColor):
            return lcss.CssColor("#00ff00")

    result = lcss.bundle_css(str(main_css), visitor=MutatingVisitor())

    assert "url(b.png)" in result
    assert "margin-top:15px" in result
    assert "#0f0" in result


def test_bundle_css_visitor_rule_observation_dispatch(tmp_path: Path):
    main_css = tmp_path / "main.css"
    main_css.write_text("a{color:red}")

    seen_rules: list[str] = []

    class ObservingVisitor(lcss.Visitor):
        def __init__(self):
            super().__init__(lcss.VISIT_RULES)

        def visit_rule(self, value: lcss.CssRule):
            seen_rules.append(value.css)
            return lcss.CssRule(".ignored{color:blue}")

    result = lcss.bundle_css(str(main_css), visitor=ObservingVisitor())

    assert result == "a{color:red}"
    assert len(seen_rules) == 1
    assert "a" in seen_rules[0]
