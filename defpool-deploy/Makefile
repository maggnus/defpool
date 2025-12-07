.PHONY: up down build logs db db-stop db-logs backup restore

# Start all services
up:
	docker-compose up --build -d

# Stop all services
down:
	docker-compose down

# Build all services
build:
	docker-compose build

# View logs for all services
logs:
	docker-compose logs -f

# Start only the database
db:
	docker-compose up -d postgres

# Stop the database
db-stop:
	docker-compose stop postgres

# View database logs
db-logs:
	docker-compose logs -f postgres

# Backup database
backup:
	./scripts/backup.sh

# Restore database
restore:
	@echo "Usage: make restore BACKUP=<backup-file>"
	@if [ -z "$(BACKUP)" ]; then \
		echo "Error: Please specify BACKUP file"; \
		echo "Example: make restore BACKUP=./backups/defpool_backup_20231207.sql.gz"; \
		exit 1; \
	fi
	./scripts/restore.sh $(BACKUP)
