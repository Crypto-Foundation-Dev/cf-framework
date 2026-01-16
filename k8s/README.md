# CF Framework - Kubernetes Deployment Guide

This directory contains all Kubernetes manifests needed to deploy the CF Framework application to a k3s cluster.

## File Structure

| File                 | Description                                                      |
| -------------------- | ---------------------------------------------------------------- |
| `configmap.yaml`     | Non-sensitive configuration (database host, ports, API settings) |
| `secret.yaml`        | Sensitive data (database credentials, S3 keys)                   |
| `deployment.yaml`    | Pod deployment with replicas, health checks, resource limits     |
| `service.yaml`       | ClusterIP service exposing the application                       |
| `ingress.yaml`       | Traefik ingress with TLS for external access                     |
| `kustomization.yaml` | Kustomize configuration for managing all resources               |
| `deploy.sh`          | Deployment helper script                                         |

## Quick Start

### 1. Build and Import Image

```bash
# On your k3s server
cd /path/to/cf-framework
docker build -t cf-framework:latest .

# Import to k3s containerd
docker save cf-framework:latest -o cf-framework.tar
sudo k3s ctr images import cf-framework.tar
```

### 2. Configure Database Connection

Update `configmap.yaml`:

```yaml
data:
  DATABASE_HOST: "postgres-service" # Your PostgreSQL service
  DATABASE_PORT: "5432"
  DATABASE_NAME: "cf_db"
  API_HOST: "0.0.0.0"
  API_PORT: "9773"
```

Update `secret.yaml` with credentials:

```bash
# Generate base64 encoded values
echo -n "your-username" | base64
echo -n "your-password" | base64
```

### 3. Configure S3 Storage

In `secret.yaml`, add your S3-compatible storage credentials:

```yaml
stringData:
  S3_HOST: "https://s3.example.com"
  S3_ACCESS_KEY_ID: "your-access-key"
  S3_SECRET_ACCESS_KEY: "your-secret-key"
  S3_REGION: "us-east-1"
  S3_BUCKET_NAME: "cf-uploads"
```

### 4. Configure TLS Certificate

**Option A: cert-manager (Recommended)**

Install cert-manager:

```bash
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.13.0/cert-manager.yaml
```

Create ClusterIssuer:

```yaml
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: admin@example.com
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
      - http01:
          ingress:
            class: traefik
```

**Option B: Manual TLS Certificate**

```bash
kubectl create secret tls cf-framework-tls-secret \
  --cert=/path/to/tls.crt \
  --key=/path/to/tls.key \
  -n default
```

### 5. Deploy

```bash
# Apply all manifests
kubectl apply -f k8s/

# Or using Kustomize
kubectl apply -k k8s/
```

### 6. Verify Deployment

```bash
# Check pods
kubectl get pods -l app=cf-framework

# Check service
kubectl get svc cf-framework-service

# Check ingress
kubectl get ingress cf-framework-ingress

# View logs
kubectl logs -l app=cf-framework --tail=100 -f

# Test health endpoint
curl -k https://api.example.com/health
```

## Deployment Configuration

### Resource Limits

Current settings in `deployment.yaml`:

```yaml
resources:
  requests:
    memory: "256Mi"
    cpu: "250m"
  limits:
    memory: "512Mi"
    cpu: "500m"
```

### Health Checks

- **Liveness Probe**: `/health` - Restarts container if unhealthy
- **Readiness Probe**: `/health` - Removes from service if not ready
- **Port**: 9773

### Replicas

Default: 2 replicas for high availability

## Environment Variables

| Variable               | Source    | Description         |
| ---------------------- | --------- | ------------------- |
| `DATABASE_HOST`        | ConfigMap | PostgreSQL hostname |
| `DATABASE_PORT`        | ConfigMap | PostgreSQL port     |
| `DATABASE_NAME`        | ConfigMap | Database name       |
| `DATABASE_USERNAME`    | Secret    | Database user       |
| `DATABASE_PASSWORD`    | Secret    | Database password   |
| `API_HOST`             | ConfigMap | Server bind address |
| `API_PORT`             | ConfigMap | Server port         |
| `S3_HOST`              | Secret    | S3 endpoint URL     |
| `S3_ACCESS_KEY_ID`     | Secret    | S3 access key       |
| `S3_SECRET_ACCESS_KEY` | Secret    | S3 secret key       |
| `S3_REGION`            | Secret    | S3 region           |
| `S3_BUCKET_NAME`       | Secret    | S3 bucket name      |

## Update Deployment

```bash
# 1. Build new image
docker build -t cf-framework:latest .

# 2. Import to k3s (if local)
docker save cf-framework:latest -o cf-framework.tar
sudo k3s ctr images import cf-framework.tar

# 3. Restart deployment
kubectl rollout restart deployment/cf-framework

# 4. Watch rollout
kubectl rollout status deployment/cf-framework
```

## Troubleshooting

### Pods not starting

```bash
kubectl describe pod -l app=cf-framework
kubectl logs -l app=cf-framework --previous
```

### Database connection issues

```bash
# Test from inside pod
kubectl exec -it deployment/cf-framework -- sh
nc -zv $DATABASE_HOST $DATABASE_PORT
```

### Ingress not working

```bash
kubectl describe ingress cf-framework-ingress
kubectl logs -n kube-system -l app.kubernetes.io/name=traefik
```

### TLS certificate issues

```bash
kubectl get certificate -n default
kubectl describe certificate cf-framework-tls-secret
kubectl logs -n cert-manager -l app=cert-manager
```

## Cleanup

```bash
# Remove all resources
kubectl delete -f k8s/
# or
kubectl delete -k k8s/
```

## DNS Configuration

Ensure your DNS points to your k3s cluster:

```
api.example.com → <k3s-cluster-ip>
```
