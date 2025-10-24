DB_URL = postgres://postgres:postgres@localhost:5433/order_db

migration:
	diesel migration generate $(name)

migrate_up:
	diesel migration run --database-url=$(DB_URL)

migrate_down:
	diesel migration revert --database-url=$(DB_URL)

print_schema:
	diesel print-schema --database-url=$(DB_URL) > src/infrastructure/src/schema.rs
