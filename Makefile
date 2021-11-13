.PHONY: all
all: install bootstrap

.PHONY: install
install:
	cargo build --release && cp target/release/gen-pr /usr/local/bin/gen-pr

.PHONY: bootstrap
bootstrap:
	brew list gh > /dev/null || brew install gh
	brew list cargo > /dev/null || brew install cargo
	brew outdated gh || brew upgrade gh
	brew outdated cargo || brew upgrade cargo
