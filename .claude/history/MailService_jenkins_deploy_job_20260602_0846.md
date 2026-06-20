# Jenkins Deploy Job — mailapi.cursedshrine.com
**Date:** 2026-06-02 08:46 IST

## What Was Done

Created a Jenkins Freestyle job (`deploy-mailapi`) to deploy the Mail-Service (Rust/axum) running at mailapi.cursedshrine.com.

## Files Created / Modified

| File | Action |
|------|--------|
| `/home/gaurav/Projects/Mail-Service/deploy.sh` | Created — deploy script (git pull → cargo build --release → restart) |
| `/etc/sudoers.d/jenkins-mail-deploy` | Created — allows `jenkins` user to run deploy.sh as `gaurav` NOPASSWD |
| `/var/lib/jenkins/jobs/deploy-mailapi/config.xml` | Created — Jenkins Freestyle job config |

## Deploy Pipeline

1. Jenkins job runs: `sudo -u gaurav /home/gaurav/Projects/Mail-Service/deploy.sh`
2. `deploy.sh` does:
   - `git pull` (uses gaurav's SSH key)
   - `cargo build --release`
   - `sudo systemctl restart mail-service`

## Permissions Chain

```
jenkins user
  → sudo -u gaurav deploy.sh  (via /etc/sudoers.d/jenkins-mail-deploy)
    → git pull (gaurav's SSH key)
    → cargo build
    → sudo systemctl restart mail-service  (gaurav has NOPASSWD ALL)
```

## Trigger

Manual trigger from Jenkins UI at jenkins.cursedshrine.com (job: `deploy-mailapi`).
No webhook configured — add GitHub webhook pointing to Jenkins if auto-deploy on push is needed.

## Notes

- First run will be slow (cold Rust build ~5-10 min on Pi)
- Jenkins was restarted to load the new job
- Service stays on port 7070 behind cloudflared-mail tunnel
