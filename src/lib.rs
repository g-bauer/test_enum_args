use pyo3::prelude::*;

/// An enum exposed to python.
#[pyclass]
#[derive(Clone)]
pub enum MyEnum {
    /// This documentation will not show up.
    Variant1,
}

/// Returns the enum variant as string.
/// 
/// Parameters
/// ----------
/// e : MyEnum, optional
///     The enum that will be stringified.
///     Defaults to MyEnum.Variant1.
/// 
/// Returns
/// -------
/// str
#[pyfunction]
#[pyo3(
    signature = (e=MyEnum::Variant1),
    // text_signature = "(e)",                   // this works
    text_signature = "(e=MyEnum.Variant1)",  // this does not work
)]
pub fn variant_string(e: MyEnum) -> PyResult<String> {
    match e {
        MyEnum::Variant1 => Ok("V1".to_string()),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn test_enum_args(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(variant_string, m)?)?;
    m.add_class::<MyEnum>()
}
