.PHONY: all
all: bootstrap install

.PHONY: install
install:
	cargo build --release && cp target/release/gen-pr /usr/local/bin/gen-pr

.PHONY: bootstrap
bootstrap:
	brew list gh > /dev/null || brew install gh
	brew outdated gh || brew upgrade gh
