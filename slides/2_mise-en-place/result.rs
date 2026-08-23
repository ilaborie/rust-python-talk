/// Markdown module
#[pyo3::pymodule]
mod md {
    use pyo3::prelude::*;
    // ...

    /// Render markdown content into HTML
    #[pyfunction(signature = (md:"str", gfm = false))]
    pub fn to_html(md: &str, gfm: bool) -> PyResult<String> {
        let mut options = Options::default();
        let mut plugins = Plugins::default();
        // ...
        let result = comrak::markdown_to_html_with_plugins(md, &options, &plugins);
        Ok(result)
    }
}
