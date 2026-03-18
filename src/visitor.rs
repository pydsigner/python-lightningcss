use lightningcss::dependencies::Location as UrlLocation;
use lightningcss::media_query::MediaQuery;
use lightningcss::printer::PrinterOptions;
use lightningcss::properties::custom::{EnvironmentVariable, Function, TokenOrValue, Variable};
use lightningcss::properties::Property;
use lightningcss::rules::supports::SupportsCondition;
use lightningcss::rules::CssRule;
use lightningcss::selector::Selector;
use lightningcss::stylesheet::ParserOptions;
use lightningcss::traits::{Parse, ParseWithOptions, ToCss};
use lightningcss::values::angle::Angle;
use lightningcss::values::color::CssColor;
use lightningcss::values::ident::{CustomIdent, DashedIdent};
use lightningcss::values::image::Image;
use lightningcss::values::length::LengthValue;
use lightningcss::values::ratio::Ratio;
use lightningcss::values::resolution::Resolution;
use lightningcss::values::time::Time;
use lightningcss::values::url::Url;
use lightningcss::visitor::{Visit, VisitTypes, Visitor};

use pyo3::exceptions::PyAttributeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// ── Helper ──────────────────────────────────────────────────────────────────

/// Serialize any lightningcss ToCss value to a CSS string using default options.
fn lc_to_css_string<T: ToCss>(value: &T) -> String {
    value
        .to_css_string(PrinterOptions::default())
        .unwrap_or_default()
}

// ── PyAngle ──────────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::angle::Angle`.
#[pyclass(module = "lightningcss", name = "Angle", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyAngle {
    #[pyo3(get, set)]
    pub value: f32,
    #[pyo3(get, set)]
    pub unit: String,
}

#[pymethods]
impl PyAngle {
    #[new]
    pub fn new(value: f32, unit: String) -> Self {
        Self { value, unit }
    }
    fn __repr__(&self) -> String {
        format!("Angle(value={}, unit={:?})", self.value, self.unit)
    }
    fn __str__(&self) -> String {
        format!("{}{}", self.value, self.unit)
    }
}

impl From<Angle> for PyAngle {
    fn from(a: Angle) -> Self {
        let (value, unit) = match a {
            Angle::Deg(v) => (v, "deg"),
            Angle::Rad(v) => (v, "rad"),
            Angle::Grad(v) => (v, "grad"),
            Angle::Turn(v) => (v, "turn"),
        };
        Self { value, unit: unit.to_string() }
    }
}

impl TryFrom<&PyAngle> for Angle {
    type Error = PyErr;
    fn try_from(v: &PyAngle) -> Result<Self, Self::Error> {
        let css = format!("{}{}", v.value, v.unit);
        Angle::parse_string(css.as_str()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse Angle from '{}': {}", css, e))
        })
    }
}

// ── PyRatio ──────────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::ratio::Ratio`.
#[pyclass(module = "lightningcss", name = "Ratio", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyRatio {
    #[pyo3(get, set)]
    pub numerator: f32,
    #[pyo3(get, set)]
    pub denominator: f32,
}

#[pymethods]
impl PyRatio {
    #[new]
    pub fn new(numerator: f32, denominator: f32) -> Self {
        Self { numerator, denominator }
    }
    fn __repr__(&self) -> String {
        format!("Ratio(numerator={}, denominator={})", self.numerator, self.denominator)
    }
    fn __str__(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }
}

impl From<Ratio> for PyRatio {
    fn from(r: Ratio) -> Self {
        Self { numerator: r.0, denominator: r.1 }
    }
}

impl TryFrom<&PyRatio> for Ratio {
    type Error = PyErr;
    fn try_from(v: &PyRatio) -> Result<Self, Self::Error> {
        let css = format!("{}/{}", v.numerator, v.denominator);
        Ratio::parse_string(css.as_str()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse Ratio from '{}': {}", css, e))
        })
    }
}

// ── PyResolution ─────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::resolution::Resolution`.
#[pyclass(module = "lightningcss", name = "Resolution", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyResolution {
    #[pyo3(get, set)]
    pub value: f32,
    #[pyo3(get, set)]
    pub unit: String,
}

#[pymethods]
impl PyResolution {
    #[new]
    pub fn new(value: f32, unit: String) -> Self {
        Self { value, unit }
    }
    fn __repr__(&self) -> String {
        format!("Resolution(value={}, unit={:?})", self.value, self.unit)
    }
    fn __str__(&self) -> String {
        format!("{}{}", self.value, self.unit)
    }
}

