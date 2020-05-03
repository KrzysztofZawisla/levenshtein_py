extern crate levenshtein;
use levenshtein::levenshtein;
use pyo3::prelude::*;

#[pyclass]
struct LevenshteinPy {}

#[pymethods]
impl LevenshteinPy {
    #[staticmethod]
    fn filter(input: String, distance: usize, collection: Vec<String>) -> PyResult<Vec<String>> {
        let input_chars_array: Vec<char> = input.chars().collect();
        let mut result_vec = vec![];
        for value in collection.iter() {
            let mut shortened_value = vec![];
            let value_chars_array: Vec<char> = value.chars().collect();
            for (i, _) in input_chars_array.iter().enumerate() {
                if value_chars_array.len() > i {
                    shortened_value.push(value_chars_array[i]);
                } else {
                    break;
                }
            }
            let shortened_value: String = shortened_value.into_iter().collect();
            let counted_distance = levenshtein(&input, &shortened_value);
            if counted_distance <= distance {
                result_vec.push(value.to_owned());
            }
        }
        Ok(result_vec)
    }
}

#[pymodule]
fn levenshtein_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LevenshteinPy>()?;
    Ok(())
}
