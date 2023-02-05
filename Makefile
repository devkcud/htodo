all: run

compile:
	@cargo build --release

run: compile
	@./target/release/htodo
