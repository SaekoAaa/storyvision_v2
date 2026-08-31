pipeline {
    agent any

    options {
        skipDefaultCheckout(true)
        timestamps()
        disableConcurrentBuilds()
        buildDiscarder(logRotator(numToKeepStr: '30'))
    }

    triggers {
        pollSCM('H/5 * * * *')
    }

    environment {
        CARGO_TERM_COLOR = 'always'
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Version metadata') {
            steps {
                script {
                    env.GIT_SHORT_SHA = sh(
                        script: 'git rev-parse --short=7 HEAD',
                        returnStdout: true
                    ).trim()
                    env.AUTH_VERSION = packageVersion('auth_service')
                    env.PROJECTS_VERSION = packageVersion('projects_service')
                    env.ENTITIES_VERSION = packageVersion('entities_service')
                    env.MIGRATOR_VERSION = packageVersion('migrator_service')

                    currentBuild.description = "${env.BRANCH_NAME ?: 'manual'} @ ${env.GIT_SHORT_SHA}"
                    echo "Auth image version: ${env.AUTH_VERSION}-${env.GIT_SHORT_SHA}"
                    echo "Projects image version: ${env.PROJECTS_VERSION}-${env.GIT_SHORT_SHA}"
                    echo "Entities image version: ${env.ENTITIES_VERSION}-${env.GIT_SHORT_SHA}"
                    echo "Migrator image version: ${env.MIGRATOR_VERSION}-${env.GIT_SHORT_SHA}"
                }
            }
        }

        stage('Format') {
            steps {
                sh 'cargo fmt --all -- --check'
            }
        }

        stage('Clippy') {
            steps {
                sh '''
                    cargo clippy \
                        --workspace \
                        --all-targets \
                        --all-features \
                        -- \
                        -D warnings
                '''
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --workspace --all-targets --all-features'
            }
        }

        stage('Compose validation') {
            steps {
                sh '''
                    cp .env.example .env
                    docker compose --profile all --profile ci config --quiet
                    rm .env
                '''
            }
        }
    }

    post {
        always {
            sh 'rm -f .env'
        }
    }
}

String packageVersion(String packageName) {
    return sh(
        script: "cargo pkgid -p '${packageName}' | sed 's/.*@//'",
        returnStdout: true
    ).trim()
}
