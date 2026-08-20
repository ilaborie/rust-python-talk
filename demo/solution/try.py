from rusty_md import markdown_to_html

print(markdown_to_html("# Hello **world**"))
print(markdown_to_html("~~barré~~", github_flavored=True))
