use std::collections::HashSet;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use browserslist::Error as BrowserslistError;
use lightningcss::stylesheet::{StyleSheet, MinifyOptions, ParserFlags, ParserOptions, PrinterOptions};
use lightningcss::targets::{Browsers, Targets};

/// Processes provided CSS and returns as a string.
///
/// If `filename` is supplied, it will be used in parser error messages.
/// If `error_recovery` is True, broken CSS will be omitted rather than
/// producing a parse error. Enable with caution!
/// If `parser_flags` is supplied, it should be a flag created by
/// `calc_parser_flags()`. See that function for more details.
/// If `unused_symbols` is supplied, it should be a set of known unused
/// symbols, like class names, ids, or keyframe names, to be removed from the
/// output. Note that symbols should be specified in bare form, i.e.
/// `unused_symbols={'a', 'b'}`, not `unused_symbols={'.a', '#b'}`, and will
/// remove both ids and classes if they share a name. Use with caution!
/// If `browsers_list` is specified, it should be a list of browserslist
/// targets to be used to determine automatic prefixing and transpilation. If
/// it is not specified, no prefixing/transpilation will occur.
/// If `minify` is True, the final output will be minified. Otherwise, it will
/// be pretty-printed.
#[pyfunction]
#[pyo3(signature = (
    code,
    /,
    filename="",
    error_recovery=false,
    parser_flags=0,
    unused_symbols=None,
    browsers_list=None,
    minify=true,
))]
fn process_stylesheet(code: &str,
                      filename: &str,
                      error_recovery: bool,
                      parser_flags: u8,
                      unused_symbols: Option<HashSet<String>>,
                      browsers_list: Option<Vec<String>>,
                      minify: bool) -> PyResult<String> {

    let targets = match mk_targets(browsers_list) {
        Ok(val) => val,
        Err(e) => {return Err(PyValueError::new_err(format!("browsers_list failed validation: {}", e.to_string())))}
    };
    let mut stylesheet = match StyleSheet::parse(code, mk_parser_options(filename, error_recovery, parser_flags)) {
        Ok(val) => val,
        Err(e) => {return Err(PyValueError::new_err(format!("Parsing stylesheet failed: {}", e.to_string())))}
    };
    match stylesheet.minify(mk_minify_options(unused_symbols, &targets)) {
        Ok(_) => {}
        Err(e) => {return Err(PyValueError::new_err(format!("Minifying stylesheet failed: {}", e.to_string())));}
    }
    return match stylesheet.to_css(mk_printer_options(&targets, minify)) {
        Ok(val) => Ok(val.code),
        Err(e) => Err(PyValueError::new_err(format!("Printing stylesheet failed: {}", e.to_string())))
    };
}

/// Calculates parser_flags for process_stylesheet().
#[pyfunction]
#[pyo3(signature = (nesting=false, custom_media=false, deep_selector_combinator=false))]
fn calc_parser_flags(nesting: bool, custom_media: bool, deep_selector_combinator: bool) -> u8 {
    let mut flags = 0;
    if nesting { flags |= 1 << 0; }
    if custom_media { flags |= 1 << 1; }
    if deep_selector_combinator { flags |= 1 << 2; }
    return flags;
}

fn mk_parser_options(filename: &str,
                     error_recovery: bool,
                     parser_flags: u8) -> ParserOptions {
    return ParserOptions {
        filename: filename.to_string(),
        error_recovery: error_recovery,
        flags: ParserFlags::from_bits_truncate(parser_flags),
        ..Default::default()
    };
}

fn mk_targets(browsers_list: Option<Vec<String>>) -> Result<Targets, BrowserslistError> {
    match browsers_list {
        Some(bl) if !bl.is_empty() => Browsers::from_browserslist(bl).map(Targets::from),
        _ => Ok(Targets::default()),
    }
}

fn mk_minify_options(unused_symbols: Option<HashSet<String>>,
                     targets: &Targets) -> MinifyOptions {
    let mut options = MinifyOptions {targets: *targets, ..Default::default()};
    match unused_symbols {
        Some(val) => { options.unused_symbols = val; }
        None => {}
    }
    return options;
}

fn mk_printer_options<'a>(targets: &Targets,
                          minify: bool) -> PrinterOptions<'a> {
    return PrinterOptions {
        minify: minify,
        targets: *targets,
        ..PrinterOptions::default()
    };
}

/// A python wrapper for core functionality of lightningcss.
#[pymodule]
#[pyo3(name = "lightningcss")]
fn pylightningcss(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_stylesheet, m)?)?;
    m.add_function(wrap_pyfunction!(calc_parser_flags, m)?)?;
    Ok(())
}
