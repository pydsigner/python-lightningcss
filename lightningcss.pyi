"""
A python wrapper for the core CSS transformation functionality of lightningcss.
"""

from __future__ import annotations

from typing import NewType

ParserFlags = NewType('ParserFlags', int)


class Angle:
    value: float
    unit: str
    def __init__(self, value: float, unit: str) -> None: ...


class Ratio:
    numerator: float
    denominator: float
    def __init__(self, numerator: float, denominator: float) -> None: ...


class Resolution:
    value: float
    unit: str
    def __init__(self, value: float, unit: str) -> None: ...


class Time:
    value: float
    unit: str
    def __init__(self, value: float, unit: str) -> None: ...


class LengthValue:
    value: float
    unit: str
    def __init__(self, value: float, unit: str) -> None: ...


class CssColor:
    def __init__(self, css: str) -> None: ...
    def css(self) -> str: ...


class Url:
    url: str
    def __init__(self, url: str) -> None: ...


class CustomIdent:
    ident: str
    def __init__(self, ident: str) -> None: ...


class DashedIdent:
    ident: str
    def __init__(self, ident: str) -> None: ...


class Function:
    name: str
    arguments: str
    def __init__(self, name: str, arguments: str) -> None: ...


class Variable:
    name: str
    fallback: str | None
    def __init__(self, name: str, fallback: str | None = None) -> None: ...


class EnvironmentVariable:
    name: str
    indices: list[int]
    fallback: str | None
    def __init__(
        self,
        name: str,
        indices: list[int] = [],
        fallback: str | None = None,
    ) -> None: ...


class Image:
    css: str
    def __init__(self, css: str) -> None: ...


class Selector:
    css: str
    def __init__(self, css: str) -> None: ...


class MediaQuery:
    css: str
    def __init__(self, css: str) -> None: ...


class SupportsCondition:
    css: str
    def __init__(self, css: str) -> None: ...


class CssRule:
    css: str
    def __init__(self, css: str) -> None: ...


class Property:
    css: str
    def __init__(self, css: str) -> None: ...


class TokenOrValue:
    css: str
    def __init__(self, css: str) -> None: ...


class Visitor:
    """
    Base class for visitors. To create a visitor, subclass this and override
    the visit methods for the node types you want to observe or mutate. Then,
    pass an instance of your visitor to `process_stylesheet()` or
    `bundle_css()`. The visitor will be called for each visited node in the
    stylesheet, allowing for observation or mutation of the node.
    """
    def __init__(self, visit_types: int = 0) -> None: ...

    def visit_url(self, value: Url) -> Url | None: ...
    def visit_color(self, value: CssColor) -> CssColor | None: ...
    def visit_length(self, value: LengthValue) -> LengthValue | None: ...
    def visit_angle(self, value: Angle) -> Angle | None: ...
    def visit_ratio(self, value: Ratio) -> Ratio | None: ...
    def visit_resolution(self, value: Resolution) -> Resolution | None: ...
    def visit_time(self, value: Time) -> Time | None: ...
    def visit_custom_ident(self, value: CustomIdent) -> CustomIdent | None: ...
    def visit_dashed_ident(self, value: DashedIdent) -> DashedIdent | None: ...

    def visit_selector(self, value: Selector) -> object | None: ...
    def visit_rule(self, value: CssRule) -> object | None: ...
    def visit_property(self, value: Property) -> object | None: ...
    def visit_image(self, value: Image) -> object | None: ...
    def visit_variable(self, value: Variable) -> object | None: ...
    def visit_environment_variable(self, value: EnvironmentVariable) -> object | None: ...
    def visit_media_query(self, value: MediaQuery) -> object | None: ...
    def visit_supports_condition(self, value: SupportsCondition) -> object | None: ...
    def visit_function(self, value: Function) -> object | None: ...
    def visit_token(self, value: TokenOrValue) -> object | None: ...


