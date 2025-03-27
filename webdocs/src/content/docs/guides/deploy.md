---
title: Deploy to your server
description: A guide to deploy mojiji to your server.
---

## Installation Methods

### 1. GitHub Releases Installation

#### Step-by-Step Manual Download

1. Visit the [Mojiji GitHub Releases page](https://github.com/HidemaruOwO/mojiji/releases)
2. Find the latest release
3. Check URL for the latest release binary. (e.g. `mojiji_v0.1.0-amd64.tar.gz`)

```bash
# Download the latest release (e.g. v0.1.0 amd64)
wget https://github.com/HidemaruOwO/mojiji/releases/download/v0.1.0/mojiji_v0.1.0-amd64.tar.gz

# Extract the archive
tar xvf mojiji_v0.1.0-amd64.tar.gz

# Move the binary to system path
sudo mv mojiji /usr/local/bin/
```

### 2. Source Code Installation

```bash
# Clone the repository
git clone https://github.com/HidemaruOwO/mojiji.git
cd mojiji

# Build the project
cargo build --release

# Install the binary
sudo install -Dm0755 -t "/usr/local/bin/" "target/release/mojiji"
```

## Deployment Options

### 1. Basic Host Deployment

```bash
# Run Mojiji directly
mojiji

# Run in background
nohup mojiji &

# Stop the background process
pkill mojiji
```

### 2. Systemd Service Deployment

#### Create Systemd Service File

```bash
sudo vim /etc/systemd/system/mojiji.service
```

Paste the following content:

```ini
[Unit]
Description=Mojiji Web API
After=network.target

[Service]
#User=user
#WorkingDirectory=/home/user/app
ExecStart=/usr/local/bin/mojiji
Restart=always
StandardOutput=journal
StandardError=journal
Environment=PATH=/usr/bin:/usr/local/bin

[Install]
WantedBy=multi-user.target
```

#### Manage the Service

```bash
# Enable and start the service
sudo systemctl enable --now mojiji.service

# Check service status
sudo systemctl status mojiji.service
```

### 3. Docker Deployment

#### Understanding the Docker Image

- **Image Source**: `ghcr.io/hidemaruowo/puyodeliver:latest`
- **Maintained by**: HidemaruOwO
- **Container Registry**: GitHub Container Registry (GHCR)

#### Running with Docker

```bash
docker run -d \
  --name mojiji \
  -p 8000:8000 \
  ghcr.io/hidemaruowo/puyodeliver:latest
```

#### Running with Docker Compose

```bash
vim docker-compose.yaml
```

Paste the following content:

```yaml
version: "3"
services:
  mojiji:
    image: ghcr.io/hidemaruowo/puyodeliver:latest
    ports:
      - "8000:8000"
    restart: always
    # Uncomment and modify if you need to pass environment variables
    # environment:
    #   - SOME_ENV_VARIABLE=value
```

Start the service:

```bash
# Start the service
docker-compose up -d
```

## Additional Notes

- Ensure you have the necessary permissions for installation
- Firewall configurations may need adjustment
- Always check the latest documentation for most up-to-date instructions

## Troubleshooting

- Check system logs: `journalctl -xeu mojiji.service` or `docker compose logs`
- Verify binary permissions
- Ensure all dependencies are installed
- Check network and firewall configurations

## Security Notes

- Run services with minimal required permissions
- Keep the system and Mojiji updated
- Use firewall rules to restrict access (allow: `8000`)
- Consider implementing authentication if exposing to public internet
