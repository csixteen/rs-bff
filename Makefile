.PHONY: install test

FILES := $(shell find ./tests/ -name "*.bf" -exec basename -s .bf {} \;)
JOBS := $(addprefix job,${FILES})

test: ${JOBS} ; @echo "[$@] finished!"

${JOBS}: job%: ; cargo run -p bff --release -- -f tests/$*.bf

install:
	cargo install --path .
