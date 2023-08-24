.PHONY: all run clean

all: target/debug/my_program

target/debug/my_program:
	cargo build

run: target/debug/my_program
	../target/debug/rustEditor

clean:
	cargo clean

