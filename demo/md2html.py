import md

content = open("test.md").read()

result = md.to_html(content, True)

with open("result.html", 'w+') as out:
    out.write(result)

print(f"Generate {result.__len__()} bytes of HTML")
