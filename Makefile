# Makefile for markdown-ppp release checks and development tasks

.PHONY: all check-release check test clippy doc fmt fmt-check fmt-readme build build-release clean help

# Default target
all: check-release

# Main release check target - runs all quality checks
check-release: fmt-check fmt-readme check clippy test doc-strict build-release
	@echo "[OK] All release checks passed!"

# Individual check targets
check:
	@echo "[CHECK] Running cargo check with all features..."
	cargo check --all-targets --all-features
	cargo check --all-targets --all-features --tests
	cargo check --all-targets --all-features --benches

clippy:
	@echo "[CLIPPY] Running clippy with all features..."
	cargo clippy --all-targets --all-features -- -D warnings
	cargo clippy --all-targets --all-features --tests -- -D warnings
	cargo clippy --all-targets --all-features --benches -- -D warnings

test:
	@echo "[TEST] Running tests with all features..."
	cargo test --all-targets --all-features --lib

# Run only doctests (may have issues with features)
test-doc:
	@echo "[TEST] Running doctests..."
	cargo test --all-features --doc

doc:
	@echo "[DOC] Generating documentation with all features..."
	cargo doc --all-features --no-deps

# Strict documentation check
doc-strict:
	@echo "[DOC] Checking for broken documentation links..."
	RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc --all-features --no-deps --quiet

fmt:
	@echo "[FMT] Formatting code..."
	cargo fmt

fmt-check:
	@echo "[FMT] Checking code formatting..."
	cargo fmt --check

fmt-readme:
	@echo "[FMT] Formatting README.md..."
	markdown-tool format ./README.md

build:
	@echo "[BUILD] Building in debug mode with all features..."
	cargo build --all-features

build-release:
	@echo "[BUILD] Building in release mode with all features..."
	cargo build --all-features --release

# Comprehensive test with all feature combinations
test-features:
	@echo "[TEST] Testing with different feature combinations..."
	cargo test --no-default-features
	cargo test --features parser
	cargo test --features printer
	cargo test --features html-printer
	cargo test --features latex-printer
	cargo test --features ast-transform
	cargo test --features ast-serde
	cargo test --all-features

# Security audit
audit:
	@echo "[AUDIT] Running security audit..."
	cargo audit

# Check for outdated dependencies
outdated:
	@echo "[DEPS] Checking for outdated dependencies..."
	cargo outdated

# Generate coverage report (requires cargo-tarpaulin)
coverage:
	@echo "[COV] Generating test coverage report..."
	cargo tarpaulin --all-features --out Html --output-dir coverage

# Benchmark tests (if any exist)
bench:
	@echo "[BENCH] Running benchmarks..."
	cargo bench --all-features

# Clean build artifacts
clean:
	@echo "[CLEAN] Cleaning build artifacts..."
	cargo clean
	rm -rf target/doc
	rm -rf coverage

# Check that Cargo.toml is properly formatted
toml-check:
	@echo "[TOML] Checking Cargo.toml formatting..."
	@if command -v taplo >/dev/null 2>&1; then \
		taplo fmt --check Cargo.toml; \
	else \
		echo "[WARN] taplo not found, skipping TOML formatting check"; \
	fi

# Full pre-release check including external tools
pre-release: check-release audit toml-check
	@echo "[PRE-RELEASE] Running comprehensive pre-release checks..."
	@if command -v cargo-outdated >/dev/null 2>&1; then \
		$(MAKE) outdated; \
	else \
		echo "[WARN] cargo-outdated not found, skipping dependency check"; \
	fi
	@echo "[OK] All pre-release checks completed!"

# Development helpers
dev-setup:
	@echo "[SETUP] Setting up development environment..."
	rustup component add rustfmt clippy
	cargo install cargo-audit cargo-outdated taplo-cli
	@echo "[OK] Development environment ready!"

# Watch for changes and run tests
watch:
	@echo "[WATCH] Watching for changes..."
	@if command -v cargo-watch >/dev/null 2>&1; then \
		cargo watch -x "test --all-features"; \
	else \
		echo "[ERROR] cargo-watch not found. Install with: cargo install cargo-watch"; \
	fi

# Help target
help:
	@echo "[HELP] Available targets:"
	@echo "  check-release  - Run all release quality checks (default)"
	@echo "  check          - Run cargo check with all features"
	@echo "  clippy         - Run clippy linter with all features"
	@echo "  test           - Run unit tests with all features"
	@echo "  test-doc       - Run doctests (may have feature issues)"
	@echo "  test-features  - Test with different feature combinations"
	@echo "  doc            - Generate documentation"
	@echo "  doc-strict     - Generate documentation with strict checks"
	@echo "  fmt            - Format code"
	@echo "  fmt-check      - Check code formatting"
	@echo "  fmt-readme     - Format README.md"
	@echo "  build          - Build in debug mode"
	@echo "  build-release  - Build in release mode"
	@echo "  pre-release    - Comprehensive pre-release checks"
	@echo "  audit          - Run security audit"
	@echo "  outdated       - Check for outdated dependencies"
	@echo "  coverage       - Generate test coverage report"
	@echo "  bench          - Run benchmarks"
	@echo "  clean          - Clean build artifacts"
	@echo "  dev-setup      - Set up development environment"
	@echo "  watch          - Watch for changes and run tests"
	@echo "  help           - Show this help message"
