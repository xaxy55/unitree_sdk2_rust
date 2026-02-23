# Unitree SDK2 — root Makefile
#
# Targets for the Rust SDK (unitree_sdk2_rust/) and the
# legacy C++ SDK (old_py/), plus devcontainer helpers.

RUST_DIR  := unitree_sdk2_rust
CPP_DIR   := old_py
EXAMPLE   ?= go2_sport_client

.PHONY: help build test run run-dev update docs clean build-cpp install-cpp

help:
	@echo ""
	@echo "Usage: make <target> [EXAMPLE=<name>]"
	@echo ""
	@echo "Rust SDK targets ($(RUST_DIR)/):"
	@echo "  build        Build the Rust SDK (debug)"
	@echo "  test         Run all Rust tests"
	@echo "  run          Run an example  (default: $(EXAMPLE))"
	@echo "               e.g.  make run EXAMPLE=go2_low_level"
	@echo "  update       Update Rust dependencies (cargo update)"
	@echo "  docs         Generate and open Rust API docs"
	@echo "  clean        Remove all build artifacts"
	@echo ""
	@echo "Legacy C++ SDK targets ($(CPP_DIR)/):"
	@echo "  build-cpp    Configure and build with CMake"
	@echo "  install-cpp  Install the C++ SDK (may require sudo)"
	@echo ""
	@echo "DevContainer targets:"
	@echo "  run-dev      Start the devcontainer via docker-compose"
	@echo ""

# ---------------------------------------------------------------------------
# Rust SDK
# ---------------------------------------------------------------------------

build:
	cd $(RUST_DIR) && cargo build

test:
	cd $(RUST_DIR) && cargo test

run:
	cd $(RUST_DIR) && cargo run --example $(EXAMPLE)

update:
	cd $(RUST_DIR) && cargo update

docs:
	cd $(RUST_DIR) && cargo doc --open

clean:
	cd $(RUST_DIR) && cargo clean
	rm -rf $(CPP_DIR)/build

# ---------------------------------------------------------------------------
# Legacy C++ SDK
# ---------------------------------------------------------------------------

build-cpp:
	mkdir -p $(CPP_DIR)/build
	cd $(CPP_DIR)/build && cmake .. && $(MAKE)

install-cpp:
	mkdir -p $(CPP_DIR)/build
	cd $(CPP_DIR)/build && cmake .. && $(MAKE) install

# ---------------------------------------------------------------------------
# DevContainer
# ---------------------------------------------------------------------------

run-dev:
	docker-compose -f .devcontainer/docker-compose.yml up --build