impl From<Resolution> for PyResolution {
    fn from(r: Resolution) -> Self {
        let (value, unit) = match r {
            Resolution::Dpi(v) => (v, "dpi"),
            Resolution::Dpcm(v) => (v, "dpcm"),
            Resolution::Dppx(v) => (v, "dppx"),
        };
        Self { value, unit: unit.to_string() }
    }
}

impl TryFrom<&PyResolution> for Resolution {
    type Error = PyErr;
    fn try_from(v: &PyResolution) -> Result<Self, Self::Error> {
        let css = format!("{}{}", v.value, v.unit);
        Resolution::parse_string(css.as_str()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse Resolution from '{}': {}", css, e))
        })
    }
}

// ── PyTime ────────────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::time::Time`.
#[pyclass(module = "lightningcss", name = "Time", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyTime {
    #[pyo3(get, set)]
    pub value: f32,
    #[pyo3(get, set)]
    pub unit: String,
}

#[pymethods]
impl PyTime {
    #[new]
    pub fn new(value: f32, unit: String) -> Self {
        Self { value, unit }
    }
    fn __repr__(&self) -> String {
        format!("Time(value={}, unit={:?})", self.value, self.unit)
    }
    fn __str__(&self) -> String {
        format!("{}{}", self.value, self.unit)
    }
}

impl From<Time> for PyTime {
    fn from(t: Time) -> Self {
        let (value, unit) = match t {
            Time::Seconds(v) => (v, "s"),
            Time::Milliseconds(v) => (v, "ms"),
        };
        Self { value, unit: unit.to_string() }
    }
}

impl TryFrom<&PyTime> for Time {
    type Error = PyErr;
    fn try_from(v: &PyTime) -> Result<Self, Self::Error> {
        let css = format!("{}{}", v.value, v.unit);
        Time::parse_string(css.as_str()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse Time from '{}': {}", css, e))
        })
    }
}

// ── PyLengthValue ─────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::length::LengthValue`.
///
/// Represents any CSS length as a `(value: f32, unit: str)` pair, e.g.
/// `(32.0, "px")`, `(1.5, "rem")`, `(100.0, "vw")`.
#[pyclass(module = "lightningcss", name = "LengthValue", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyLengthValue {
    #[pyo3(get, set)]
    pub value: f32,
    #[pyo3(get, set)]
    pub unit: String,
}

#[pymethods]
impl PyLengthValue {
    #[new]
    pub fn new(value: f32, unit: String) -> Self {
        Self { value, unit }
    }
    fn __repr__(&self) -> String {
        format!("LengthValue(value={}, unit={:?})", self.value, self.unit)
    }
    fn __str__(&self) -> String {
        format!("{}{}", self.value, self.unit)
    }
}

impl From<LengthValue> for PyLengthValue {
    fn from(l: LengthValue) -> Self {
        let (value, unit) = l.to_unit_value();
        Self { value, unit: unit.to_string() }
    }
}

impl TryFrom<&PyLengthValue> for LengthValue {
    type Error = PyErr;
    fn try_from(v: &PyLengthValue) -> Result<Self, Self::Error> {
        let css = format!("{}{}", v.value, v.unit);
        LengthValue::parse_string(css.as_str()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse LengthValue from '{}': {}", css, e))
        })
    }
}

// ── PyCssColor ────────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::color::CssColor`.
/// Constructed from a CSS color string; the underlying parsed color is held directly.
#[pyclass(module = "lightningcss", name = "CssColor", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyCssColor {
    pub inner: CssColor,
}

#[pymethods]
impl PyCssColor {
    /// Create from a CSS color string, e.g. `"red"`, `"#ff0000"`, `"hsl(0 100% 50%)"`.
    #[new]
    pub fn new(css: String) -> PyResult<Self> {
        CssColor::parse_string(css.as_str())
            .map(|inner| Self { inner })
            .map_err(|e| PyValueError::new_err(format!("Failed to parse color '{}': {}", css, e)))
    }
    /// Return the canonical CSS string representation.
    pub fn css(&self) -> String {
        lc_to_css_string(&self.inner)
    }
    fn __repr__(&self) -> String {
        format!("CssColor({:?})", lc_to_css_string(&self.inner))
    }
    fn __str__(&self) -> String {
        lc_to_css_string(&self.inner)
    }
}

