.PHONY: all fmt fmt-check clippy clippy-no-default \
	    check-default check-no-default check-ely check-fabric check-neoforge check-all check \
	    build build-windows dist clean help

all: check

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

clippy:
	cargo clippy --all-targets -- -D warnings

clippy-no-default:
	cargo clippy --all-targets --no-default-features -- -D warnings

check-default:
	cargo check --all-targets

check-no-default:
	cargo check --all-targets --no-default-features

check-ely:
	cargo check --all-targets --no-default-features --features ely

check-fabric:
	cargo check --all-targets --no-default-features --features fabric

check-neoforge:
	cargo check --all-targets --no-default-features --features neoforge

check-all: check-default check-no-default check-ely check-fabric check-neoforge

check: fmt-check clippy clippy-no-default check-all

build:
	cargo build --release

build-windows:
	cargo build --release --target x86_64-pc-windows-gnu

dist: build build-windows
	mkdir -p dist
	cp target/release/rml dist/rml
	cp target/x86_64-pc-windows-gnu/release/rml.exe dist/rml.exe
	@echo "Artifacts in dist/:"
	@ls -la dist/

clean:
	cargo clean
	rm -rf dist

help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "  fmt               Format code"
	@echo "  fmt-check         Check formatting"
	@echo "  clippy            Clippy (default features, -D warnings)"
	@echo "  clippy-no-default Clippy (no features, -D warnings)"
	@echo ""
	@echo "  check-default     cargo check (default: ely + fabric + neoforge)"
	@echo "  check-no-default  cargo check --no-default-features"
	@echo "  check-ely         cargo check with only 'ely'"
	@echo "  check-fabric      cargo check with only 'fabric'"
	@echo "  check-neoforge    cargo check with only 'neoforge'"
	@echo "  check-all         run all of the above check-* targets"
	@echo "  check             fmt-check + clippy + clippy-no-default + check-all"
	@echo ""
	@echo "  build             cargo build --release (Linux)"
	@echo "  build-windows     cargo build --release --target x86_64-pc-windows-gnu"
	@echo "  dist              build + build-windows, then copy into dist/"
	@echo "  clean             cargo clean + rm -rf dist"
	@echo "  help              Show this help"
