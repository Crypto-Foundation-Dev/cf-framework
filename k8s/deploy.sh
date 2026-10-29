#!/bin/bash

# Deployment script for framework to k3s
# This script helps you deploy the framework application to your k3s cluster

set -e

echo "======================================"
echo "framework k3s Deployment Script"
echo "======================================"
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found. Please install kubectl first."
    exit 1
fi

# Check cluster connectivity
echo "🔍 Checking k3s cluster connectivity..."
if ! kubectl cluster-info &> /dev/null; then
    echo "❌ Cannot connect to k3s cluster. Please check your kubeconfig."
    exit 1
fi
echo "✅ Connected to k3s cluster"
echo ""

# Ask for database configuration
echo "📝 Database Configuration"
echo "-------------------------"
read -p "PostgreSQL service name or host (default: postgres-service): " DB_HOST
DB_HOST=${DB_HOST:-postgres-service}

read -p "PostgreSQL database username (default: postgres): " DB_USER
DB_USER=${DB_USER:-postgres}

read -sp "PostgreSQL database password: " DB_PASS
echo ""

if [ -z "$DB_PASS" ]; then
    echo "❌ Database password is required"
    exit 1
fi

echo ""
echo "🔧 Updating configuration files..."

# Update ConfigMap with database host
sed -i.bak "s/DATABASE_HOST: .*/DATABASE_HOST: \"$DB_HOST\"/" k8s/configmap.yaml && rm k8s/configmap.yaml.bak

# Update Secret with credentials using stringData
cat > k8s/secret.yaml << EOF
apiVersion: v1
kind: Secret
metadata:
  name: framework-secret
  namespace: default
  labels:
    app: framework
type: Opaque
stringData:
  DATABASE_USERNAME: "$DB_USER"
  DATABASE_PASSWORD: "$DB_PASS"
EOF

echo "✅ Configuration updated"
echo ""

# Deploy to k3s
echo "🚀 Deploying to k3s..."
kubectl apply -f k8s/

echo ""
echo "⏳ Waiting for deployment to be ready..."
kubectl rollout status deployment/framework-template --timeout=5m

echo ""
echo "✅ Deployment complete!"
echo ""

# Display status
echo "📊 Deployment Status"
echo "--------------------"
kubectl get pods -l app=framework
echo ""
kubectl get svc framework-service
echo ""
kubectl get ingress framework-ingress

echo ""
echo "🔗 Your application should be available at:"
echo "   http://api.framework.local"
echo ""
echo "📝 To view logs:"
echo "   kubectl logs -l app=framework --tail=100 -f"
echo ""
echo "🔍 To check pod status:"
echo "   kubectl describe pod -l app=framework"
echo ""
