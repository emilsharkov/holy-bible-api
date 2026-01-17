FROM postgres:16-alpine

# Install bash (optional but nicer)
RUN apk add --no-cache bash

WORKDIR /restore

# Copy restore script + dump
COPY restore.sh /restore/restore.sh
COPY backup.dump /restore/backup.dump

RUN chmod +x /restore/restore.sh

ENTRYPOINT ["/restore/restore.sh"]
