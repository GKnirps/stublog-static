.PHONY: all
all: directories compress

.PHONY: directories
directories:
	mkdir -p dist/assets

.PHONY: generate-html
generate-html:
	cd generator && cargo run --release ../content ../dist

### Assets
.PHONY: assets
assets: dist/assets/favicon.png dist/assets/style.css

dist/assets/favicon.png: assets/favicon.png
	cp assets/favicon.png dist/assets/favicon.png

dist/assets/style.css: $(wildcard assets/stylesheets/*.scss)
	sassc -t compressed assets/stylesheets/style.css.scss dist/assets/style.css

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

.PHONY: clean clean_dist
clean: clean-dist
	-rm -r generator/target

clean-dist:
	-rm -r dist
###
