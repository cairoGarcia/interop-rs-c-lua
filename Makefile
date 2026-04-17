librs=./rust/target/release/

compile_n_link:
	cd rust; cargo build --release
	mkdir -p ./c/target
	gcc c/main.c $(librs)librs.a -o c/target/main
	./c/target/main
