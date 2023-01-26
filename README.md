# Enums in `text_signature`

Test how Sphinx can be used to document Python packages built with PyO3.

## Setup

Setup a virtualenv or conda environment and activate environment.

```
virtualenv venv
source venv/bin/activate
pip install maturin sphinx
```

Build python package

```
maturin develop
```

Build documentation

```
cd docs
make html
```

## Problem

When an enum is used as default argument withing PyO3's `text_signature`,
Sphinx is not able to process the signature:

```rust
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
``` 

When the `text_signature` is removed (or the enum), everything works as expected.

### Error
```
WARNING: error while formatting signature for test_enum_args.variant_string: Handler <function record_typehints at 0x7f90007ed3a0> for event 'autodoc-process-signature' threw an exception (exception: )
```