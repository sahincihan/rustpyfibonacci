use pyo3::prelude::*;

#[pyfunction]
pub fn fib (n: u32) -> u32 {
    if n <= 0 {
          return 0;
    } else if n== 1{
          return 1;
} else {
    return fib (n-1)  + fib(n-2);
 }
}


#[pymodule]
fn fibonacci(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib, m)?)?;

    Ok(())
}



