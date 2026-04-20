librs=./rust/target/release/librs.a

compile_n_link:
	cd rust; cargo build --release
	mkdir -p ./c/target
	gcc c/main.c $(librs) -o c/target/main
	# gcc -nostartfiles -fpic c/lib.c $(librs) -o c/target/lib.so
	./c/target/main
