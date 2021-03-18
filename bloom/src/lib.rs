
extern crate bloom;

use pyo3::prelude::*;

use bloom::{ASMS, BloomFilter};

#[pyclass]
struct PyBloomFilter {
    inner: BloomFilter
}


#[pymethods]
impl PyBloomFilter {
    #[new]
    pub fn new() -> Self {
        PyBloomFilter {
            inner: BloomFilter::with_rate(0.01,100)
        }
    }

    pub fn insert(&mut self, value: usize) {
        self.inner.insert(&value);
    }

    pub fn contains(&self, value: usize) -> bool {
        self.inner.contains(&value)
    }
}

#[pymodule]
fn bloom(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBloomFilter>()?;

    Ok(())
}