impl From<CssColor> for PyCssColor {
    fn from(c: CssColor) -> Self {
        Self { inner: c }
    }
}

impl From<PyCssColor> for CssColor {
    fn from(c: PyCssColor) -> Self {
        c.inner
    }
}

impl From<&PyCssColor> for CssColor {
    fn from(c: &PyCssColor) -> Self {
        c.inner.clone()
    }
}

// ── PyUrl ─────────────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::url::Url`.
#[pyclass(module = "lightningcss", name = "Url", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyUrl {
    #[pyo3(get, set)]
    pub url: String,
}

#[pymethods]
impl PyUrl {
    #[new]
    pub fn new(url: String) -> Self {
        Self { url }
    }
    fn __repr__(&self) -> String {
        format!("Url(url={:?})", self.url)
    }
    fn __str__(&self) -> String {
        format!("url({:?})", self.url)
    }
}

impl<'i> From<Url<'i>> for PyUrl {
    fn from(u: Url<'i>) -> Self {
        Self { url: u.url.as_ref().to_string() }
    }
}

impl<'i> From<&PyUrl> for Url<'i> {
    fn from(v: &PyUrl) -> Self {
        Url {
            url: v.url.clone().into(),
            loc: UrlLocation { line: 0, column: 0 },
        }
    }
}

// ── PyCustomIdent ─────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::ident::CustomIdent`.
#[pyclass(module = "lightningcss", name = "CustomIdent", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyCustomIdent {
    #[pyo3(get, set)]
    pub ident: String,
}

#[pymethods]
impl PyCustomIdent {
    #[new]
    pub fn new(ident: String) -> Self {
        Self { ident }
    }
    fn __repr__(&self) -> String {
        format!("CustomIdent(ident={:?})", self.ident)
    }
    fn __str__(&self) -> String {
        self.ident.clone()
    }
}

impl<'i> From<CustomIdent<'i>> for PyCustomIdent {
    fn from(c: CustomIdent<'i>) -> Self {
        Self { ident: c.0.as_ref().to_string() }
    }
}

impl<'i> From<&PyCustomIdent> for CustomIdent<'i> {
    fn from(v: &PyCustomIdent) -> Self {
        CustomIdent(v.ident.clone().into())
    }
}

// ── PyDashedIdent ─────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::values::ident::DashedIdent`.
#[pyclass(module = "lightningcss", name = "DashedIdent", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyDashedIdent {
    #[pyo3(get, set)]
    pub ident: String,
}

#[pymethods]
impl PyDashedIdent {
    #[new]
    pub fn new(ident: String) -> Self {
        Self { ident }
    }
    fn __repr__(&self) -> String {
        format!("DashedIdent(ident={:?})", self.ident)
    }
    fn __str__(&self) -> String {
        self.ident.clone()
    }
}

impl<'i> From<DashedIdent<'i>> for PyDashedIdent {
    fn from(d: DashedIdent<'i>) -> Self {
        Self { ident: d.0.as_ref().to_string() }
    }
}

impl<'i> From<&PyDashedIdent> for DashedIdent<'i> {
    fn from(v: &PyDashedIdent) -> Self {
        DashedIdent(v.ident.clone().into())
    }
}

// ── PyFunction ────────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::properties::custom::Function`.
#[pyclass(module = "lightningcss", name = "Function", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyFunction {
    #[pyo3(get, set)]
    pub name: String,
    /// Debug representation of the argument token list.
    #[pyo3(get, set)]
    pub arguments: String,
}

#[pymethods]
impl PyFunction {
    #[new]
    pub fn new(name: String, arguments: String) -> Self {
        Self { name, arguments }
    }
    fn __repr__(&self) -> String {
        format!("Function(name={:?}, arguments={:?})", self.name, self.arguments)
    }
    fn __str__(&self) -> String {
        format!("{}({})", self.name, self.arguments)
    }
}

impl<'i> From<&Function<'i>> for PyFunction {
    fn from(f: &Function<'i>) -> Self {
        Self {
            name: f.name.0.as_ref().to_string(),
            arguments: format!("{:?}", f.arguments),
        }
    }
}

// ── PyVariable ────────────────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::properties::custom::Variable`.
#[pyclass(module = "lightningcss", name = "Variable", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyVariable {
    /// The CSS custom property name, e.g. `"--my-color"`.
    #[pyo3(get, set)]
    pub name: String,
    /// Debug representation of the optional fallback token list.
    #[pyo3(get, set)]
    pub fallback: Option<String>,
}

