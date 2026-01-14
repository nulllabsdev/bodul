.PHONY: help build test fmt lint clean align-markdown-table-columns

help:
	@echo "Available targets:"
	@echo "  build                      Build the Rust workspace"
	@echo "  test                       Run all tests in the workspace"
	@echo "  fmt                        Format code with rustfmt"
	@echo "  lint                       Check code with clippy"
	@echo "  clean                      Remove build artifacts"
	@echo "  align-markdown-table-columns  Align markdown table columns"

build:
	cargo build --workspace

test:
	cargo test --workspace

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets -- -D warnings

clean:
	cargo clean

align-markdown-table-columns:
	@cd dev/align-markdown-table-columns && go build -o align-markdown-table-columns .
	@./dev/align-markdown-table-columns/align-markdown-table-columns ./docs
