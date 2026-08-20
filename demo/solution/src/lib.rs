use pyo3::prelude::*;

/// Convertit du Markdown en HTML.
#[pyfunction]
#[pyo3(signature = (input, *, github_flavored = false))]
fn markdown_to_html(input: &str, github_flavored: bool) -> String {
    let mut options = comrak::Options::default();
    options.extension.strikethrough = github_flavored;
    options.extension.table = github_flavored;
    comrak::markdown_to_html(input, &options)
}

/// Module Python `rusty_md`.
#[pymodule]
fn rusty_md(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(markdown_to_html, m)?)?;
    Ok(())
}
