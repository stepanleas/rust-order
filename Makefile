DB_URL = postgres://postgres:postgres@localhost:5433/order_db

migration:
	diesel migration generate $(name)

migrate_up:
	diesel migration run --database-url=$(DB_URL)

migrate_down:
	diesel migration revert --database-url=$(DB_URL)

print_schema:
	diesel print-schema --database-url=$(DB_URL) > src/infrastructure/src/schema.rs

cargo_fmt:
	cargo fmt --all

cargo_fmt_check:
	cargo fmt --all -- --check

cargo_fix:
	cargo fix --workspace --allow-dirty --allow-staged

cargo_clippy:
	cargo clippy --all-targets --all-features -- -D warnings

cargo_test:
	cargo test --workspace --all-features -- --nocapture

cargo_test_coverage:
	cargo llvm-cov --workspace --all-features

cargo_clean:
	rm -rf ~/.cargo/registry/index/*

cargo_all_checks: cargo_fmt_check cargo_fix cargo_clippy cargo_test