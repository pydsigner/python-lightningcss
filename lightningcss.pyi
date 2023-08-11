"""
A python wrapper for the core CSS transformation functionality of lightningcss.
"""

from typing import NewType

ParserFlags = NewType('ParserFlags', int)

def calc_parser_flags(
    nesting: bool = False,
    custom_media: bool = False,
    deep_selector_combinator: bool = False
) -> ParserFlags:
    """
    Calculates the `parser_flags` argument of `process_stylesheet()`.
    """

def process_stylesheet(
    code: str,
    /,
    filename: str = "",
    error_recovery: bool = False,
    parser_flags: ParserFlags = ParserFlags(0),
    unused_symbols: set[str] | None = None,
    browsers_list: list[str] | None = None,
    minify: bool = True
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
    :param minify: Is True, the final output will be minified. Otherwise, it
        will be pretty-printed.
    :return: A string containing a processed CSS stylesheet.
    """
