# SPDX-License-Identifier: GPL-2.0
.PHONY: check test clean run install doc fmt lint
all: fmt lint test
check:
	@cargo check
test:
	@cargo test
clean:
	@cargo clean
run:
	@cargo run
install:
	@cargo install --force --path .
doc:
	@cargo doc --all --open
fmt:
	@rustfmt --edition 2018 --check src/*.rs
lint:
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
