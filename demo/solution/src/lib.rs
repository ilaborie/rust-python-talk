/// Markdown module
#[pyo3::pymodule]
mod md {
    use comrak::Options;
    use comrak::options::Plugins;
    use comrak::plugins::syntect::SyntectAdapterBuilder;
    use pyo3::prelude::*;

    /// Render markdown content into HTML
    #[pyfunction(signature = (md:"str", gfm = false))]
    pub fn to_html(md: &str, gfm: bool) -> PyResult<String> {
        let mut options = Options::default();
        options.extension.autolink = gfm;
        options.extension.alerts = gfm;

        let mut plugins = Plugins::default();
        let sh = SyntectAdapterBuilder::new()
            .theme("base16-ocean.light")
            .build();
        plugins.render.codefence_syntax_highlighter = Some(&sh);

        let result = comrak::markdown_to_html_with_plugins(md, &options, &plugins);
        Ok(result)
    }
}
