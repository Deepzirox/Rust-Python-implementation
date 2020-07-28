#[macro_use]
extern crate cpython;
use std::ptr;


use cpython::{PyResult, Python, PyDict, PyObject, PyList, PyInt, ToPyObject};


/* fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;

    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }
    Ok(total)
} */

fn swap_vect(list: &mut Vec<i64>, a: usize, b: usize)
{
    unsafe {
        let pa: *mut i64 = &mut list[a];
        let pb: *mut i64 = &mut list[b];
        ptr::swap(pa, pb);
    }
}

fn convert_obj_int(_py: Python, vect: &Vec<PyObject>) -> Vec<i64>
{
    let mut new_vect: Vec<i64> = Vec::new();

    for val in vect.iter()
    {
        let tmp: i64 = val.extract(_py).unwrap();
        new_vect.push(tmp);
    }

    return new_vect;
}

fn bubble_sort(list: &mut Vec<i64>)
{
    let lenght = list.len();

    for val in 0..lenght - 1
    {
        for val2 in 0..lenght - val - 1
        {
            if list[val2] > list[val2 + 1]
            {
                swap_vect(list, val2, val2 + 1);
            }
        }
    }
}

// idx: PyInt = list[0].extract(_py)?; convert pyobject to int
fn sort_list(_py: Python, obj: PyList) -> PyResult<PyList>
{
    let list: Vec<PyObject> = obj.iter(_py).collect();
    let mut converted = convert_obj_int(_py, &list);
    bubble_sort(&mut converted);
    Ok(converted.to_py_object(_py).into_py_object(_py))
}

fn get_value_from_rust(_py: Python, val: PyDict, tofind: &str) -> PyResult<PyObject>
{
    Ok(val.get_item(_py, tofind).unwrap())
}

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_myrustlib, |py, m| {
    m.add(py, "__doc__", "Libreria en Rust para Python")?;
    m.add(py, "get_value_from_rust", py_fn!(py, get_value_from_rust(val: PyDict, tofind: &str)))?;
    m.add(py, "sort_list", py_fn!(py, sort_list(obj: PyList)))?;
    Ok(())
});
