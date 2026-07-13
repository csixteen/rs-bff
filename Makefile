.PHONY: install test flamegraph

FILES := $(shell find ./tests/ -name "*.bf" -exec basename -s .bf {} \;)
JOBS := $(addprefix job,${FILES})

test: ${JOBS} ; @echo "[$@] finished!"

${JOBS}: job%: ; cargo run -p bff --release -- -f tests/$*.bf

install:
	cargo install --locked --profile release --path crates/bff
	cargo install --locked --profile release --path crates/bff-tui

build-bench:
	cargo build --locked -p bff --profile bench

FILE_PREFIX := $(shell date "+%Y%m%d_%H%M%S")
flamegraph: build-bench
	flamegraph -o $(addprefix $(FILE_PREFIX), _bff.svg) -- target/release/bff -f tests/beer.bf
