#!/usr/bin/env bash
set -euo pipefail

: "${PGHOST:?PGHOST is required}"
: "${PGPORT:?PGPORT is required}"
: "${PGUSER:?PGUSER is required}"
: "${PGDATABASE:?PGDATABASE is required}"
: "${PGPASSWORD:?PGPASSWORD is required}"

DUMP_FILE="/restore/backup.dump"
MARKER_FILE="/var/lib/postgresql/data/.restored"

# Skip if already restored
if [ -f "$MARKER_FILE" ]; then
  echo "Restore marker found ($MARKER_FILE). Skipping restore."
  exit 0
fi

echo "Waiting for PostgreSQL at $PGHOST..."
until pg_isready -h "$PGHOST" -U "$PGUSER" > /dev/null 2>&1; do
  sleep 2
done

echo "PostgreSQL is ready. Starting restore..."

pg_restore \
  -h "$PGHOST" \
  -p "$PGPORT" \
  -U "$PGUSER" \
  -d "$PGDATABASE" \
  --no-owner \
  --no-privileges \
  -v \
  "$DUMP_FILE"

# Create marker after successful restore
touch "$MARKER_FILE"

echo "Restore completed successfully. Marker created."
