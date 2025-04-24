use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyList;

// ---------- Globale CKIP‑Instanz (einmaliges Laden) ----------
static WS: Lazy<PyObject> = Lazy::new(|| {
    let res: PyResult<PyObject> = Python::with_gil(|py| {
        let nlp = py.import("ckip_transformers.nlp")?;
        let cls = nlp.getattr("CkipWordSegmenter")?;
        let driver = cls.call1(("bert-base",))?;
        Ok(driver.into_py(py)) // Py<PyAny>
    });
    res.expect("CKIP‑Initialisierung fehlgeschlagen")
});

/// Segmente einen einzelnen Text in Wörter.
pub fn segment(text: &str) -> PyResult<Vec<String>> {
    Python::with_gil(|py| {
        let result: PyObject = WS.call1(py, (PyList::new(py, [text]).unwrap(),)).unwrap();
        println!("result: {result:?}");

        let bound = result.into_bound(py);
        let vals: Vec<Vec<String>> = bound.extract()?;
        //let vals: Vec<String> = bound.get_item(1)?.extract()?;
        // 4. Aus der List[List[str]] das erste Element extrahieren
        //let first: &PyAny = result.as_any().get_item(0)?;
        //first.extract()
        Ok(vals.first().unwrap().to_vec())
    })
}
