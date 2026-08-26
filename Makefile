# Everything you need to do on your first install
init: submodules install-deps setup-rust

# Downloads the git submodules (image converter)
submodules:
	git submodule update --init --recursive

# Installs all our local helpers
install-deps:
	cd helpers/peat_track_compiler && cargo build --release
	cargo install --path helpers/peat_track_compiler
	cd helpers/rgba8888-to-rgb332 && cargo build --release
	cargo install --path helpers/rgba8888-to-rgb332

# Sets up rust with targeting the ARM cortex m0+ architecture
setup-rust:
	rustup target add thumbv6m-none-eabi

# Instals an optional tui track player for the PEAT format
setup-player:
	cd helpers/peat_track_player && cargo build --release
	cargo install --path helpers/peat_track_player

# Build the ARM ELF for the project
build:
	cd embedded && make build

# Build and upload the ARM UF2 to an attached pico
run:
	cd embedded && make run

# Delete build artifacts
clean:
	cd embedded && cargo clean
