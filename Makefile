### Build ###
.PHONY: build-frontend
build-frontend:
	@cd frontend && npm run build

.PHONY: build-backend
build-backend:
	@cd backend && cargo build --release

### Test ###
.PHONY: test-backend
test-backend:
	@cd backend && cargo test --release

.PHONY: e2e
e2e:
	@cd frontend && npm run build:test && npx playwright test

### Run ###
.PHONY: run-frontend
run-frontend:
	@cd frontend && npm run dev

.PHONY: run-backend
run-backend:
	@cd backend && cargo run --bin app --release

### Format & Lint ###
.PHONY: format-frontend
format-frontend:
	@cd frontend && npm run format

.PHONY: format-backend
format-backend:
	@cd backend && cargo fmt

.PHONY: format
format: format-frontend format-backend

### Other ###
.PHONY: schema
schema:
	@cd backend && cargo run --bin gen_schema --release -- ../schema.json
	@cd frontend && npx openapi-typescript ../schema.json --output src/schema.ts --immutable-types

.PHONY: reset-db
reset-db:
	@cd backend && sqlx database drop -y && sqlx database create && sqlx migrate run
