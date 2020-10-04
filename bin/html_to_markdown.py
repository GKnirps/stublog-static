#!/usr/bin/env python3

# this is a script to convert the body part of the file format
# this blog generator uses from html to markdown.
# is it barely tested and will not handle every case correctly.
# Use with caution! Check the output manually!

import sys
from markdownify import markdownify as md

def file_to_markdown(filename):
    content_areas = []
    with open(filename) as f:
        raw_content = f.read()
        content_areas = raw_content.split("---\n", 2)
        if len(content_areas) != 3:
            raise ValueError("Expected a header and a body")
    content_areas[2] = md(content_areas[2], bullets="-*+")
    with open(filename, "w") as f:
        f.write("---\n".join(content_areas))

if len(sys.argv) < 2:
    raise ValueError("Expected at least one argument")

for filename in sys.argv[1:]:
    file_to_markdown(filename)


