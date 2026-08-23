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
