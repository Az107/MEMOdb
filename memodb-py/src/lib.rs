use memodb::MEMOdb;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::sync::{Arc, Mutex};

/// Wrapper en Rust para exponer MEMOdb a Python
#[pyclass]
struct Pymemodb {
    inner: Arc<Mutex<MEMOdb>>,
}

#[pyclass]
struct Collection {
    inner: Arc<Mutex<MEMOdb>>,
    name: String,
}

#[pymethods]
impl Pymemodb {
    #[new]
    fn new() -> Self {
        Pymemodb {
            inner: Arc::new(Mutex::new(MEMOdb::new())),
        }
    }

    fn get_version(&self) -> &str {
        self.inner.lock().unwrap().version
    }

    fn create_collection(&mut self, name: &str) -> PyResult<()> {
        let mut db = self.inner.lock().unwrap();
        let r = db.create_collection(name);
        if r.is_ok() {
            return Ok(());
        } else {
            return Err(PyValueError::new_err("Collection already exists"));
        }
    }

    fn list_collections(&self) -> Vec<String> {
        let db = self.inner.lock().unwrap();
        db.get_collection_list()
    }

    fn get_collection(&mut self, name: &str) -> PyResult<Collection> {
        let mut db = self.inner.lock().unwrap();
        let c = db.get_collection(name);
        if c.is_none() {
            return Err(PyValueError::new_err("error getting collection"));
        }
        let col = Collection {
            inner: Arc::clone(&self.inner),
            name: name.to_string(),
        };
        Ok(col)
    }
}

#[pymethods]
impl Collection {
    fn get(&mut self, k: &str) -> PyResult<String> {
        let mut db = self.inner.lock().unwrap();
        let c = db.get_collection(self.name.as_str());
        if c.is_none() {
            return Err(PyValueError::new_err("error getting collection"));
        }
        let c = c.unwrap();
        let r = c.get(k);
        if r.is_none() {
            return Err(PyValueError::new_err("error getting key"));
        }
        let r = r.unwrap();
        return Ok(r.to_string());
    }

    fn set(&mut self, k: &str, v: &str) -> PyResult<()> {
        let mut db = self.inner.lock().unwrap();
        let c = db.get_collection(self.name.as_str());
        if c.is_none() {
            return Err(PyValueError::new_err("error getting collection"));
        }
        let c = c.unwrap();
        c.add(k, memodb::DataType::Text(v.to_string()));
        return Ok(());
    }
}

#[pymodule]
fn py_memodb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Pymemodb>()?;
    Ok(())
}
