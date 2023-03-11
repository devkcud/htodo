BUILD_DIR := build

all: compile

compile:
	cargo build --release
	@mkdir -p ${BUILD_DIR}
	cp ./target/release/htodo ${BUILD_DIR}/htodo

install: compile
	chmod +x ./scripts/install.sh
	./scripts/install.sh

uninstall:
	-rm ${HOME}/.local/bin/htodo
	-sudo rm /usr/bin/htodo

clean:
	-cargo clean
	-rm -rf ${BUILD_DIR}
	-rm -rf ~/.config/htodo/
	-rm ${HOME}/.local/bin/htodo
	-sudo rm /usr/bin/htodo

