mod lib;


fn main() {
    for i in 0..10 {
        println!("Fibonacci({}) = {}", i, lib::fib(i));
    }
}

pub fn fib (n: u32) -> u32 {
    if n <= 0 {
          return 0;
    } else if n== 1{
          return 1;
} else {
    return fib (n-1)  + fib(n-2);
 }
}
#
fn rustpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib, m)?)?;

    Ok(())
}