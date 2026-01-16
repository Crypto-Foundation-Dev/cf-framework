#!/bin/bash
set -e

echo "======================================"
echo "Starting Application Initialization"
echo "======================================"

# Step 1: Run database migrations
echo ""
echo "Step 1: Running database migrations..."
cd /app
/app/bin/cf-migration

if [ $? -ne 0 ]; then
    echo "❌ Database migration failed!"
    exit 1
fi

echo "✅ Database migrations completed successfully!"

# Step 2: Start the API
cd /app
echo ""
echo "Step 2: Starting API server..."
echo "  - API will start on port 3000"
echo ""
exec /app/bin/cf-api
