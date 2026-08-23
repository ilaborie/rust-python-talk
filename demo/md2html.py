# import maturin_import_hook
# maturin_import_hook.install()

import time
import md

content = open("test.md").read()

start = time.perf_counter()
result = md.to_html(content, True)
end = time.perf_counter()
print(f"Elapsed time: {(end - start) * 1000:.6f} ms")


with open("result.html", 'w+') as out:
    out.write(result)
