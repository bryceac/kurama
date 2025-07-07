prefix ?= /usr/local
bindir = $(prefix)/bin
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	test ! -d $(resourcedir) && mkdir -p $(resourcedir)

	install "target/release/nikki" "$(bindir)/nikki"
else
	install -D "target/release/nikki" "$(bindir)/nikki"
endif
uninstall:
	rm -rf "$(bindir)/nikki"
clean:
	rm -rf target
.PHONY: build install uninstall clean