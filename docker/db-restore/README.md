# Generating backup.dump
On a device with a populated database run `pg_dump -U postgres -h localhost -d postgres -Fc -v -f backup.dump`

# Building and Pushing Image to GHCR
```
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t ghcr.io/emilsharkov/holy-bible-api/db-restore:latest \
  --push \
  --cache-to=type=inline \
  .
echo $CR_PAT | docker login ghcr.io -u USERNAME --password-stdin
docker push ghcr.io/emilsharkov/holy-bible-api/holy-bible-db-restore:latest
```