pipeline {
    agent { label 'staging' }

    environment {
        DEPLOYMENT_NAME = 'framework-template'
        NAMESPACE = 'default'
        IMAGE_TAG = "framework-template:${BUILD_NUMBER}"
        IMAGE_FILE = "framework-template-${BUILD_NUMBER}.tar"
        KUBECONFIG = '/etc/rancher/k3s/k3s.yaml'
        
        // Jenkins credentials bindings (Replace with your own credentials)
        APP_DATABASE_URL = credentials('db-url')
        DATABASE_USERNAME = credentials('db-username')
        DATABASE_PASSWORD = credentials('db-password')
        DATABASE_HOST = credentials('db-host')
        DATABASE_NAME = credentials('db-name')
        DATABASE_PORT = credentials('db-port')

        // S3 credentials
        S3_ACCESS_KEY_ID = credentials("s3-access-key")
        S3_SECRET_ACCESS_KEY = credentials("s3-secret-access-key")
        S3_HOST = credentials("s3-host")
        S3_REGION = credentials("s3-region")
        S3_BUCKET_NAME = "your-bucket-name"

        // SSH key for private Git dependencies
        SSH_PRIVATE_KEY = credentials("github-ssh-key")
    }

    stages {        
        stage('Debug Kube Context') {
            steps {
                sh 'kubectl config current-context'
                sh 'kubectl get pods --all-namespaces'
            }
        }

        stage('Build and Import Docker Image') {
            steps {
                echo "Building and importing Docker image..."
                sh '''
                    # Copy SSH key for Docker build (if needed)
                    cat "$SSH_PRIVATE_KEY" > ./id_rsa
                    chmod 600 ./id_rsa
                    
                    # Build with SSH key
                    docker build -t $IMAGE_TAG .
                    
                    # Clean up SSH key after build
                    rm -f ./id_rsa
                    
                    docker save -o $IMAGE_FILE $IMAGE_TAG
                    k3s ctr image import $IMAGE_FILE
                '''
            }
        }
        
        stage('Update k8s Manifests with Secrets') {
            steps {
                echo "Updating Kubernetes secret manifest with Jenkins credentials..."
                sh '''
                    # Update database and S3 credentials in secret.yaml
                    cat > k8s/secret.yaml <<EOF
apiVersion: v1
kind: Secret
metadata:
  name: framework-secret
  namespace: $NAMESPACE
  labels:
    app: framework
type: Opaque
stringData:
  DATABASE_USERNAME: "$DATABASE_USERNAME"
  DATABASE_PASSWORD: "$DATABASE_PASSWORD"
  S3_ACCESS_KEY_ID: "$S3_ACCESS_KEY_ID"
  S3_SECRET_ACCESS_KEY: "$S3_SECRET_ACCESS_KEY"
EOF

                    # Update database connection info in configmap.yaml
                    sed -i "s|DATABASE_HOST: .*|DATABASE_HOST: \\"$DATABASE_HOST\\"|" k8s/configmap.yaml
                    sed -i "s|DATABASE_PORT: .*|DATABASE_PORT: \\"$DATABASE_PORT\\"|" k8s/configmap.yaml
                    sed -i "s|DATABASE_NAME: .*|DATABASE_NAME: \\"$DATABASE_NAME\\"|" k8s/configmap.yaml
                    
                    # Update S3 configuration in configmap.yaml
                    sed -i "s|S3_HOST: .*|S3_HOST: \\"$S3_HOST\\"|" k8s/configmap.yaml
                    sed -i "s|S3_REGION: .*|S3_REGION: \\"$S3_REGION\\"|" k8s/configmap.yaml
                    sed -i "s|S3_BUCKET_NAME: .*|S3_BUCKET_NAME: \\"$S3_BUCKET_NAME\\"|" k8s/configmap.yaml
                    
                    # Update image tag in deployment.yaml to force new deployment
                    sed -i "s|image: framework-template:.*|image: $IMAGE_TAG|" k8s/deployment.yaml
                '''
            }
        }


        stage('Apply Kubernetes Manifests') {
            steps {
                echo "Applying all Kubernetes manifests from k8s/ directory..."
                sh '''
                    # Apply in order: ConfigMap, Secret, then Deployment/Service/Ingress
                    kubectl apply -f k8s/configmap.yaml -n $NAMESPACE
                    kubectl apply -f k8s/secret.yaml -n $NAMESPACE
                    kubectl apply -f k8s/service.yaml -n $NAMESPACE
                    kubectl apply -f k8s/deployment.yaml -n $NAMESPACE
                    kubectl apply -f k8s/ingress.yaml -n $NAMESPACE
                '''
            }
        }

        stage('Wait for Deployment Readiness') {
            steps {
                echo "Waiting for deployment to be ready..."
                sh 'kubectl rollout status deployment/$DEPLOYMENT_NAME -n $NAMESPACE'
            }
        }
    }
}