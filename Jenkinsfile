pipeline {

  agent {
    docker {
      image 'rust:latest'
    }
  }

  stages {
    stage('Test') {
        steps {
            sh 'cargo test'
        }
    }

    stage('Build and Push Image') {
        steps {
            sh 'docker build -f ./Docker/server/server.dev.Dockerfile -t "customer_care/dev/server:$BUILD_TAG" .'
            sh 'docker push "customer_care/dev/server:$BUILD_TAG"'
      }
    }
  }
}