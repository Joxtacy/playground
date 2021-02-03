# Jenkins

## Create Docker Network

    $: docker network create jenkins

## Build Jenkins Blueocean Image

    $: docker build -t myjenkins-blueocean:1.1 .

## Run

### Docker in Docker

    $: docker run --name jenkins-docker \
        --rm \
        --detach \
        --privileged \
        --network jenkins \
        --network-alias docker \
        --env DOCKER_TLS_CERTDIR=/certs \
        --volume ${PWD}/jenkins_docker_certs:/certs/client \
        --volume ${PWD}/jenkins_home:/var/jenkins_home \
        --publish 2376:2376 \
        docker:dind

### Jenkins

    $: docker run --name jenkins-blueocean \
        --rm \
        --detach \
        --network jenkins \
        --env DOCKER_HOST=tcp://docker:2376 \
        --env DOCKER_CERT_PATH=/certs/client \
        --env DOCKER_TLS_VERIFY=1 \
        --publish 8080:8080 \
        --publish 50000:50000 \
        --volume ${PWD}/jenkins_docker_certs:/certs/client:ro \
        --volume ${PWD}/jenkins_home:/var/jenkins_home \
        myjenkins-blueocean:1.1

