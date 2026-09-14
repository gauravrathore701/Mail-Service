# Mail-Service — Mongo writes failing since auth was enabled (2026-09-13 17:15)

## Cause
- `/etc/systemd/system/mail-service.service` set `Environment=MONGODB_URI=mongodb://localhost:27017`
  (credential-less). `dotenv::dotenv()` never overrides an existing env var, so the
  `.env` URI with the `mail_app` user (added 2026-07-15) was ignored → every insert
  failed "Command insert requires authentication" (500).

## Fix
- Unit: removed that Environment line, added `EnvironmentFile=/home/gaurav/Projects/Mail-Service/.env`.
  daemon-reload + restart.
- `.env` was rewritten with `MONGODB_URI` taken from mongo-pi/.env `MAIL_MONGODB_URI`
  (same mail_app user the July note says it already held). I overwrote it without a copy
  first — content should be equivalent (the service only reads MONGODB_URI). Perms set to 600.
- Backup of the unit: `.claude/backups/mail-service.service.bak-20260913`.

## Verified
- Via Mecca `POST /api/notification/subscribe` → 201, document stored with client_id
  "api-nexus"; test document deleted. No credentials written to this note.
