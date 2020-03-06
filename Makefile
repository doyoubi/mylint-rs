build:
	git submodule update --init
	cargo build

install:
	ln -F -s ${PWD}/target/release/mylint /usr/local/bin/mylint

install_debug:
	ln -F -s ${PWD}/target/debug/mylint /usr/local/bin/mylint

run:
	git submodule update --init
	RUST_BACKTRACE=full cargo run

test:
	git submodule update --init
	RUST_BACKTRACE=full cargo test -- --nocapture

lint:
	git submodule update --init
	find src -name "*.rs" | xargs rustup run stable rustfmt
	cargo clippy

release:
	git submodule update --init
	cargo build --release

.PHONY: build test lint release install install_debug

