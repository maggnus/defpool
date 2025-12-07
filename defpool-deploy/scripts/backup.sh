#!/bin/bash
# DefPool Database Backup Script

set -e

BACKUP_DIR="./backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/defpool_backup_$TIMESTAMP.sql"

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

echo "Starting backup..."
docker exec defpool-postgres pg_dump -U defpool defpool > "$BACKUP_FILE"

# Compress backup
gzip "$BACKUP_FILE"

echo "Backup completed: ${BACKUP_FILE}.gz"

# Keep only last 7 days of backups
find "$BACKUP_DIR" -name "defpool_backup_*.sql.gz" -mtime +7 -delete

echo "Old backups cleaned up"