#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature = (name, fallback = None))]
    pub fn new(name: String, fallback: Option<String>) -> Self {
        Self { name, fallback }
    }
    fn __repr__(&self) -> String {
        format!("Variable(name={:?}, fallback={:?})", self.name, self.fallback)
    }
    fn __str__(&self) -> String {
        match &self.fallback {
            Some(fb) => format!("var({}, {})", self.name, fb),
            None => format!("var({})", self.name),
        }
    }
}

impl<'i> From<&Variable<'i>> for PyVariable {
    fn from(v: &Variable<'i>) -> Self {
        Self {
            name: lc_to_css_string(&v.name),
            fallback: v.fallback.as_ref().map(|fb| format!("{:?}", fb)),
        }
    }
}

// ── PyEnvironmentVariable ─────────────────────────────────────────────────────

/// Python wrapper for `lightningcss::properties::custom::EnvironmentVariable`.
#[pyclass(module = "lightningcss", name = "EnvironmentVariable", from_py_object)]
#[derive(Clone, Debug)]
pub struct PyEnvironmentVariable {
    /// The environment variable name, e.g. `"safe-area-inset-top"`.
    #[pyo3(get, set)]
    pub name: String,
    /// Optional integer indices into the environment variable dimensions.
    #[pyo3(get, set)]
    pub indices: Vec<i32>,
    /// Debug representation of the optional fallback token list.
    #[pyo3(get, set)]
    pub fallback: Option<String>,
}

#[pymethods]
impl PyEnvironmentVariable {
    #[new]
    #[pyo3(signature = (name, indices = vec![], fallback = None))]
    pub fn new(name: String, indices: Vec<i32>, fallback: Option<String>) -> Self {
        Self { name, indices, fallback }
    }
    fn __repr__(&self) -> String {
        format!(
            "EnvironmentVariable(name={:?}, indices={:?}, fallback={:?})",
            self.name, self.indices, self.fallback
        )
    }
    fn __str__(&self) -> String {
        let mut s = format!("env({})", self.name);
        for i in &self.indices {
            s.push_str(&format!(" {}", i));
        }
        if let Some(fb) = &self.fallback {
            s.push_str(&format!(", {}", fb));
        }
        s
    }
}

impl<'i> From<&EnvironmentVariable<'i>> for PyEnvironmentVariable {
    fn from(e: &EnvironmentVariable<'i>) -> Self {
        Self {
            name: lc_to_css_string(&e.name),
            indices: e.indices.iter().map(|i| *i as i32).collect(),
            fallback: e.fallback.as_ref().map(|fb| format!("{:?}", fb)),
        }
    }
}

// ── CSS-string-backed wrappers for complex recursive types ────────────────────

macro_rules! impl_css_string_wrapper {
    ($wrapper:ident, $name:literal, $doc:literal) => {
        #[doc = $doc]
        #[pyclass(module = "lightningcss", name = $name, from_py_object)]
        #[derive(Clone, Debug)]
        pub struct $wrapper {
            #[pyo3(get)]
            pub css: String,
        }

        #[pymethods]
        impl $wrapper {
            #[new]
            pub fn new(css: String) -> Self {
                Self { css }
            }
            fn __repr__(&self) -> String {
                format!("{}({:?})", stringify!($wrapper), self.css)
            }
            fn __str__(&self) -> String {
                self.css.clone()
            }
        }
    };
}

impl_css_string_wrapper!(PyImage, "Image", "Python wrapper for `lightningcss::values::image::Image`.");
impl_css_string_wrapper!(PySelector, "Selector", "Python wrapper for a CSS selector.");
impl_css_string_wrapper!(PyMediaQuery, "MediaQuery", "Python wrapper for `lightningcss::media_query::MediaQuery`.");
impl_css_string_wrapper!(PySupportsCondition, "SupportsCondition", "Python wrapper for `lightningcss::rules::supports::SupportsCondition`.");
impl_css_string_wrapper!(PyCssRule, "CssRule", "Python wrapper for `lightningcss::rules::CssRule`.");
impl_css_string_wrapper!(PyProperty, "Property", "Python wrapper for `lightningcss::properties::Property`.");
impl_css_string_wrapper!(PyTokenOrValue, "TokenOrValue", "Python wrapper for `lightningcss::properties::custom::TokenOrValue`.");

