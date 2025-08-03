lint:
	@cargo fmt
	@cargo clippy

mega_lint:
	@cargo fmt
	@cargo clippy -- -W clippy::pedantic