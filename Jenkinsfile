pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh 'make dev/build'
      }
    }
    stage('Integration') {
      steps {
        withCredentials([file(credentialsId: 'CUSTOMER_CARE_DOT_ENV', variable: 'ENV_FILE')]) {
          sh 'cp $ENV_FILE .env'
        }
        sh 'make dev/up-detached && sleep 5'
        sh 'chmod +x -R tests && docker cp tests server:/app && docker exec server ./tests/run_tests.sh'
        sh 'make dev/drop'
      }
    }
    stage('Build Release') {
      steps {
        sh 'make prod/build'
      }
    }
  }
  post {
    always {
      sh 'docker system prune --force --volumes'
      // deleteDir()
    }
  }
}
