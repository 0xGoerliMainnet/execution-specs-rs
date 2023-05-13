
RUST_EXECUTION_FORLDER = rust-execution-specs

help: ## This help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: init test docs open-docs

init: ## Init and pull git submodules
	@git submodule init
	@git submodule update --recursive --remote

test: ## Execute rust-execution-specs test
	cd $(RUST_EXECUTION_FORLDER); cargo test

docs: ## Generate rust-execution-specs docs
	cd $(RUST_EXECUTION_FORLDER); cargo doc

open-docs: docs ## Open docs
	x-www-browser target/doc/rust_execution_specs/index.html
