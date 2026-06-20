#!/bin/bash
set -e
echo "[$(date)] === Deploy started ==="

cd /home/gaurav/Projects/Mail-Service

echo "[$(date)] Pulling latest from git..."
git pull

echo "[$(date)] Building release binary..."
/home/gaurav/.cargo/bin/cargo build --release

echo "[$(date)] Restarting mail-service..."
sudo systemctl restart mail-service

echo "[$(date)] === Deploy complete ==="
