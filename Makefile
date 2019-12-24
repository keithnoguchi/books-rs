# SPDX-License-Identifier: GPL-2.0
.PHONY: build check test clean run install update doc doc-all fmt lint
all: fmt lint test
build:
	@cd flatbuf/schema && flatc -r *.fbs
check: build
	@cargo check
test: build
	@cargo test
clean:
	@cargo clean
run: build
	@cargo run --bin style-book
	@cargo run --package the-book --example ch20
install: build
	@for i in book async; do \
		cargo install --force --path $$i; \
	done
update:
	@cargo update
doc:
	@cargo doc --open &
doc-all: doc-book doc-std
	@cargo doc --all --open &
doc-%:
	@rustup doc --$* &
fmt: build
	@rustfmt --edition 2018 --check **/src/*.rs
lint: build
	@cargo clippy -- -D warnings
# CI targets.
.PHONY: arch64 ubuntu64
arch64: arch64-image
	docker run -v $(PWD):/home/build rustbox/$@ make all clean
ubuntu64: ubuntu64-image
	docker run -v $(PWD):/home/build rustbox/$@ make all clean
%-arch64: arch64-image
	docker run -v $(PWD):/home/build rustbox/arch64 make $* clean
%-ubuntu64: ubuntu64-image
	docker run -v $(PWD):/home/build rustbox/ubuntu64 make $* clean
%-image:
	docker build -t rustbox/$* -f Dockerfile.$* .
