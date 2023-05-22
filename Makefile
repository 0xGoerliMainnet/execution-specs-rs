
PY_EXECUTION_FOLDER=execution-specs/src
RUST_EXECUTION_FORLDER=rust-execution-specs

help: ## This help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: init test docs open-docs

init: ## Init and pull git submodules
	@git submodule update --init --recursive

update:
	@git submodule update --recursive

test: ## Execute rust-execution-specs test
	@cd $(RUST_EXECUTION_FORLDER); cargo test

docs: ## Generate rust-execution-specs docs
	@cd $(RUST_EXECUTION_FORLDER); cargo doc

docs-open: docs ## Open docs
	@x-www-browser target/doc/execution_specs_rs/index.html


exec-python:
	echo 'ethereum.rlp.encode("")'
	cd $(PY_EXECUTION_FOLDER); python -c 'from ethereum.rlp import encode; output = encode(""); print("Output: ", output, "\n")'
	echo "ethereum.rlp.encode(ethereum.base_types.Uint(7))'"
	cd $(PY_EXECUTION_FOLDER); python -c 'from ethereum.rlp import encode; from ethereum.base_types import Uint; output = encode(Uint(7)); print("Output:", output, "\n")'

# quick-test:
# 	cp quick_test.py $(PY_EXECUTION_FOLDER)
# 	cd $(PY_EXECUTION_FOLDER); python quick_test.py
# 	cd rust-execution-specs; cargo test frontier::test_trie::quick_test

clean:
	@rm -rf execution-specs
	@rm -rf tests