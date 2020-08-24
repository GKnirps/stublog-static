GZ = gzip --keep --best --force

.PHONY: compress
compress: dist/home.html.gz $(patsubst %.html, %.html.gz, $(wildcard dist/*/*.html)) dist/assets/favicon.png.gz dist/assets/style.css.gz

dist/home.html.gz: dist/home.html
	$(GZ) dist/home.html

dist/blogposts/%.html.gz: dist/blogposts/%.html
	$(GZ) $<

dist/categories/%.html.gz: dist/categories/%.html
	$(GZ) $<

dist/archive/%.html.gz: dist/archive/%.html
	$(GZ) $<

dist/tags/%.html.gz: dist/tags/%.html
	$(GZ) $<

dist/assets/%.gz: dist/assets/%
	$(GZ) $<