impl<'i> From<&Image<'i>> for PyImage {
    fn from(v: &Image<'i>) -> Self { Self { css: lc_to_css_string(v) } }
}
impl<'i> TryFrom<&'i PyImage> for Image<'i> {
    type Error = PyErr;
    fn try_from(v: &'i PyImage) -> Result<Self, Self::Error> {
        Image::parse_string(v.css.as_str()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse Image from '{}': {}", v.css, e))
        })
    }
}

impl<'i> From<&Selector<'i>> for PySelector {
    fn from(v: &Selector<'i>) -> Self { Self { css: lc_to_css_string(v) } }
}
impl<'i> TryFrom<&'i PySelector> for Selector<'i> {
    type Error = PyErr;
    fn try_from(v: &'i PySelector) -> Result<Self, Self::Error> {
        Selector::parse_string_with_options(v.css.as_str(), ParserOptions::default()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse Selector from '{}': {}", v.css, e))
        })
    }
}

impl<'i> From<&MediaQuery<'i>> for PyMediaQuery {
    fn from(v: &MediaQuery<'i>) -> Self { Self { css: lc_to_css_string(v) } }
}
impl<'i> TryFrom<&'i PyMediaQuery> for MediaQuery<'i> {
    type Error = PyErr;
    fn try_from(v: &'i PyMediaQuery) -> Result<Self, Self::Error> {
        MediaQuery::parse_string_with_options(v.css.as_str(), ParserOptions::default()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse MediaQuery from '{}': {}", v.css, e))
        })
    }
}

impl<'i> From<&SupportsCondition<'i>> for PySupportsCondition {
    fn from(v: &SupportsCondition<'i>) -> Self { Self { css: lc_to_css_string(v) } }
}
impl<'i> TryFrom<&'i PySupportsCondition> for SupportsCondition<'i> {
    type Error = PyErr;
    fn try_from(v: &'i PySupportsCondition) -> Result<Self, Self::Error> {
        SupportsCondition::parse_string(v.css.as_str()).map_err(|e| {
            PyValueError::new_err(format!("Failed to parse SupportsCondition from '{}': {}", v.css, e))
        })
    }
}

impl<'i> From<&CssRule<'i>> for PyCssRule {
    fn from(v: &CssRule<'i>) -> Self { Self { css: lc_to_css_string(v) } }
}

impl<'i> From<&Property<'i>> for PyProperty {
    fn from(v: &Property<'i>) -> Self { Self { css: format!("{:?}", v) } }
}

impl<'i> From<&TokenOrValue<'i>> for PyTokenOrValue {
    fn from(v: &TokenOrValue<'i>) -> Self { Self { css: format!("{:?}", v) } }
}


/// A Visitor implementation that delegates to Python callback functions.
///
/// Subclass this from Python and implement methods like `visit_url`,
/// `visit_color`, etc. Methods receive typed wrapper objects (e.g. `Url`,
/// `CssColor`, `LengthValue`). For leaf values, returning a replacement wrapper
/// mutates the stylesheet; returning `None` leaves the original value unchanged.
///
/// Leaf types (URLs, colors, lengths, angles, ratios, resolutions, times,
/// custom idents, dashed idents) support mutation via the return value.
/// Branch types (rules, properties, images, variables, etc.) call the callback
/// for observation; the return value is ignored and child traversal continues.
#[pyclass(module="lightningcss", name = "Visitor", subclass)]
pub struct PyVisitor {
    visit_types: VisitTypes,
    py_self: Option<Py<PyAny>>,
}

#[pymethods]
impl PyVisitor {
    /// Creates a `lightningcss::visitor::Visitor`.
    #[new]
    pub fn new() -> Self {
        PyVisitor {
            visit_types: VisitTypes::empty(),
            py_self: None,
        }
    }

