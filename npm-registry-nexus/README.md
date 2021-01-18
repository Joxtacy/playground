# Nexus NPM privaet repository
Inspiration:
* https://levelup.gitconnected.com/deploying-private-npm-packages-to-nexus-a16722cc8166
* https://github.com/julie-ng/nexus-private-npm-registry

## Docker

    docker run --rm -it -p 8081:8081 -v `pwd`/nexus-data:/nexus-data sonatype/nexus3


## For Nexus 2
Repository GUI available at http://localhost:8081/nexus/

    docker run --rm -it -d -p 8081:8081 -v `pwd`/nexus-data-v2:/sonatype-work --name nexusv2 sonatype/nexus:oss

