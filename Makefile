librs=./rust/target/release/librs.a

compile_n_link:
	cd rust; cargo build --release
	gcc -shared -fpic c/lib.c $(librs) -o lua/lib_c.so
	cd lua; lua run.lua;
