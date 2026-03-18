use std::collections::HashSet;
use std::path::Path;

use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;

use browserslist::Error as BrowserslistError;
use lightningcss::bundler::{Bundler, FileProvider, BundleErrorKind};
use lightningcss::stylesheet::{
    MinifyOptions, ParserFlags, ParserOptions, PrinterOptions, StyleSheet,
};
use lightningcss::targets::{Browsers, Targets};
use lightningcss::visitor::{Visit, VisitTypes};

mod visitor;

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
    visitor=None,
))]
fn process_stylesheet(code: &str,
                      filename: &str,
                      error_recovery: bool,
                      parser_flags: u8,
                      unused_symbols: Option<HashSet<String>>,
                      browsers_list: Option<Vec<String>>,
                      minify: bool,
                      visitor: Option<&mut visitor::PyVisitor>) -> PyResult<String> {

    let targets = match mk_targets(browsers_list) {
        Ok(val) => val,
        Err(e) => {return Err(PyValueError::new_err(format!("Validation of browsers_list failed: {}", e.to_string())))}
    };
    let mut stylesheet = match StyleSheet::parse(code, mk_parser_options(filename, error_recovery, parser_flags)) {
        Ok(val) => val,
        Err(e) => {return Err(PyValueError::new_err(format!("Parsing stylesheet failed: {}", e.to_string())))}
    };
    if let Some(visitor) = visitor {
        match stylesheet.visit(visitor) {
            Ok(_) => {}
            Err(e) => {return Err(PyValueError::new_err(format!("Visiting stylesheet failed: {}", e.to_string())))}
        }
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

fn mk_parser_options<'o, 'i>(filename: &'o str,
                             error_recovery: bool,
                             parser_flags: u8) -> ParserOptions<'o, 'i> {
    return ParserOptions {
        filename: filename.into(),
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

/// Bundles a CSS file and returns as a string.
#[pyfunction]
#[pyo3(signature = (
    path,
    /,
    error_recovery=false,
    parser_flags=0,
    unused_symbols=None,
    browsers_list=None,
    minify=true,
    visitor=None,
))]
fn bundle_css(
    path: &str,
    error_recovery: bool,
    parser_flags: u8,
    unused_symbols: Option<HashSet<String>>,
    browsers_list: Option<Vec<String>>,
    minify: bool,
    visitor: Option<&mut visitor::PyVisitor>
) -> PyResult<String> {
    let fs = FileProvider::new();
    let mut bundler = Bundler::new(
        &fs,
        None,
        ParserOptions {
            error_recovery: error_recovery,
            flags: ParserFlags::from_bits_truncate(parser_flags),
            ..Default::default()
        },
    );

    let mut stylesheet = match bundler.bundle(Path::new(&path)) {
        Ok(s) => s,
        Err(e) => {
            let message = e.to_string();
            match e.kind {
                // Resolver errors, probably related to file I/O
                BundleErrorKind::ResolverError(_) => {
                    return Err(PyIOError::new_err(format!("Bundling failed: {}", message)));
                }
                // Parser and logical errors
                _ => {
                    return Err(PyValueError::new_err(format!("Bundling failed: {}", message)));
                }
            }
        }
    };

    if let Some(visitor) = visitor {
        match stylesheet.visit(visitor) {
            Ok(_) => {}
            Err(e) => {return Err(PyValueError::new_err(format!("Visiting stylesheet failed: {}", e.to_string())))}
        }
    };

    let targets = match mk_targets(browsers_list) {
        Ok(val) => val,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "browsers_list failed validation: {}",
                e.to_string()
            )))
        }
    };

    match stylesheet.minify(mk_minify_options(unused_symbols, &targets)) {
        Ok(_) => {}
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Minifying stylesheet failed: {}",
                e.to_string()
            )));
        }
    }
    return match stylesheet.to_css(mk_printer_options(&targets, minify)) {
        Ok(val) => Ok(val.code),
        Err(e) => Err(PyValueError::new_err(format!(
            "Printing stylesheet failed: {}",
            e.to_string()
        ))),
    };
}

/// A python wrapper for core functionality of lightningcss.
#[pymodule(name = "lightningcss")]
mod py_lightningcss {
    #[pymodule_export]
    use super::calc_parser_flags;
    #[pymodule_export]
    use super::process_stylesheet;
    #[pymodule_export]
    use super::bundle_css;

