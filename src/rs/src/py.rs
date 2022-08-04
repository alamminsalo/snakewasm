use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn state() -> PyResult<Vec<Vec<i8>>> {
    Ok(crate::state())
}

#[pyfunction]
fn state_model() -> PyResult<Vec<Vec<f32>>> {
    Ok(crate::state_model())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn reset(w: usize, h: usize) {
    crate::reset(w as u16, h as u16);
}

#[pyfunction]
fn score() -> PyResult<usize> {
    Ok(crate::snake_len() - 4)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn input(cmd: u8) {
    match cmd {
        0 => crate::snake_up(),
        1 => crate::snake_down(),
        2 => crate::snake_left(),
        _ => crate::snake_right(),
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn input_turn(cmd: u8) {
    match cmd {
        1 => crate::snake_turn_left(),
        2 => crate::snake_turn_right(),
        _ => {}
    }
}
/// Formats the sum of two numbers as string.
#[pyfunction]
fn tick() {
    crate::tick();
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn done() -> PyResult<bool> {
    Ok(crate::is_ended())
}

#[pyfunction]
fn head() -> PyResult<(i16, i16)> {
    Ok(crate::head())
}

#[pyfunction]
fn food() -> PyResult<(i16, i16)> {
    Ok(crate::food())
}

#[pyfunction]
fn tick_count() -> PyResult<usize> {
    Ok(crate::tick_count())
}

#[pyfunction]
fn snake_dir() -> PyResult<u8> {
    Ok(crate::snake_dir())
}

#[pyfunction]
fn snake_body() -> PyResult<Vec<(i16, i16)>> {
    Ok(crate::snake_body())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn snake(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reset, m)?)?;
    m.add_function(wrap_pyfunction!(state, m)?)?;
    m.add_function(wrap_pyfunction!(state_model, m)?)?;
    m.add_function(wrap_pyfunction!(tick, m)?)?;
    m.add_function(wrap_pyfunction!(score, m)?)?;
    m.add_function(wrap_pyfunction!(input, m)?)?;
    m.add_function(wrap_pyfunction!(done, m)?)?;
    m.add_function(wrap_pyfunction!(tick_count, m)?)?;
    m.add_function(wrap_pyfunction!(food, m)?)?;
    m.add_function(wrap_pyfunction!(head, m)?)?;
    m.add_function(wrap_pyfunction!(input_turn, m)?)?;
    m.add_function(wrap_pyfunction!(snake_dir, m)?)?;
    m.add_function(wrap_pyfunction!(snake_body, m)?)?;
    Ok(())
}
