#!/bin/bash
# Print a banner with numbered points describing the script's actions
cat << "EOF"

╔════════════════════════════════════════════════════════════════╗
║                 IronLog Linux Journalctl Installer             ║
╚════════════════════════════════════════════════════════════════╝

This script will:
1. Build the ironlog-linux-journalctl binary
2. Copy the binary to /usr/bin
3. Create a systemd service file
4. Enable and start the ironlog-linux-journalctl service

EOF

# Ask for user confirmation
read -p "Do you want to continue? (y/n): " confirm
if [[ $confirm != [yY] ]]; then
    echo "Installation aborted."
    exit 1
fi

echo "Proceeding with installation..."
echo


# Check if running on Linux
if [[ "$(uname)" != "Linux" ]]; then
    echo "Error: This script is designed to run on Linux systems only."
    exit 1
fi

# Check if systemd is available
if ! command -v systemctl &> /dev/null; then
    echo "Error: systemd is not available on this system."
    exit 1
fi

# Parse command line arguments
URL="127.0.0.1:5000"
API_KEY=""
APP_NAME=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --url)
            URL="$2"
            shift 2
            ;;
        --api-key)
            API_KEY="$2"
            shift 2
            ;;
        --app-name)
            APP_NAME="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# If app name is not provided, use hostname
if [ -z "$APP_NAME" ]; then
    APP_NAME=$(hostname)
fi

# Build the binary as the current user
echo "Building the binary..."
cargo build --release

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "Error: Failed to build the binary."
    exit 1
fi

# The rest of the script requires root privileges
echo "The rest of the installation requires root privileges."
echo "You will be prompted for your password."

# Stop the service if it already exists
if sudo systemctl is-active --quiet ironlog-linux-journalctl.service; then
    echo "Stopping existing ironlog-linux-journalctl service..."
    sudo systemctl stop ironlog-linux-journalctl.service
fi

# Copy binary to /usr/bin
echo "Copying binary to /usr/bin..."
sudo cp target/release/log_pipe /usr/bin/ironlog-linux-journalctl

# Prepare the ExecStart command
EXEC_START="/usr/bin/ironlog-linux-journalctl --url $URL --app-name $APP_NAME"
if [ -n "$API_KEY" ]; then
    EXEC_START="$EXEC_START --api-key $API_KEY"
fi

# Create systemd service file
echo "Creating systemd service file..."
sudo bash -c "cat << EOF > /etc/systemd/system/ironlog-linux-journalctl.service
[Unit]
Description=IronLog Linux Journalctl Service
After=network.target

[Service]
ExecStart=$EXEC_START
Restart=always
User=root

[Install]
WantedBy=multi-user.target
EOF"

# Reload systemd, enable and start the service
echo "Reloading systemd, enabling and starting the service..."
sudo systemctl daemon-reload
sudo systemctl enable ironlog-linux-journalctl.service
sudo systemctl start ironlog-linux-journalctl.service

echo "Installation complete. The ironlog-linux-journalctl service is now running."