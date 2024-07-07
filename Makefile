.PHONY: test build

build:
	cargo build

test: build
	cd writing-a-c-compiler-tests && ./test_compiler ../target/debug/waccrs $(args)