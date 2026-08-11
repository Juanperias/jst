run-x86_64: build-x86_64-dummy
	cargo r -- dist/x86_64_dummy

build-x86_64-dummy: dirt
	nasm -f elf64 dummy/x86_64_dummy.asm -o dist/x86_64_dummy.o
	ld -o dist/x86_64_dummy dist/x86_64_dummy.o

clean:
	rm -rf dist
	cargo clean

# Alice in chains reference!
dirt:
	mkdir -p dist

