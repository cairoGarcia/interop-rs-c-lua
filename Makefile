lib_rs_path=./rust/target/release/

compile_n_link:
	cd rust; cargo build --release
	mkdir -p ./c/target
	gcc c/main.c $(lib_rs_path)liblib_rs.a -o c/target/main
	./c/target/main
