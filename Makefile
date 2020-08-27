PROJECT = omst
VERSION = $(shell git describe)
FILES = $(shell git ls-files src/ Cargo.toml Cargo.lock README.md LICENSE.md CHANGELOG.md Makefile)

dist: $(PROJECT)-$(VERSION).tar.xz

$(PROJECT)-$(VERSION).tar.xz: $(FILES)
	rm -f $(PROJECT)-$(VERSION).tar.xz
	tar c -f - $(FILES) | xz -9 > $(PROJECT)-$(VERSION).tar.xz
