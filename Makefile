.PHONY: test

FILES := $(shell find ./tests/ -name "*.bf" -exec basename -s .bf {} \;)
JOBS := $(addprefix job,${FILES})

test: ${JOBS} ; @echo "[$@] finished!"

${JOBS}: job%: ; cargo run tests/$*.bf
