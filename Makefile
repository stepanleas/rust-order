DB_URL = postgres://postgres:postgres@localhost:5433/order_db

migrate_up:
	diesel migration run --database-url=$(DB_URL)

migrate_down:
	diesel migration revert --database-url=$(DB_URL)

print_schema:
	diesel print-schema > src/infrastructure/src/schema.rs --database-url=$(DB_URL)
