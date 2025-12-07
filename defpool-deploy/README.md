# DefPool Deployment

This directory contains all deployment-related files for DefPool.

## Structure

```
defpool-deploy/
├── docker-compose.yml   # Main Docker Compose configuration
├── Makefile             # Build and deployment commands
├── docker/              # Docker-related files
│   ├── Dockerfile.*     # Custom Dockerfiles
│   └── .dockerignore    # Docker ignore patterns
├── scripts/             # Deployment scripts
│   ├── init-db.sql      # Database initialization
│   ├── backup.sh        # Backup script
│   └── restore.sh       # Restore script
├── config/              # Configuration templates
│   ├── .env.example     # Environment variables template
│   └── nginx.conf       # Nginx configuration (if needed)
└── README.md            # This file
```

## Quick Start

### Development

```bash
# Copy environment template
cp env.example .env

# Edit .env with your settings (optional - defaults provided)
vim .env

# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Access Points

- **Web Dashboard**: http://localhost:8080
- **API Server**: http://localhost:3000/api/v1/
- **Mining Proxy**: stratum+tcp://localhost:3333
- **Database**: localhost:5432

### Production

```bash
# Set production environment variables
export DB_PASSWORD="your-secure-password"

# Start services
cd defpool-deploy && docker-compose up -d

# Check health
docker-compose ps
```

## Services

### PostgreSQL
- **Port**: 5432
- **Database**: defpool
- **User**: defpool
- **Data**: Persisted in `postgres_data` volume

### DefPool Server
- **Port**: 3000
- **API**: http://localhost:3000/api/v1/
- **Depends on**: PostgreSQL

### DefPool Proxy
- **Port**: 3333
- **Protocol**: Stratum V1
- **Depends on**: DefPool Server

### DefPool Portal
- **Port**: 8080
- **Web Dashboard**: http://localhost:8080
- **API Proxy**: Routes /api/* to DefPool Server
- **Depends on**: DefPool Server

## Database Management

### Backup
```bash
docker exec defpool-postgres pg_dump -U defpool defpool > backup.sql
```

### Restore
```bash
docker exec -i defpool-postgres psql -U defpool defpool < backup.sql
```

### Connect to Database
```bash
docker exec -it defpool-postgres psql -U defpool -d defpool
```

## Monitoring

### View Logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f defpool-server
docker-compose logs -f defpool-proxy
docker-compose logs -f postgres
```

### Check Health
```bash
# Service status
docker-compose ps

# Database health
docker exec defpool-postgres pg_isready -U defpool
```

## Scaling

### Horizontal Scaling
```bash
# Scale proxy instances
docker-compose up -d --scale defpool-proxy=3
```

### Database Optimization
- Enable connection pooling (PgBouncer)
- Add read replicas
- Enable TimescaleDB for time-series data

## Security

### Production Checklist
- [ ] Change default database password
- [ ] Use secrets management (Docker Secrets, Vault)
- [ ] Enable SSL/TLS for database connections
- [ ] Configure firewall rules
- [ ] Enable database backups
- [ ] Set up monitoring and alerts
- [ ] Use reverse proxy (Nginx) for API
- [ ] Enable rate limiting

## Troubleshooting

### Database Connection Issues
```bash
# Check if PostgreSQL is running
docker-compose ps postgres

# Check logs
docker-compose logs postgres

# Test connection
docker exec defpool-postgres psql -U defpool -d defpool -c "SELECT 1;"
```

### Service Not Starting
```bash
# Check dependencies
docker-compose ps

# View service logs
docker-compose logs [service-name]

# Restart service
docker-compose restart [service-name]
```

## Maintenance

### Update Services
```bash
# Pull latest images
docker-compose pull

# Rebuild and restart
docker-compose up -d --build
```

### Clean Up
```bash
# Remove stopped containers
docker-compose down

# Remove volumes (WARNING: deletes data)
docker-compose down -v

# Clean up Docker system
docker system prune -a
```
