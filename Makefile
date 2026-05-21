RUST_CRATES := server cli bridge

.PHONY: help
help: ## Show this help
	@echo "The Answer Protocol — available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'

.PHONY: install
install: ## Install/fetch all dependencies (Rust crates + GUI npm packages)
	@for c in $(RUST_CRATES); do echo ">> cargo fetch ($$c)"; (cd $$c && cargo fetch); done
	@echo ">> npm install (gui)"
	cd gui && npm install

.PHONY: build
build: ## Compile every component (release-less debug build)
	@for c in $(RUST_CRATES); do echo ">> cargo build ($$c)"; (cd $$c && cargo build); done
	cd gui && npm run build

.PHONY: run-server
run-server: ## Run the TCP game server (127.0.0.1:4000)
	cd server && cargo run

.PHONY: run-client
run-client: ## Run the CLI (terminal) client
	cd cli && cargo run

.PHONY: run-bridge
run-bridge: ## Run the WebSocket<->TCP bridge the GUI needs (127.0.0.1:7878)
	cd bridge && cargo run

.PHONY: run-client-gui
run-client-gui: ## Run the GUI dev server (needs run-server + run-bridge)
	@echo "NOTE: the GUI needs the server (make run-server) and bridge (make run-bridge) running."
	cd gui && npm install
	cd gui && npm run dev

.PHONY: lint
lint: ## Check formatting + clippy on all Rust crates
	@for c in $(RUST_CRATES); do \
		echo ">> fmt check ($$c)"; (cd $$c && cargo fmt --all -- --check) || exit 1; \
		echo ">> clippy ($$c)"; (cd $$c && cargo clippy --all-targets -- -D warnings) || exit 1; \
	done
	cd gui && npm run lint

.PHONY: clean
clean: ## Remove all build artifacts (Rust target/ + GUI dist + node_modules)
	@for c in $(RUST_CRATES); do echo ">> clean ($$c)"; (cd $$c && cargo clean); done
	@echo ">> clean (gui)"
	rm -rf gui/dist gui/dist-ssr gui/node_modules gui/.vite gui/*.local
