#!/bin/bash

# BreezeBlogs Performance Benchmark Script (using wrk)
# Tests baseline implementation performance

set -e  # Exit on error

echo "======================================"
echo "BreezeBlogs Performance Testing"
echo "======================================"
echo ""

# Check if wrk is installed
if ! command -v wrk &> /dev/null; then
    echo "wrk is not installed!"
    echo "Install it with:"
    echo "  macOS: brew install wrk"
    echo "  Linux: sudo apt-get install wrk"
    echo "  Or build from: https://github.com/wg/wrk"
    exit 1
fi

# Configuration
DURATION="30s"
THREADS=12
CONNECTIONS=100
HOST="http://localhost:8000"
RESULTS_DIR="../comparison_results/baseline_results"
OUTPUT_PREFIX="baseline"

# Create results directory
mkdir -p $RESULTS_DIR

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to wait for server to be ready
wait_for_server() {
    echo "Waiting for server to start..."
    for i in {1..30}; do
        if curl -s http://localhost:8000/session > /dev/null 2>&1; then
            echo "✓ Server is ready!"
            return 0
        fi
        sleep 1
        echo -n "."
    done
    echo ""
    echo "Server failed to start!"
    exit 1
}

# Function to create test user
setup_test_user() {
    echo ""
    echo "Setting up test user..."
    
    # Register user
    echo "  • Registering user..."
    curl -X POST $HOST/register \
        -H "Content-Type: application/json" \
        -d '{"username":"testuser","email":"test@example.com","password":"password123"}' \
        -s > /dev/null 2>&1 || true
    
    # Login to get cookie
    echo "  • Logging in..."
    curl -X POST $HOST/login \
        -H "Content-Type: application/json" \
        -d '{"email":"test@example.com","password":"password123"}' \
        -c $RESULTS_DIR/cookie.txt \
        -s > /dev/null
    
    # Extract cookie value
    COOKIE=$(grep user_email $RESULTS_DIR/cookie.txt | awk '{print $7}')
    
    # Set some interests
    echo "  • Setting interests..."
    curl -X POST $HOST/interests \
        -H "Content-Type: application/json" \
        -H "Cookie: user_email=$COOKIE" \
        -d '{"interests":["food","fashion"]}' \
        -s > /dev/null
    
    echo "✓ Test user ready!"
}

# Function to benchmark GET endpoint
benchmark_get() {
    local name=$1
    local endpoint=$2
    local needs_auth=$3
    local output_prefix=$4
    
    echo -e "${BLUE}Testing GET: $name${NC}"
    
    if [ "$needs_auth" = "true" ]; then
        wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
            -H "Cookie: user_email=$COOKIE" \
            --latency \
            $HOST$endpoint > $RESULTS_DIR/${output_prefix}_${name}.txt 2>&1
    else
        wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
            --latency \
            $HOST$endpoint > $RESULTS_DIR/${output_prefix}_${name}.txt 2>&1
    fi
    
    echo "  ✓ Complete"
}

# Function to benchmark POST endpoint
benchmark_post() {
    local name=$1
    local endpoint=$2
    local data=$3
    local needs_auth=$4
    local output_prefix=$5
    
    echo -e "${BLUE}Testing POST: $name${NC}"
    
    # Create Lua script for POST
    cat > $RESULTS_DIR/post_${name}.lua << EOF
wrk.method = "POST"
wrk.body = '$data'
wrk.headers["Content-Type"] = "application/json"
EOF

    if [ "$needs_auth" = "true" ]; then
        echo 'wrk.headers["Cookie"] = "user_email='$COOKIE'"' >> $RESULTS_DIR/post_${name}.lua
    fi
    
    wrk -t$THREADS -c$CONNECTIONS -d$DURATION \
        --latency \
        -s $RESULTS_DIR/post_${name}.lua \
        $HOST$endpoint > $RESULTS_DIR/${output_prefix}_${name}.txt 2>&1
    
    echo "  ✓ Complete"
}

# ==========================================
# MAIN EXECUTION
# ==========================================

# Check if server is already running
if curl -s http://localhost:8000/session > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Server already running on port 8000${NC}"
    echo "Do you want to:"
    echo "  1) Use existing server"
    echo "  2) Kill and restart server"
    echo "  3) Exit"
    read -p "Choose (1/2/3): " choice
    
    case $choice in
        1)
            echo "Using existing server..."
            ;;
        2)
            echo "Killing existing server..."
            pkill -f "cargo run" || true
            sleep 2
            ;;
        3)
            echo "Exiting..."
            exit 0
            ;;
        *)
            echo "Invalid choice, exiting..."
            exit 1
            ;;
    esac
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Testing baseline Version${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Build and start baseline server if not running
if ! curl -s http://localhost:8000/session > /dev/null 2>&1; then
    echo "Building baseline version..."
    cargo build --release

    echo "Starting server..."
    cargo run --release > $RESULTS_DIR/server.log 2>&1 &
    SERVER_PID=$!
    
    # Wait for server
    wait_for_server
else
    echo "Using existing server"
    SERVER_PID=""
fi

# Setup test data
setup_test_user

echo ""
echo -e "${GREEN}Running benchmarks (this will take ~5 minutes)...${NC}"
echo ""

# Test all endpoints
benchmark_post "register" "/register" \
    '{"username":"newuser","email":"new@example.com","password":"pass123"}' \
    "false" "baseline" "$OUTPUT_PREFIX"

benchmark_post "login" "/login" \
    '{"email":"test@example.com","password":"password123"}' \
    "false" "baseline" "$OUTPUT_PREFIX"

benchmark_get "session" "/session" "true" "baseline" "$OUTPUT_PREFIX"

benchmark_get "get_interests" "/interests" "true" "baseline" "$OUTPUT_PREFIX"

benchmark_post "set_interests" "/interests" \
    '{"interests":["food", "fashion"]}' \
    "true" "baseline" "$OUTPUT_PREFIX"

benchmark_get "get_blog_posts" "/blog-posts" "true" "baseline" "$OUTPUT_PREFIX"

benchmark_post "set_email" "/email" \
    '{"email":"newsletter@example.com"}' \
    "true" "baseline" "$OUTPUT_PREFIX"

benchmark_get "send_news_mails" "/send-news-mails" "false" "baseline" "$OUTPUT_PREFIX"

benchmark_post "logout" "/logout" \
    '{}' \
    "true" "baseline" "$OUTPUT_PREFIX"

# Stop server if we started it
if [ ! -z "$SERVER_PID" ]; then
    echo ""
    echo "Stopping server..."
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Benchmark Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Results saved to: $RESULTS_DIR/"
echo ""
echo "Sample result:"
echo "----------------------------------------"
head -20 $RESULTS_DIR/baseline_session.txt
echo "----------------------------------------"
