.PHONY: all run clean

all: target/debug/rustEditor

target/debug/rustEditor:
	cargo build

run: target/debug/rustEditor
	./target/debug/rustEditor

clean:
	cargo clean

