[![PyPI - Project Version](https://img.shields.io/pypi/v/lightningcss)](https://pypi.org/project/lightningcss)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/lightningcss)](https://pypi.org/project/lightningcss)
[![GitHub - Project License](https://img.shields.io/github/license/pydsigner/python-lightningcss)](https://github.com/pydsigner/python-lightningcss)

# python-lightningcss

python-lightningcss offers PyO3 bindings to the lightningcss library. Presently
only a subset of functionality is exposed, intended for integration into a
Python-centric web application rather than a JavaScript-based workflow. If the
latter is needed, consider using the [official npm lightningcss module, parcel,
or webpack](https://lightningcss.dev/docs.html) instead.

## Installation

python-lightningcss includes wheels which can be installed for most platforms
using pip: `pip install lightningcss`.

Alternatively, python-lightningcss may be installed directly from source:
`pip install git+https://github.com/pydsigner/python-lightningcss`. This will
require that the Rust toolchain be installed.

## Usage

```py
import lightningcss

parser_flags = lightningcss.calc_parser_flags(nesting=True)
input_css = """
a {
    padding-left: 0;
    padding-right: 0;
    padding-top: 5px;
    padding-bottom: 5px;
}
"""
output_css = lightningcss.process_stylesheet(input_css, filename="abc.css", browsers_list=['defaults'])
```
