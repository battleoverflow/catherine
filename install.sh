#!/bin/bash

# Current version of Catherine executable
v="v0.3.47"

# Installs Catherine
function install_catherine() {
    wget https://github.com/CatherineFramework/Catherine/releases/download/$v/catherine
    chmod +x catherine
}

install_catherine