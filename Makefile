SHELL := /bin/bash
.PHONY: help

all: b ## Build the project using cargo

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

b: ## Build the project using cargo
	cargo build

br: ## Build the project using release profile using cargo
	cargo build --release

test: ## Test the project using cargo
	cargo test

r: ## Run the project using debug profile using cargo
	cargo run

rr: ## Run the project using release profile using cargo
	cargo run --release

lint: ## Lint the project using cargo
# @rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
# @rustup component add rustfmt 2> /dev/null
	cargo fmt

cov-html: ## Generate coverage report using llvm
	@cargo llvm-cov --html --ignore-filename-regex='main.rs'
	open target/llvm-cov/html/index.html

cov: ## Generate coverage summary to terminal using llvm
	@cargo llvm-cov --summary-only --ignore-filename-regex='main.rs'

bump: ## Bump the version number
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"

release: ## Release the project using cargo
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"
	cargo publish --dry-run
	@read -p "Do you want to publish the release? (y/n) " answer; \
	if [ "$$answer" = "y" ]; then \
		cargo publish; \
	else \
		echo "Release not published"; \
	fi
	@echo "Release process completed"
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"
	@echo "Release process completed"

bumptag: ## Bump and tag the release both locally and on remote
	@git diff --quiet || (echo "Uncommitted changes present. Please commit or stash them before tagging." && exit 1)
	# Make sure it builds without any error before tagging
	@cargo clean
	@cargo build --release || (echo "Build failed! Aborting tag process." && exit 1)
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"
	sleep 2
	git add Cargo.toml Cargo.lock
	git commit -m "Bump version to v$$(cargo pkgid | cut -d# -f2)"
	@git push origin develop
	git tag -a v$$(cargo pkgid | cut -d# -f2) -m "Release v$$(cargo pkgid | cut -d# -f2)"
	git push origin v$$(cargo pkgid | cut -d# -f2)
	@echo "Tag v$$(cargo pkgid | cut -d# -f2) created and pushed to remote"
	@echo "Release process completed"
