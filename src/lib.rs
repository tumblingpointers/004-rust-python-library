use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn yell_name() -> PyResult<()> {
    println!("HELLO UTKARSH!");
    Ok(())
}

#[pymodule]
fn utkarshpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(yell_name))?;
    Ok(())
}
