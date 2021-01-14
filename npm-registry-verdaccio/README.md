# Private NPM Registry with Verdaccio

## Run the docker

```
$: docker run -it --detach \
    --publish 4873:4873 \
    --volume `pwd`/conf:/verdaccio/conf \
    --volume `pwd`/storage:/verdaccio/storage \
    --volume `pwd`/plugins:/verdaccio/plugins \
    --name verdaccio \
    verdaccio/verdaccio
```
