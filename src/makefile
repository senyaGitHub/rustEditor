.PHONY: all run clean

all: target/debug/my_program

target/debug/my_program:
	cargo build

run: target/debug/my_program
	../target/debug/rust-editor

clean:
	cargo clean

