librs=./rust/target/release/librs.a

compile_n_link:
	cd rust; cargo build --release
	mkdir -p ./c/target
	gcc -shared -fpic c/lib.c $(librs) -o c/target/lib_c.so
	# gcc -shared -o c/target/lib_c.so c/target/main.o
	# gcc c/main.c $(librs) -o c/target/main
	# gcc -nostartfiles -fpic c/lib.c $(librs) -o c/target/lib.so
	# ./c/target/main
