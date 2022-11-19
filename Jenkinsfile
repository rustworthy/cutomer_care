pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh 'make prod/build'
      }
    }
    stage('Integration') {
      environment {
        NETWORK_ALIAS="docker"  // from jenkins + docker-in-docker composition 
      }
      steps {
        withCredentials([
          file(credentialsId: 'CUSTOMER_CARE_DOT_ENV', variable: 'ENV_FILE'),
          string(credentialsId: 'MODERATOR_AUTH_KEY', variable: 'MODERATOR_AUTH_KEY')
        ]) {
          sh 'cp $ENV_FILE .env'
          sh 'make ci/up-detached && sleep 5'
          sh "chmod +x -R tests && ./tests/run_tests.sh $NETWORK_ALIAS"
          sh 'make ci/down'
        }
      }
    }
  }
  post {
    always {
      sh 'docker system prune --force'
    }
    success {
      deleteDir()
    }
  }
}
