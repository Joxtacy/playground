# Nexus NPM privaet repository
Inspiration: https://levelup.gitconnected.com/deploying-private-npm-packages-to-nexus-a16722cc8166

## Docker

    docker run --rm -it -p 8081:8081 -v `pwd`/nexus-data:/nexus-data sonatype/nexus3
