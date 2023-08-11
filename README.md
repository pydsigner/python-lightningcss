# python-lightningcss

python-lightningcss offers PyO3 bindings to the lightningcss library. Presently
only a subset of functionality is exposed, intended for integration into a
Python-centric web application rather than a JavaScript-based workflow. If the
latter is needed, consider using the [official npm lightningcss module, parcel,
or webpack](https://lightningcss.dev/docs.html) instead.

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
