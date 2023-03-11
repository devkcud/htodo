BUILD_DIR := build

all: compile

compile:
	cargo build --release
	@mkdir -p ${BUILD_DIR}
	cp ./target/release/htodo ${BUILD_DIR}/htodo

clean:
	-cargo clean
	-rm -rf ${BUILD_DIR}
	-rm -rf ~/.config/htodo/

