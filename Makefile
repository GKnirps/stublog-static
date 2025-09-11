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

.PHONY: all
all: copy-hosted-files compress dist/quotes/strangerthanusual.tar.bz2 | directories

.PHONY: directories
directories:
	mkdir -p dist/assets
	mkdir -p dist/file

# We explicitly name the quote fortune-DB as target here as well, so the dat-file gets updates
# when a quote changes
.PHONY: generate-html
dist/quotes/strangerthanusual generate-html: assets
	cd generator && cargo run --release ../content ../dist

### content files (such as images used in posts)

.PHONY: copy-hosted-files
copy-hosted-files: $(patsubst content/file/%, dist/file/%, $(wildcard content/file/*))

dist/file/%: content/file/% | directories
	cp "$<" "$@"
###

### fortune cookies

dist/quotes/strangerthanusual.dat: dist/quotes/strangerthanusual
	strfile -s dist/quotes/strangerthanusual

dist/quotes/strangerthanusual.tar.bz2: dist/quotes/strangerthanusual.dat dist/quotes/strangerthanusual
	tar -C dist/quotes -cjf dist/quotes/strangerthanusual.tar.bz2 strangerthanusual strangerthanusual.dat

### end fortune cookies

### Assets
.PHONY: assets
assets: dist/assets/favicon.png dist/assets/style.css dist/robots.txt

dist/assets/favicon.png: assets/favicon.png
	cp assets/favicon.png dist/assets/favicon.png

dist/assets/style.css: $(wildcard assets/stylesheets/*.scss)
	sassc -t compressed assets/stylesheets/style.css.scss dist/assets/style.css

dist/robots.txt: assets/robots.txt
	cp assets/robots.txt dist/robots.txt

### end assets

### pre-compress files for http-transport
.PHONY: compress
compress: generate-html assets
	# I was told makefile recursion "is considered harmful"™
	# However, since dependencies are evaluated before any targets are created, if I define the
	# compress targets in this file, the dependencies for the target may not have been created yet,
	# so we don't know which files needs to be compressed. It's complicated.
	$(MAKE) -f compress.makefile compress

### cleanup

.PHONY: clean clean-dist
clean: clean-dist
# -f to ignore errors if the directory does not exist
	-rm -rf generator/target

clean-dist:
# -f to ignore errors if the directory does not exist
	-rm -rf dist

clean-html:
	find dist/ -name "*.html" -execdir rm {} +
###
