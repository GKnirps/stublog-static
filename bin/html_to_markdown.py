#!/usr/bin/env python3

# This file is part of stublog-static.
#
#  stublog-static is free software: you can redistribute it and/or modify
#  it under the terms of the GNU Affero General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  stublog-static is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
#  GNU Affero General Public License for more details.
#
#  You should have received a copy of the GNU Affero General Public License
#  along with stublog-static. If not, see <https://www.gnu.org/licenses/>.

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


