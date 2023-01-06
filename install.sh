#!/bin/bash

# Current version of Catherine
v="v0.3.47"

# Created directory for Catherine
catherine_dir="/opt/catherine"

# Installs Catherine
function install_catherine() {

    mkdir $catherine_dir

    if [[ -f $catherine_dir ]]
    then
        cd $catherine_dir
        wget https://github.com/CatherineFramework/Catherine/releases/download/$v/catherine
        chmod +x $catherine_dir/catherine
    fi

}

install_catherine

function install_modules() {
    if [[ -f $catherine_dir ]]
    then
        cd $catherine_dir
        git clone https://github.com/CatherineFramework/modules.git
    fi
}

install_modules

function installation_complete() {
    echo "Catherine should now be available under $catherine_dir"
    echo ""
    echo "How to run: '$catherine_dir/catherine'"
}

installation_complete