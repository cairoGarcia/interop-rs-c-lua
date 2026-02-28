lib_rs_path=../rust/target/release/

all:
	gcc main.c -L$(lib_rs_path) -llib_rs -o main
	LD_LIBRARY_PATH=$(lib_rs_path) ./main
