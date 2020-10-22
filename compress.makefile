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

GZ = zopfli --gzip
BR = brotli --keep --best --force --no-copy-stat

.PHONY: compress
compress: dist/home.html.gz dist/home.html.br dist/404.html.gz dist/404.html.br $(patsubst %.html, %.html.gz, $(wildcard dist/*/*.html)) $(patsubst %.html, %.html.br, $(wildcard dist/*/*.html)) dist/assets/style.css.gz dist/assets/style.css.br dist/feed.atom.gz dist/feed.atom.br

dist/home.html.gz: dist/home.html
	$(GZ) dist/home.html

dist/home.html.br: dist/home.html
	$(BR) dist/home.html

dist/404.html.gz: dist/404.html
	$(GZ) dist/404.html

dist/404.html.br: dist/404.html
	$(BR) dist/404.html

dist/blogposts/%.html.gz: dist/blogposts/%.html
	$(GZ) $<

dist/blogposts/%.html.br: dist/blogposts/%.html
	$(BR) $<

dist/quote/%.html.gz: dist/quote/%.html
	$(GZ) $<

dist/quote/%.html.br: dist/quote/%.html
	$(BR) $<

dist/quotes/%.html.gz: dist/quotes/%.html
	$(GZ) $<

dist/quotes/%.html.br: dist/quotes/%.html
	$(BR) $<

dist/categories/%.html.gz: dist/categories/%.html
	$(GZ) $<

dist/categories/%.html.br: dist/categories/%.html
	$(BR) $<

dist/archive/%.html.gz: dist/archive/%.html
	$(GZ) $<

dist/archive/%.html.br: dist/archive/%.html
	$(BR) $<

dist/tags/%.html.gz: dist/tags/%.html
	$(GZ) $<

dist/tags/%.html.br: dist/tags/%.html
	$(BR) $<

dist/assets/%.gz: dist/assets/%
	$(GZ) $<

dist/assets/%.br: dist/assets/%
	$(BR) $<

dist/feed.atom.gz: dist/feed.atom
	$(GZ) $<

dist/feed.atom.br: dist/feed.atom
	$(BR) $<

dist/files_metadata/%.gz: dist/files_metadata/%
	$(GZ) $<

dist/files_metadata/%.br: dist/files_metadata/%
	$(BR) $<
