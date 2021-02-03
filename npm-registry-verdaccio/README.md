# Private NPM Registry with Verdaccio
Inspiration: https://blog.bitsrc.io/how-to-set-up-a-private-npm-registry-locally-1065e6790796

## Run the docker

### With docker-compose (recommended)

`--build` is used to build the image if it does not exist yet.
`-d` is to start the container in detached mode.

    $: docker-compose up --build -d

### With just the Verdaccio image (the old way)

```
$: docker run -it --detach \
    --publish 4873:4873 \
    --network jenkins \
    --network-alias verdaccio \
    --volume `pwd`/conf:/verdaccio/conf \
    --volume `pwd`/storage:/verdaccio/storage \
    --volume `pwd`/plugins:/verdaccio/plugins \
    --name verdaccio \
    verdaccio/verdaccio
```
