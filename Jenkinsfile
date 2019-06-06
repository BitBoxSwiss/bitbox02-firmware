pipeline {
    agent {
        docker {
            image 'shiftcrypto/firmware_v2:latest'
            registryUrl 'https://registry.hub.docker.com'
            args '--privileged -it'
            reuseNode true
        }
    }
    stages {
        stage('Clean-up firmware, bootloader and device test') {
            steps {
                sh 'make clean'
            }
        }
        stage('Check style') {
            steps {
                sh './.ci/check-style'
                sh 'make -C py'
                sh './.ci/check-pep8'
            }
        }
        stage('Build bootloader') {
            steps {
                sh 'make -j8 bootloader'
            }
        }
        stage('Build devdevice bootloader') {
            steps {
                sh 'make -j8 bootloader-devdevice'
            }
        }
        stage('Build production bootloader') {
            steps {
                sh 'make -j8 bootloader-production'
            }
        }
        stage('Build firmware') {
            steps {
                sh 'make -j8 firmware'
            }
        }
        stage('Build factory-setup') {
            steps {
                sh 'make -j8 factory-setup'
            }
        }
        stage('Build unit tests') {
            steps {
                sh 'make -j8 unit-test'
            }
        }
        stage('Run unit tests') {
            steps {
                sh 'make -j8 run-unit-tests'
            }
        }
        stage('Build device test') {
            steps {
                sh 'make -j8 device-test'
            }
        }
    }
    post {
        always {
            archiveArtifacts 'build/coverage_html/**/*'
        }
    }
}
