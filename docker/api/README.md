# API Docker Image

Build the image from `docker/api`:

```sh
docker build -t holy-bible-api:latest -f ./Dockerfile ../../api
```

This uses `api/` as the build context and the Dockerfile in `docker/api/`.