    #[pymodule_export(name = "Visitor")]
    use super::visitor::PyVisitor;

    // Export CSS value type classes
    #[pymodule_export(name = "Url")]
    use super::visitor::PyUrl;
    #[pymodule_export(name = "CssColor")]
    use super::visitor::PyCssColor;
    #[pymodule_export(name = "LengthValue")]
    use super::visitor::PyLengthValue;
    #[pymodule_export(name = "Angle")]
    use super::visitor::PyAngle;
    #[pymodule_export(name = "Ratio")]
    use super::visitor::PyRatio;
    #[pymodule_export(name = "Resolution")]
    use super::visitor::PyResolution;
    #[pymodule_export(name = "Time")]
    use super::visitor::PyTime;
    #[pymodule_export(name = "CustomIdent")]
    use super::visitor::PyCustomIdent;
    #[pymodule_export(name = "DashedIdent")]
    use super::visitor::PyDashedIdent;
    #[pymodule_export(name = "Selector")]
    use super::visitor::PySelector;
    #[pymodule_export(name = "CssRule")]
    use super::visitor::PyCssRule;
    #[pymodule_export(name = "Property")]
    use super::visitor::PyProperty;
    #[pymodule_export(name = "Image")]
    use super::visitor::PyImage;
    #[pymodule_export(name = "Variable")]
    use super::visitor::PyVariable;
    #[pymodule_export(name = "EnvironmentVariable")]
    use super::visitor::PyEnvironmentVariable;
    #[pymodule_export(name = "MediaQuery")]
    use super::visitor::PyMediaQuery;
    #[pymodule_export(name = "SupportsCondition")]
    use super::visitor::PySupportsCondition;
    #[pymodule_export(name = "Function")]
    use super::visitor::PyFunction;
    #[pymodule_export(name = "TokenOrValue")]
    use super::visitor::PyTokenOrValue;

    // Export all VisitTypes flag constants so they're accessible from Python
    #[pymodule_export]
    const VISIT_RULES: u32 = super::VisitTypes::RULES.bits();
    #[pymodule_export]
    const VISIT_PROPERTIES: u32 = super::VisitTypes::PROPERTIES.bits();
    #[pymodule_export]
    const VISIT_URLS: u32 = super::VisitTypes::URLS.bits();
    #[pymodule_export]
    const VISIT_COLORS: u32 = super::VisitTypes::COLORS.bits();
    #[pymodule_export]
    const VISIT_IMAGES: u32 = super::VisitTypes::IMAGES.bits();
    #[pymodule_export]
    const VISIT_LENGTHS: u32 = super::VisitTypes::LENGTHS.bits();
    #[pymodule_export]
    const VISIT_ANGLES: u32 = super::VisitTypes::ANGLES.bits();
    #[pymodule_export]
    const VISIT_RATIOS: u32 = super::VisitTypes::RATIOS.bits();
    #[pymodule_export]
    const VISIT_RESOLUTIONS: u32 = super::VisitTypes::RESOLUTIONS.bits();
    #[pymodule_export]
    const VISIT_TIMES: u32 = super::VisitTypes::TIMES.bits();
    #[pymodule_export]
    const VISIT_CUSTOM_IDENTS: u32 = super::VisitTypes::CUSTOM_IDENTS.bits();
    #[pymodule_export]
    const VISIT_DASHED_IDENTS: u32 = super::VisitTypes::DASHED_IDENTS.bits();
    #[pymodule_export]
    const VISIT_VARIABLES: u32 = super::VisitTypes::VARIABLES.bits();
    #[pymodule_export]
    const VISIT_ENVIRONMENT_VARIABLES: u32 = super::VisitTypes::ENVIRONMENT_VARIABLES.bits();
    #[pymodule_export]
    const VISIT_MEDIA_QUERIES: u32 = super::VisitTypes::MEDIA_QUERIES.bits();
    #[pymodule_export]
    const VISIT_SUPPORTS_CONDITIONS: u32 = super::VisitTypes::SUPPORTS_CONDITIONS.bits();
    #[pymodule_export]
    const VISIT_SELECTORS: u32 = super::VisitTypes::SELECTORS.bits();
    #[pymodule_export]
    const VISIT_FUNCTIONS: u32 = super::VisitTypes::FUNCTIONS.bits();
    #[pymodule_export]
    const VISIT_TOKENS: u32 = super::VisitTypes::TOKENS.bits();
}
