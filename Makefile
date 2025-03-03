.DEFAULT_GOAL := build

CARGO ?= cargo

build:
	$(CARGO) build --release

run: build
	./target/release/ready_set_boole

test:
	$(CARGO) test

clean:
	$(CARGO) clean

.PHONY: build run test clean

