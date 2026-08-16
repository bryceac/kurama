prefix ?= /usr/local
bindir = $(prefix)/bin
SYS := $(shell $(CC) -dumpmachine)

build:
	cargo build --release
install: build
ifneq (, $(findstring darwin, $(SYS)))
	install "target/release/kurama" "$(bindir)/kurama"
else
	install -D "target/release/kurama" "$(bindir)/kurama"
endif
uninstall:
	rm -rf "$(bindir)/kurama"
clean:
	rm -rf target
.PHONY: build install uninstall clean
