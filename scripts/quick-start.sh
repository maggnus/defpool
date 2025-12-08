#!/bin/bash
# DefPool Quick Start Script
# This script sets up and starts DefPool for local development/testing

set -e

echo "🚀 DefPool Quick Start"
echo "====================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v docker &> /dev/null; then
    echo -e "${RED}✗ Docker not found. Please install Docker first.${NC}"
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo -e "${RED}✗ Docker Compose not found. Please install Docker Compose first.${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Cargo not found. Please install Rust first.${NC}"
    exit 1
fi

if ! command -v psql &> /dev/null; then
    echo -e "${YELLOW}⚠ psql not found. Database migrations will be skipped.${NC}"
    SKIP_MIGRATIONS=1
fi

echo -e "${GREEN}✓ All prerequisites met${NC}"
echo ""

# Start database
echo "🗄️  Starting PostgreSQL database..."
cd defpool-deploy
docker-compose up -d postgres
echo -e "${GREEN}✓ Database started${NC}"
echo ""

# Wait for database to be ready
echo "⏳ Waiting for database to be ready..."
sleep 5

# Run migrations
if [ -z "$SKIP_MIGRATIONS" ]; then
    echo "📊 Running database migrations..."
    export DATABASE_URL="postgres://defpool:defpool_dev_password@localhost:5432/defpool"
    
    if psql $DATABASE_URL -c "SELECT 1" &> /dev/null; then
        cd ../defpool-server
        
        # Check if migrations already applied
        if psql $DATABASE_URL -c "SELECT 1 FROM miners LIMIT 1" &> /dev/null 2>&1; then
            echo -e "${YELLOW}⚠ Migrations already applied${NC}"
        else
            psql $DATABASE_URL < src/db/migrations/001_initial_schema.sql
            psql $DATABASE_URL < src/db/migrations/002_payout_system.sql
            echo -e "${GREEN}✓ Migrations applied${NC}"
        fi
        cd ..
    else
        echo -e "${RED}✗ Could not connect to database${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠ Skipping migrations (psql not found)${NC}"
fi
echo ""

# Build server
echo "🔨 Building DefPool Server..."
cd defpool-server
if cargo build --release --quiet; then
    echo -e "${GREEN}✓ Server built${NC}"
else
    echo -e "${RED}✗ Server build failed${NC}"
    exit 1
fi
cd ..
echo ""

# Build proxy
echo "🔨 Building DefPool Proxy..."
cd defpool-proxy
if cargo build --release --quiet; then
    echo -e "${GREEN}✓ Proxy built${NC}"
else
    echo -e "${RED}✗ Proxy build failed${NC}"
    exit 1
fi
cd ..
echo ""

# Start server in background
echo "🚀 Starting DefPool Server..."
cd defpool-server
nohup cargo run --release > ../defpool-server.log 2>&1 &
SERVER_PID=$!
echo $SERVER_PID > ../defpool-server.pid
cd ..
echo -e "${GREEN}✓ Server started (PID: $SERVER_PID)${NC}"
echo ""

# Wait for server to be ready
echo "⏳ Waiting for server to be ready..."
sleep 5

# Check if server is responding
if curl -s http://localhost:3000/api/v1/target > /dev/null; then
    echo -e "${GREEN}✓ Server is responding${NC}"
else
    echo -e "${RED}✗ Server not responding${NC}"
    echo "Check logs: tail -f defpool-server.log"
    exit 1
fi
echo ""

# Start proxy in background
echo "🚀 Starting DefPool Proxy..."
cd defpool-proxy
nohup cargo run --release > ../defpool-proxy.log 2>&1 &
PROXY_PID=$!
echo $PROXY_PID > ../defpool-proxy.pid
cd ..
echo -e "${GREEN}✓ Proxy started (PID: $PROXY_PID)${NC}"
echo ""

# Wait for proxy to be ready
echo "⏳ Waiting for proxy to be ready..."
sleep 3

# Check if proxy is listening
if netstat -an 2>/dev/null | grep -q ":3333.*LISTEN" || lsof -i :3333 &>/dev/null; then
    echo -e "${GREEN}✓ Proxy is listening${NC}"
else
    echo -e "${YELLOW}⚠ Could not verify proxy is listening${NC}"
fi
echo ""

# Display status
echo "✅ DefPool is running!"
echo ""
echo "📊 Status:"
echo "  Server:   http://localhost:3000"
echo "  Proxy:    stratum+tcp://localhost:3333"
echo "  Database: localhost:5432"
echo ""
echo "📝 Logs:"
echo "  Server: tail -f defpool-server.log"
echo "  Proxy:  tail -f defpool-proxy.log"
echo ""
echo "🛑 To stop:"
echo "  ./scripts/stop.sh"
echo ""
echo "🧪 To test with xmrig:"
echo "  xmrig -o localhost:3333 -u YOUR_WALLET_ADDRESS -p x"
echo ""
echo "📚 API Documentation:"
echo "  GET  /api/v1/target           - Current mining target"
echo "  GET  /api/v1/targets          - All targets with profitability"
echo "  GET  /api/v1/miners/{wallet}/stats    - Miner statistics"
echo "  GET  /api/v1/miners/{wallet}/balances - Miner balances"
echo ""