VISIT_RULES: int
VISIT_PROPERTIES: int
VISIT_URLS: int
VISIT_COLORS: int
VISIT_IMAGES: int
VISIT_LENGTHS: int
VISIT_ANGLES: int
VISIT_RATIOS: int
VISIT_RESOLUTIONS: int
VISIT_TIMES: int
VISIT_CUSTOM_IDENTS: int
VISIT_DASHED_IDENTS: int
VISIT_VARIABLES: int
VISIT_ENVIRONMENT_VARIABLES: int
VISIT_MEDIA_QUERIES: int
VISIT_SUPPORTS_CONDITIONS: int
VISIT_SELECTORS: int
VISIT_FUNCTIONS: int
VISIT_TOKENS: int


def calc_parser_flags(
    nesting: bool = False,
    custom_media: bool = False,
    deep_selector_combinator: bool = False,
) -> ParserFlags:
    """
    Calculates the `parser_flags` argument of `process_stylesheet()` and `bundle_css()`.
    """


def process_stylesheet(
    code: str,
    /,
    filename: str = "",
    error_recovery: bool = False,
    parser_flags: ParserFlags = ParserFlags(0),
    unused_symbols: set[str] | None = None,
    browsers_list: list[str] | None = None,
    minify: bool = True,
    visitor: Visitor | None = None,
) -> str:
    """
    Processes the supplied CSS stylesheet and returns as a string.

    :param code: A string containing a CSS stylesheet.
    :param filename: Optional filename to be used in parser error messages.
    :param error_recovery: Whether or not to omit broken CSS rather than
        producing a parse error. Enable with caution!
    :param parser_flags: An optional flag created by `calc_parser_flags()`.
        See that function for more details.
    :param unused_symbols: An optional set of known unused symbols, like
        classnames, ids, or keyframe names, to be removed from the output.
        Note that symbols should be specified in bare form, i.e.
        `unused_symbols={'a', 'b'}`, not `unused_symbols={'.a', '#b'}`, and
        will remove both ids and classes if they share a name. Use with caution!
    :param browsers_list: An optional list of browserslist targets to be used
        to determine automatic prefixing and transpilation. If it is not
        specified, no prefixing/transpilation will occur.
    :param minify: If True, the final output will be minified. Otherwise, it
        will be pretty-printed.
    :param visitor: An optional Visitor instance. If provided, the visitor will
        be called for each visited node in the stylesheet, allowing for
        observation or mutation of the node. See the Visitor class for more
        details.
    :return: A string containing a processed CSS stylesheet.
    """


def bundle_css(
    path: str,
    /,
    error_recovery: bool = False,
    parser_flags: ParserFlags = ParserFlags(0),
    unused_symbols: set[str] | None = None,
    browsers_list: list[str] | None = None,
    minify: bool = True,
    visitor: Visitor | None = None,
) -> str:
    """
    Processes the supplied CSS stylesheet file and returns the bundle as a string.

    Resolves all `@import` rules to create a single CSS bundle. The resources
    referenced via `@import` are resolved relative to the main file.

    :param path: A string containing the path of the stylesheet file to process.
    :param error_recovery: Whether or not to omit broken CSS rather than
        producing a parse error. Enable with caution!
    :param parser_flags: An optional flag created by `calc_parser_flags()`.
        See that function for more details.
    :param unused_symbols: An optional set of known unused symbols, like
        classnames, ids, or keyframe names, to be removed from the output.
        Note that symbols should be specified in bare form, i.e.
        `unused_symbols={'a', 'b'}`, not `unused_symbols={'.a', '#b'}`, and
        will remove both ids and classes if they share a name. Use with caution!
    :param browsers_list: An optional list of browserslist targets to be used
        to determine automatic prefixing and transpilation. If it is not
        specified, no prefixing/transpilation will occur.
    :param minify: If True, the final output will be minified. Otherwise, it
        will be pretty-printed.
    :param visitor: An optional Visitor instance. If provided, the visitor will
        be called for each visited node in the stylesheet, allowing for
        observation or mutation of the node. See the Visitor class for more
        details.
    :return: A string containing the CSS bundle.
    """
