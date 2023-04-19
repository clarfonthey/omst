PROJECT = omst
VERSION = $(shell cargo read-manifest | jq --raw-output .version)
FILES = $(wildcard src/*.rs Cargo.toml Cargo.lock README.md LICENSE.md CHANGELOG.md Makefile)
GITEA_TOKEN = $(shell yq --raw-output .logins[0].token ~/.config/tea/config.yml)

.PHONY: dist upload
.SILENT: upload

dist: $(PROJECT)-v$(VERSION).tar.xz

$(PROJECT)-v$(VERSION).tar.xz: $(FILES)
	rm -f $(PROJECT)-v$(VERSION).tar.xz
	tar --mtime='1970-01-01' --create --file - $(FILES) | xz -9 > $(PROJECT)-v$(VERSION).tar.xz

upload: dist
	echo curl \
		--upload-file $(PROJECT)-v$(VERSION).tar.xz \
		https://vc.ltdk.xyz/api/packages/cli/generic/$(PROJECT)/$(VERSION)/source.tar.xz
	curl \
		--upload-file $(PROJECT)-v$(VERSION).tar.xz \
		https://cli:$(GITEA_TOKEN)@vc.ltdk.xyz/api/packages/cli/generic/$(PROJECT)/$(VERSION)/source.tar.xz
