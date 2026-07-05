# mailapi.cursedshrine.com + mail-tunnel removed

**Date:** 2026-07-04 09:35

## What was found
- The `mail-tunnel` no longer existed in the Cloudflare account (`cloudflared tunnel list`
  showed only `sharemarketstudies-app` and `CursedShrine`), so `cloudflared-mail.service`
  was flapping in `activating` forever and https://mailapi.cursedshrine.com returned 530.
  The public endpoint was already dead before this cleanup.
- No live code references the public URL — api-nexus reaches Mail-Service internally
  at `localhost:7070`. Mail-Service itself is untouched and still running.

## Removed
- `cloudflared-mail.service` — stopped, disabled, unit file deleted, daemon reloaded.
- `/home/gaurav/.cloudflared-mail/` — config.yml + tunnel credentials json deleted.
- Nothing to delete account-side: the tunnel was already gone from Cloudflare.

## Manual step remaining (needs Dashboard)
- Delete the `mailapi` CNAME record in Cloudflare Dashboard → cursedshrine.com → DNS.
  No zone-scoped API token is stored on the Pi (by design), and `cert.pem` is
  tunnel-scope only, so this can't be done from here.

## Noticed in passing (not changed)
- Two systemd units run the SAME main tunnel: `cloudflare-tunnel.service` (via config)
  and `cloudflared.service` (by name `sharemarketstudies-app`) — duplicate connectors.
- Orphan `CursedShrine` tunnel (8f932295…) exists in the account with zero connections.
