lib_rs_path=./rust/target/release/

compile_n_link:
	cd rust; cargo build --release
	mkdir -p ./c/target
	gcc ./c/main.c -L$(lib_rs_path) -llib_rs -o ./c/target/main
	LD_LIBRARY_PATH=$(lib_rs_path) ./c/target/main
