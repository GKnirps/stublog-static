GZ = gzip --keep --best --force
BR = brotli --keep --best --force --no-copy-stat

.PHONY: compress
compress: dist/home.html.gz dist/home.html.br $(patsubst %.html, %.html.gz, $(wildcard dist/*/*.html)) $(patsubst %.html, %.html.br, $(wildcard dist/*/*.html)) dist/assets/style.css.gz dist/assets/style.css.br

dist/home.html.gz: dist/home.html
	$(GZ) dist/home.html

dist/home.html.br: dist/home.html
	$(BR) dist/home.html

dist/blogposts/%.html.gz: dist/blogposts/%.html
	$(GZ) $<

dist/blogposts/%.html.br: dist/blogposts/%.html
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
