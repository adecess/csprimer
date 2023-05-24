import re

with open('simple.css', 'r') as file:
    css_content = file.read()

matches = re.findall(r'(color|background-color):\s*([^;]+);', css_content)

print(matches)
