use pyo3::prelude::*;
use pyo3::Python;

#[pyfunction]
fn add_two_floats(a: f32, b: f32) -> PyResult<f32> {
    return Ok(main_library::add_two_f32(a, b));
}

#[pyfunction]
fn greet_person(person: String) -> PyResult<String> {
    let person = main_library::Person::from_json_str(&person);
    return Ok(main_library::get_greet_string(&person));
}

#[pymodule]
fn python_bridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_two_floats, m)?)?;
    m.add_function(wrap_pyfunction!(greet_person, m)?)?;
    Ok(())
}
