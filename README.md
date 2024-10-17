# IronLog Linux Journalctl

IronLog Linux Journalctl is a powerful tool that streams system logs from journalctl to an IronLog server. It provides real-time log monitoring and centralized log management for Linux systems using systemd. You can use this to have many linux systems such as a server, Raspberry Pi, other SBC or just your laptop streaming logs to a single location. 

## Features

- Streams logs from journalctl in real-time
- Sends logs to a specified IronLog server
- Configurable application name and server URL
- Supports optional API key for future authentication
- Runs as a systemd service for persistent operation

## Prerequisites

- Linux system with systemd
- Rust and Cargo installed
- `journalctl` command available

## Installation

1. Run Ironlog somewhere:
   ```
   cargo install ironlog
   ironlog --tcp-listener-ip 0.0.0.0
   ```
   Make note of the IP address that is printed and you can point your URL to this IP address. 

2. Clone this repository:
   ```
   git clone https://github.com/heety313/ironlog-linux-journalctl.git
   cd ironlog-linux-journalctl
   ```

3. Run the installation script:
   ```
   ./install.sh [URL] [API_KEY]
   ```
   Replace `[URL]` with your IronLog server URL (default: 127.0.0.1:5000) and `[API_KEY]` with your API key if required or leave blank.


## Usage

Once installed, the service will start automatically and begin streaming logs to your IronLog server. It is enabled and will be persistent across reboots.

To manage the service:

- Start: `sudo systemctl start ironlog-linux-journalctl`
- Stop: `sudo systemctl stop ironlog-linux-journalctl`
- Restart: `sudo systemctl restart ironlog-linux-journalctl`
- Check status: `sudo systemctl status ironlog-linux-journalctl`

## Configuration

The service uses the following configuration:

- URL: The IronLog server URL (default: 127.0.0.1:5000)
- App Name: The name of your application (default: system hostname)
- API Key: Optional API key for authentication

To change these settings, edit the systemd service file:

