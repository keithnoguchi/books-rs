# SPDX-License-Identifier: GPL-2.0
WAT := checkers
WAT += checkers_test
.PHONY: build check test clean run install update doc doc-all fmt lint
all: fmt lint test
build:
	@cd flatbuf/schema && flatc -r *.fbs
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
	@cargo run --package the-book --example ch10-largest
	@cargo run --package the-book --example ch10-point
	@cargo run --package the-book --example ch10-tweet
	@cargo run --package the-book --example ch12 SPDX Makefile
	@cargo run --package the-book --example ch13-01-cacher
	@cargo run --package the-book --example ch13-02-iter
	@cargo run --package the-book --example ch13-02-iter-mut
	@cargo run --package the-book --example ch13-02-into-iter
	@cargo run --package the-book --example ch13-02-map
	@cargo run --package the-book --example ch13-02-counter
	@cargo run --package the-book --example ch13-03-grep ch13 Makefile
	@cargo run --package the-book --example ch15-01-list
	@cargo run --package the-book --example ch15-02-graph
	@cargo run --package the-book --example ch15-03-cell
	@cargo run --package the-book --example ch15-04-cycle
	@cargo run --package the-book --example ch15-05-tree
	@cargo run --package the-book --example ch16-01-thread
	@cargo run --package the-book --example ch16-02-channel
	@cargo run --package the-book --example ch16-03-counter
	@cargo run --package the-book --example ch17-01-gui
	@cargo run --package the-book --example ch17-02-blog
	@cargo run --package the-book --example ch17-03-blog2
	@cargo run --package the-book --example ch20
	@cargo run --bin style-book
	@cargo run --package async-std-book --example ch02-02-cat -- Cargo.toml
	@cargo run --package tokio-book --example spawn
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
