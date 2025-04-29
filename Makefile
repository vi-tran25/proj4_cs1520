.ONESHELL:

.PHONY: setup_backend
setup_backend:
	cd backend/my_budget
	cargo build

.PHONY: initdb
initdb:
	cd backend/my_budget
	rm -f budget.db
	
.PHONY: run_backend
run_backend:
	cd backend/my_budget
	RUST_LOG=debug cargo run

.PHONY: setup_frontend
setup_frontend:
	cd frontend/my-budget-app
	npm install

.PHONY: run_frontend
run_frontend:
	cd frontend/my-budget-app
	npm run dev
