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
        GHCR_NAMESPACE = 'ghcr.io/saekoaaa'
        SOURCE_URL = 'https://github.com/SaekoAaa/storyvision_v2'
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

        stage('Build images') {
            steps {
                script {
                    buildImage(
                        'Storyvision Auth',
                        'auth/Dockerfile',
                        'storyvision-auth',
                        env.AUTH_VERSION
                    )
                    buildImage(
                        'Storyvision Projects',
                        'projects/Dockerfile',
                        'storyvision-projects',
                        env.PROJECTS_VERSION
                    )
                    buildImage(
                        'Storyvision Entities',
                        'entities/Dockerfile',
                        'storyvision-entities',
                        env.ENTITIES_VERSION
                    )
                    buildImage(
                        'Storyvision Migrator',
                        'db_migrator/Dockerfile',
                        'storyvision-migrator',
                        env.MIGRATOR_VERSION
                    )
                }
            }
        }

        stage('Publish images') {
            when {
                anyOf {
                    branch 'master'
                    buildingTag()
                }
            }

            steps {
                withCredentials([
                    usernamePassword(
                        credentialsId: 'ghcr-publisher',
                        usernameVariable: 'GHCR_USER',
                        passwordVariable: 'GHCR_TOKEN'
                    )
                ]) {
                    sh '''
                        set +x
                        printf '%s' "$GHCR_TOKEN" |
                            docker login ghcr.io \
                                --username "$GHCR_USER" \
                                --password-stdin
                    '''

                    script {
                        pushImage('storyvision-auth', env.AUTH_VERSION)
                        pushImage('storyvision-projects', env.PROJECTS_VERSION)
                        pushImage('storyvision-entities', env.ENTITIES_VERSION)
                        pushImage('storyvision-migrator', env.MIGRATOR_VERSION)
                    }
                }
            }

            post {
                always {
                    sh 'docker logout ghcr.io || true'
                }
            }
        }
    }

    post {
        always {
            sh '''
                rm -f .env

                docker image rm \
                    "$GHCR_NAMESPACE/storyvision-auth:$AUTH_VERSION-$GIT_SHORT_SHA" \
                    "$GHCR_NAMESPACE/storyvision-projects:$PROJECTS_VERSION-$GIT_SHORT_SHA" \
                    "$GHCR_NAMESPACE/storyvision-entities:$ENTITIES_VERSION-$GIT_SHORT_SHA" \
                    "$GHCR_NAMESPACE/storyvision-migrator:$MIGRATOR_VERSION-$GIT_SHORT_SHA" \
                    >/dev/null 2>&1 || true
            '''
        }
    }
}

String packageVersion(String packageName) {
    return sh(
        script: "cargo pkgid -p '${packageName}' | sed 's/.*@//'",
        returnStdout: true
    ).trim()
}

void buildImage(String title, String dockerfile, String imageName, String version) {
    String image = "${env.GHCR_NAMESPACE}/${imageName}"
    String tag = "${version}-${env.GIT_SHORT_SHA}"

    sh """
        docker build \
            --file '${dockerfile}' \
            --label 'org.opencontainers.image.title=${title}' \
            --label 'org.opencontainers.image.version=${version}' \
            --label 'org.opencontainers.image.revision=${env.GIT_SHORT_SHA}' \
            --label 'org.opencontainers.image.source=${env.SOURCE_URL}' \
            --tag '${image}:${tag}' \
            .
    """
}

void pushImage(String imageName, String version) {
    String image = "${env.GHCR_NAMESPACE}/${imageName}"
    String tag = "${version}-${env.GIT_SHORT_SHA}"

    sh "docker push '${image}:${tag}'"
}
