.PHONY: up down build logs clean status portal server proxy db db-stop db-logs backup restore help

# Default target - start all services
all: up

# Start all services with build
up:
	@echo "🚀 Starting DefPool services..."
	docker-compose up --build -d
	@echo "✅ All services started!"
	@echo ""
	@echo "🌐 Access Points:"
	@echo "   Web Dashboard: http://localhost:8080"
	@echo "   API Server:     http://localhost:3000"
	@echo "   Mining Proxy:   stratum+tcp://localhost:3333"
	@echo "   Database:       localhost:5432"
	@echo ""
	@echo "📊 View status: make status"
	@echo "📋 View logs:   make logs"

# Stop all services
down:
	@echo "🛑 Stopping DefPool services..."
	docker-compose down
	@echo "✅ All services stopped!"

# Stop services and remove containers/volumes
clean:
	@echo "🧹 Cleaning up DefPool services..."
	docker-compose down -v --remove-orphans
	docker system prune -f
	@echo "✅ Cleanup complete!"

# Build all services
build:
	@echo "🔨 Building DefPool services..."
	docker-compose build --no-cache
	@echo "✅ Build complete!"

# View service status
status:
	@echo "📊 DefPool Service Status:"
	@docker-compose ps
	@echo ""
	@echo "🌐 Service Health:"
	@docker-compose exec postgres pg_isready -U defpool -q && echo "✅ PostgreSQL: Healthy" || echo "❌ PostgreSQL: Unhealthy"
	@curl -s http://localhost:3000/api/v1/targets > /dev/null && echo "✅ API Server: Healthy" || echo "❌ API Server: Unhealthy"
	@curl -s http://localhost:8080 > /dev/null && echo "✅ Portal: Healthy" || echo "❌ Portal: Unhealthy"

# View logs for all services
logs:
	docker-compose logs -f

# View logs for specific services
portal-logs:
	docker-compose logs -f defpool-portal

server-logs:
	docker-compose logs -f defpool-server

proxy-logs:
	docker-compose logs -f defpool-proxy

# Start individual services
portal:
	docker-compose up -d defpool-portal

server:
	docker-compose up -d defpool-server

proxy:
	docker-compose up -d defpool-proxy

# Database management
db:
	@echo "🗄️ Starting PostgreSQL..."
	docker-compose up -d postgres
	@echo "✅ PostgreSQL started!"

db-stop:
	@echo "🗄️ Stopping PostgreSQL..."
	docker-compose stop postgres
	@echo "✅ PostgreSQL stopped!"

db-logs:
	docker-compose logs -f postgres

db-shell:
	docker-compose exec postgres psql -U defpool -d defpool

# Backup and restore
backup:
	@echo "💾 Creating database backup..."
	./scripts/backup.sh

restore:
	@echo "📁 Usage: make restore BACKUP=<backup-file>"
	@if [ -z "$(BACKUP)" ]; then \
		echo "❌ Error: Please specify BACKUP file"; \
		echo "📋 Example: make restore BACKUP=./backups/defpool_backup_20231207.sql.gz"; \
		exit 1; \
	fi
	@echo "📤 Restoring database from $(BACKUP)..."
	./scripts/restore.sh $(BACKUP)

# Development helpers
dev: up
	@echo "🔧 Development mode: Services are running with hot reload"
	@echo "💻 Edit code and see changes automatically"

stop-all:
	@echo "🛑 Stopping all DefPool containers..."
	docker stop $$(docker ps -q --filter "label=com.docker.compose.project=defpool-deploy") 2>/dev/null || true
	@echo "✅ All containers stopped!"

# Help
help:
	@echo "🚀 DefPool Docker Compose Management"
	@echo ""
	@echo "📋 Available commands:"
	@echo "  make up           - Start all services (default)"
	@echo "  make down         - Stop all services"
	@echo "  make clean        - Stop and remove all containers/volumes"
	@echo "  make build        - Build all services"
	@echo "  make status       - Show service status and health"
	@echo "  make logs         - View logs for all services"
	@echo ""
	@echo "🎯 Individual services:"
	@echo "  make portal       - Start only web portal"
	@echo "  make server       - Start only API server"
	@echo "  make proxy        - Start only mining proxy"
	@echo "  make db           - Start only database"
	@echo ""
	@echo "📊 Monitoring:"
	@echo "  make portal-logs  - View portal logs"
	@echo "  make server-logs  - View server logs"
	@echo "  make proxy-logs   - View proxy logs"
	@echo "  make db-logs      - View database logs"
	@echo "  make db-shell     - Open database shell"
	@echo ""
	@echo "💾 Database:"
	@echo "  make backup       - Create database backup"
	@echo "  make restore BACKUP=<file> - Restore from backup"
	@echo ""
	@echo "🛠️ Development:"
	@echo "  make dev          - Start in development mode"
	@echo "  make stop-all     - Emergency stop all containers"
	@echo ""
	@echo "📖 Examples:"
	@echo "  make up && make status    # Start and check status"
	@echo "  make logs                 # Monitor all logs"
	@echo "  make down && make clean   # Full cleanup"
