#!/bin/bash
# DefPool Stop Script

set -e

echo "🛑 Stopping DefPool..."
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Stop server
if [ -f defpool-server.pid ]; then
    SERVER_PID=$(cat defpool-server.pid)
    if kill -0 $SERVER_PID 2>/dev/null; then
        echo "Stopping server (PID: $SERVER_PID)..."
        kill $SERVER_PID
        rm defpool-server.pid
        echo -e "${GREEN}✓ Server stopped${NC}"
    else
        echo -e "${YELLOW}⚠ Server not running${NC}"
        rm defpool-server.pid
    fi
else
    echo -e "${YELLOW}⚠ Server PID file not found${NC}"
fi

# Stop proxy
if [ -f defpool-proxy.pid ]; then
    PROXY_PID=$(cat defpool-proxy.pid)
    if kill -0 $PROXY_PID 2>/dev/null; then
        echo "Stopping proxy (PID: $PROXY_PID)..."
        kill $PROXY_PID
        rm defpool-proxy.pid
        echo -e "${GREEN}✓ Proxy stopped${NC}"
    else
        echo -e "${YELLOW}⚠ Proxy not running${NC}"
        rm defpool-proxy.pid
    fi
else
    echo -e "${YELLOW}⚠ Proxy PID file not found${NC}"
fi

# Stop database (optional)
read -p "Stop database? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Stopping database..."
    cd defpool-deploy
    docker-compose down
    cd ..
    echo -e "${GREEN}✓ Database stopped${NC}"
fi

echo ""
echo "✅ DefPool stopped"
