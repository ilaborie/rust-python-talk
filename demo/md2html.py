# import maturin_import_hook
# maturin_import_hook.install()

import md

content = open("test.md").read()

result = md.to_html(content, True)

with open("result.html", 'w+') as out:
    out.write(result)
