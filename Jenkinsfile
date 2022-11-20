pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        retry(3) {
          sh 'make prod/build'
        }
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
    stage('Push Image') {
      environment {
        CONTAINER_REGISTRY_URL="https://index.docker.io/v1/"

        SERVER_SRC="customer_care/prod/server:latest"
        SERVER_TGT="rustworthy/customer_care:$BUILD_ID"
        SERVER_TGT_TIP="rustworthy/customer_care:latest"

        DB_SRC="customer_care/db:latest"
        DB_TGT="rustworthy/postgres:$BUILD_ID"
        DB_TGT_TIP="rustworthy/postgres:latest"
      }
      steps {
        withDockerRegistry(credentialsId: 'CUSTOMER_CARE_CONTAINER_REGISTRY', url: CONTAINER_REGISTRY_URL) {
          sh "docker tag $SERVER_SRC $SERVER_TGT && docker push $SERVER_TGT && docker image rm $SERVER_TGT"
          sh "docker tag $SERVER_SRC $SERVER_TGT_TIP && docker push $SERVER_TGT_TIP && docker image rm $SERVER_TGT_TIP"

          sh "docker tag $DB_SRC $DB_TGT && docker push $DB_TGT && docker image rm $DB_TGT"
          sh "docker tag $DB_SRC $DB_TGT_TIP && docker push $DB_TGT_TIP && docker image rm $DB_TGT_TIP"
        }
      }
    }
  }
  post {
    always {
      sh 'docker logout'
      sh 'docker system prune --force'
    }
    success {
      deleteDir()
    }
  }
}
