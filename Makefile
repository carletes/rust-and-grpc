TARGET = x86_64-unknown-linux-musl

.PHONY: all

all: debug

.PHONY: debug

debug:
	cargo build --target=$(TARGET)

.PHONY: release

release:
	cargo build --target=$(TARGET) --release
