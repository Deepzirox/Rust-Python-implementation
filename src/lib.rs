#[macro_use]
extern crate cpython;


use cpython::{PyResult, Python, PyDict, PyObject};

/* fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }
    Ok(total)
} */

fn get_value_from_rust(_py: Python, val: PyDict, tofind: &str) -> PyResult<PyObject>
{
    Ok(val.get_item(_py, tofind).unwrap())
}

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m| {
    m.add(py, "__doc__", "Libreria en Rust para Python")?;
    m.add(py, "get_value_from_rust", py_fn!(py, get_value_from_rust(val: PyDict, tofind: &str)))?;
    Ok(())
});
