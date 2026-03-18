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


def test_visitor_method_dispatch_multiple_mutations():
    code = ".content{background-image:url('a.png');margin-top:10px;color:red}"

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

    result = lcss.process_stylesheet(code, visitor=MutatingVisitor())

    assert "url(b.png)" in result
    assert "margin-top:15px" in result
    assert "#0f0" in result


def test_visitor_rule_observation_dispatch():
    code = ".content{color:red}"
    seen_rules = []

    class ObservingVisitor(lcss.Visitor):
        def __init__(self):
            super().__init__(lcss.VISIT_RULES)

        def visit_rule(self, value: lcss.CssRule):
            seen_rules.append(value.css)
            return lcss.CssRule(".ignored{color:blue}")

    result = lcss.process_stylesheet(code, visitor=ObservingVisitor())

    assert result == ".content{color:red}"
    assert len(seen_rules) == 1
    assert ".content" in seen_rules[0]
