#!/bin/bash
# DefPool Database Restore Script

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <backup-file>"
    echo "Example: $0 ./backups/defpool_backup_20231207_120000.sql.gz"
    exit 1
fi

BACKUP_FILE="$1"

if [ ! -f "$BACKUP_FILE" ]; then
    echo "Error: Backup file not found: $BACKUP_FILE"
    exit 1
fi

echo "Restoring from: $BACKUP_FILE"

# Decompress if needed
if [[ "$BACKUP_FILE" == *.gz ]]; then
    echo "Decompressing backup..."
    gunzip -c "$BACKUP_FILE" | docker exec -i defpool-postgres psql -U defpool defpool
else
    docker exec -i defpool-postgres psql -U defpool defpool < "$BACKUP_FILE"
fi

echo "Restore completed successfully"