    /// Initializes with VisitTypes bitflags (e.g. `VISIT_URLS | VISIT_COLORS`).
    #[pyo3(signature = (visit_types = 0))]
    fn __init__(slf: Py<Self>, visit_types: u32) -> PyResult<()> {
        Python::attach(|py| {
            let mut this = slf.borrow_mut(py);
            this.visit_types = VisitTypes::from_bits_truncate(visit_types);
            this.py_self = Some(slf.clone_ref(py).into_any());
            Ok(())
        })
    }
}

macro_rules! call_leaf_callback {
    ($self:expr, $flag:expr, $method:literal, $wrapper:ty, $current:expr, $apply_new:expr) => {{
        if $self.visit_types.contains($flag) {
            Python::attach(|py| -> PyResult<()> {
                let py_self = $self
                    .py_self
                    .as_ref()
                    .ok_or_else(|| PyValueError::new_err("Visitor is not initialized"))?;
                let wrapped: $wrapper = $current;
                let arg = Py::new(py, wrapped)?;
                let result = match py_self.bind(py).call_method1($method, (arg,)) {
                    Ok(result) => result,
                    Err(err) if err.is_instance_of::<PyAttributeError>(py) => return Ok(()),
                    Err(err) => return Err(err),
                };
                if !result.is_none() {
                    let new_value: $wrapper = result.extract()?;
                    $apply_new(new_value)?;
                }
                Ok(())
            })?;
        }
    }};
}

macro_rules! call_observe_callback {
    ($self:expr, $flag:expr, $method:literal, $wrapper:ty, $current:expr) => {{
        if $self.visit_types.contains($flag) {
            Python::attach(|py| -> PyResult<()> {
                let py_self = $self
                    .py_self
                    .as_ref()
                    .ok_or_else(|| PyValueError::new_err("Visitor is not initialized"))?;
                let wrapped: $wrapper = $current;
                let arg = Py::new(py, wrapped)?;
                match py_self.bind(py).call_method1($method, (arg,)) {
                    Ok(_) => Ok(()),
                    Err(err) if err.is_instance_of::<PyAttributeError>(py) => Ok(()),
                    Err(err) => Err(err),
                }
            })?;
        }
    }};
}

impl<'i> Visitor<'i> for PyVisitor {
    type Error = PyErr;

    fn visit_types(&self) -> VisitTypes {
        self.visit_types
    }

    // === Leaf visit methods (support mutation via callback return value) ===

    fn visit_url(&mut self, url: &mut Url<'i>) -> Result<(), Self::Error> {
        call_leaf_callback!(self, VisitTypes::URLS, "visit_url", PyUrl, PyUrl::from(url.clone()), |new_url: PyUrl| -> PyResult<()> {
            *url = Url::from(&new_url);
            Ok(())
        });
        Ok(())
    }

    fn visit_color(&mut self, color: &mut CssColor) -> Result<(), Self::Error> {
        call_leaf_callback!(self, VisitTypes::COLORS, "visit_color", PyCssColor, PyCssColor::from(color.clone()), |new_color: PyCssColor| -> PyResult<()> {
            *color = CssColor::from(new_color);
            Ok(())
        });
        Ok(())
    }

    fn visit_length(&mut self, length: &mut LengthValue) -> Result<(), Self::Error> {
        call_leaf_callback!(self, VisitTypes::LENGTHS, "visit_length", PyLengthValue, PyLengthValue::from(length.clone()), |new_length: PyLengthValue| -> PyResult<()> {
            *length = LengthValue::try_from(&new_length)?;
            Ok(())
        });
        Ok(())
    }

    fn visit_angle(&mut self, angle: &mut Angle) -> Result<(), Self::Error> {
        call_leaf_callback!(self, VisitTypes::ANGLES, "visit_angle", PyAngle, PyAngle::from(angle.clone()), |new_angle: PyAngle| -> PyResult<()> {
            *angle = Angle::try_from(&new_angle)?;
            Ok(())
        });
        Ok(())
    }

    fn visit_ratio(&mut self, ratio: &mut Ratio) -> Result<(), Self::Error> {
        call_leaf_callback!(self, VisitTypes::RATIOS, "visit_ratio", PyRatio, PyRatio::from(ratio.clone()), |new_ratio: PyRatio| -> PyResult<()> {
            *ratio = Ratio::try_from(&new_ratio)?;
            Ok(())
        });
        Ok(())
    }

    fn visit_resolution(&mut self, resolution: &mut Resolution) -> Result<(), Self::Error> {
        call_leaf_callback!(
            self,
            VisitTypes::RESOLUTIONS,
            "visit_resolution",
            PyResolution,
            PyResolution::from(resolution.clone()),
            |new_resolution: PyResolution| -> PyResult<()> {
                *resolution = Resolution::try_from(&new_resolution)?;
                Ok(())
            }
        );
        Ok(())
    }

