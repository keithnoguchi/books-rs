# SPDX-License-Identifier: GPL-2.0
WAT := checkers
WAT += checkers_test
.PHONY: build check test clean run install update doc doc-all fmt lint
all: fmt lint test
build:
	@cd wasm; for i in $(WAT);                              \
		do if ! wat2wasm wast/$$i.wat -o wast/$$i.wasm; \
	        then                                            \
			exit 1;                                 \
		fi;                                             \
	done
check: build
	@cargo check
test: build
	@cargo test
clean:
	@cargo clean
run: build
	@echo 2 | cargo run --package the-book --example ch02
	@cargo run -q --package the-book --example ch10-largest
	@cargo run -q --package the-book --example ch10-point
	@cargo run -q --package the-book --example ch10-tweet
	@cargo run -q --package the-book --example ch12 SPDX Makefile
	@cargo run -q --package the-book --example ch13-01-cacher
	@cargo run -q --package the-book --example ch13-02-iter
	@cargo run -q --package the-book --example ch13-02-iter-mut
	@cargo run -q --package the-book --example ch13-02-into-iter
	@cargo run -q --package the-book --example ch13-02-map
	@cargo run -q --package the-book --example ch13-02-counter
	@cargo run -q --package the-book --example ch13-03-grep ch13 Makefile
	@for i in 1 2 3 4 5 6 7 8; do \
		if ! cargo run -q --package the-book --example ch15-0$$i; then \
			exit 1; \
		fi; \
	done
	@cargo run -q --package the-book --example ch16-01-thread
	@cargo run -q --package the-book --example ch16-02-channel
	@cargo run -q --package the-book --example ch16-03-counter
	@cargo run -q --package the-book --example ch17-01-gui
	@cargo run -q --package the-book --example ch17-02-blog
	@cargo run -q --package the-book --example ch17-03-blog2
	@cargo run -q --package the-book --example ch20
	@cargo run -q --bin style-book
	@cargo run -q --package async-std-book --example ch02-02-cat -- Cargo.toml
	@cargo run -q --package tokio-book --example spawn
install: build
	@for i in book async; do \
		cargo install --force --path $$i; \
	done
update:
	@cargo update
doc:
	@cargo doc --all --open &
doc-all: doc-book doc-std doc
doc-%:
	@rustup doc --$* &
fmt: build
	@rustfmt --edition 2018 --check **/src/*.rs **/examples/*.rs
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
