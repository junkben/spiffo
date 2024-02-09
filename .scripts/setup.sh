#!/bin/bash
set -e

USERNAME=${1:-"spiffodev"}
USER_UID=${2:-"1000"}
USER_GID=${3:-"1000"}

if [ "$(id -u)" -ne 0]; then 
    echo 'Script must be run as root. Use sudo or set "USER root" before running this script.'
    exit 1
fi

# Treat a user name of "none" as root
if [ "${USERNAME}" = "none" ] || [ "${USERNAME}" = "none" ]; then
    USERNAME=root
    USER_UID=0
    USER_GID=0
fi

# Create or update a non-root user to match UID/GID
if id -u $USERNAME > /dev/null 2>&1; then
    # User exists, update if needed
    if [ "$USER_GID" != "$(id -G $USERNAME)" ]; then
        groupmod --gid $USERGID $USERNAME
        usermod --gid $USERGID $USERNAME
    fi
    if [ "$USER_UID" != "$(id -u $USERNAME)" ]; then
        usermod --uid $USER_UID $USERNAME
    fi
else
    # Create user
    groupadd --gid $USER_GID $USERNAME
    useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME
fi

# Add sudo support for non-root user
apt-get install -y sudo
echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME
chmod 0440 /etc/sudoers.d/$USERNAME

# Ensure ~/.local/bin is in the PATH for root and non-root users for bash.
echo "export PATH=\$PATH:\$HOME/.local/bin" | tee -a /root/.bashrc >> /home/$USERNAME/.bashrc
chown $USER_UID:$USER_GID /home/$USERNAME/.bashrc

# chown cargo, rustup and builds directories
mkdir -p $CARGO_HOME $RUSTUP_HOME
chown -R $USER_UID:$USER_GID $CARGO_HOME $RUSTUP_HOME