    fn visit_time(&mut self, time: &mut Time) -> Result<(), Self::Error> {
        call_leaf_callback!(self, VisitTypes::TIMES, "visit_time", PyTime, PyTime::from(time.clone()), |new_time: PyTime| -> PyResult<()> {
            *time = Time::try_from(&new_time)?;
            Ok(())
        });
        Ok(())
    }

    fn visit_custom_ident(&mut self, ident: &mut CustomIdent) -> Result<(), Self::Error> {
        call_leaf_callback!(
            self,
            VisitTypes::CUSTOM_IDENTS,
            "visit_custom_ident",
            PyCustomIdent,
            PyCustomIdent::from(ident.clone()),
            |new_ident: PyCustomIdent| -> PyResult<()> {
                *ident = CustomIdent::from(&new_ident);
                Ok(())
            }
        );
        Ok(())
    }

    fn visit_dashed_ident(&mut self, ident: &mut DashedIdent) -> Result<(), Self::Error> {
        call_leaf_callback!(
            self,
            VisitTypes::DASHED_IDENTS,
            "visit_dashed_ident",
            PyDashedIdent,
            PyDashedIdent::from(ident.clone()),
            |new_ident: PyDashedIdent| -> PyResult<()> {
                *ident = DashedIdent::from(&new_ident);
                Ok(())
            }
        );
        Ok(())
    }

    fn visit_selector(&mut self, selector: &mut Selector<'i>) -> Result<(), Self::Error> {
        call_observe_callback!(
            self,
            VisitTypes::SELECTORS,
            "visit_selector",
            PySelector,
            PySelector::from(selector as &Selector)
        );
        Ok(())
    }

    // === Branch visit methods (observation callback + child recursion) ===

    fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
        call_observe_callback!(self, VisitTypes::RULES, "visit_rule", PyCssRule, PyCssRule::from(rule as &CssRule));
        rule.visit_children(self)
    }

    fn visit_property(&mut self, property: &mut Property<'i>) -> Result<(), Self::Error> {
        call_observe_callback!(self, VisitTypes::PROPERTIES, "visit_property", PyProperty, PyProperty::from(property as &Property));
        property.visit_children(self)
    }

    fn visit_image(&mut self, image: &mut Image<'i>) -> Result<(), Self::Error> {
        call_observe_callback!(self, VisitTypes::IMAGES, "visit_image", PyImage, PyImage::from(image as &Image));
        image.visit_children(self)
    }

    fn visit_variable(&mut self, variable: &mut Variable<'i>) -> Result<(), Self::Error> {
        call_observe_callback!(self, VisitTypes::VARIABLES, "visit_variable", PyVariable, PyVariable::from(variable as &Variable));
        variable.visit_children(self)
    }

    fn visit_environment_variable(
        &mut self,
        env: &mut EnvironmentVariable<'i>,
    ) -> Result<(), Self::Error> {
        call_observe_callback!(
            self,
            VisitTypes::ENVIRONMENT_VARIABLES,
            "visit_environment_variable",
            PyEnvironmentVariable,
            PyEnvironmentVariable::from(env as &EnvironmentVariable)
        );
        env.visit_children(self)
    }

    fn visit_media_query(&mut self, query: &mut MediaQuery<'i>) -> Result<(), Self::Error> {
        call_observe_callback!(self, VisitTypes::MEDIA_QUERIES, "visit_media_query", PyMediaQuery, PyMediaQuery::from(query as &MediaQuery));
        query.visit_children(self)
    }

    fn visit_supports_condition(
        &mut self,
        condition: &mut SupportsCondition<'i>,
    ) -> Result<(), Self::Error> {
        call_observe_callback!(
            self,
            VisitTypes::SUPPORTS_CONDITIONS,
            "visit_supports_condition",
            PySupportsCondition,
            PySupportsCondition::from(condition as &SupportsCondition)
        );
        condition.visit_children(self)
    }

    fn visit_function(&mut self, function: &mut Function<'i>) -> Result<(), Self::Error> {
        call_observe_callback!(self, VisitTypes::FUNCTIONS, "visit_function", PyFunction, PyFunction::from(function as &Function));
        function.visit_children(self)
    }

    fn visit_token(&mut self, token: &mut TokenOrValue<'i>) -> Result<(), Self::Error> {
        call_observe_callback!(self, VisitTypes::TOKENS, "visit_token", PyTokenOrValue, PyTokenOrValue::from(token as &TokenOrValue));
        token.visit_children(self)
    }
}
