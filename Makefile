BUILD_DIR := build

all: run

compile:
	cargo build --release
	@mkdir -p ${BUILD_DIR}
	cp ./target/release/htodo ${BUILD_DIR}/htodo

run: compile
	@./target/release/htodo

clean:
	-cargo clean
	-rm -rf ${BUILD_DIR}

