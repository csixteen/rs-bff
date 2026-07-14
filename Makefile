.PHONY: install test flamegraph

FILES := $(shell find ./tests/ -name "*.bf" -exec basename -s .bf {} \;)
JOBS := $(addprefix job,${FILES})

test: ${JOBS} ; @echo "[$@] finished!"

${JOBS}: job%: ; cargo run -p bff --release -- -f tests/$*.bf

install:
	cargo install --locked --profile release --path crates/bff
	cargo install --locked --profile release --path crates/bff-tui

build-release-with-debug:
	cargo build --locked -p bff --profile release-with-debug

FILE_PREFIX := $(shell date "+%Y%m%d_%H%M%S")
flamegraph: build-release-with-debug
	flamegraph -o $(addprefix $(FILE_PREFIX), _bff.svg) -- target/release-with-debug/bff -f tests/beer.bf
