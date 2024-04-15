#!/bin/bash

# Project: Catherine Framework (https://github.com/battleoverflow/catherine)
# Author: battleoverflow (https://github.com/battleoverflow)
# License: BSD 2-Clause

# NetScan
# TODO: Test this function to verify it works, Go is the odd man out
function netscan() {
    cd src/modules/net/netscan/src
    echo "[+] Configuring NetScan module..."
    
    go build src/modules/net/netscan/src/main.go -o /opt/catherine/modules/net/netscan

    if [ -f "/opt/catherine/modules/net/netscan" ]
    then
        echo "[+] NetScan module successfully built"
    else
        echo "[-] NetScan module was not built properly"
    fi

    echo ""
}

# Web parsers
function parsers() {
    # Link parser
    echo "[+] Configuring Link Parser module..."
    pyinstaller src/modules/web/parsers/links.py --onefile --clean -n links --distpath /opt/catherine/modules/web/parsers/ 2>/dev/null

    if [ -f "/opt/catherine/modules/web/parsers/links" ]
    then
        echo "[+] Link Parser module successfully built"
    else
        echo "[-] Link Parser module was not built properly"
    fi

    echo ""
}

# Exec Dump
function exec_dump() {
    echo "[+] Configuring Windows Exe Dump module..."
    pyinstaller src/modules/formats/exe/win_exe_dump.py --onefile --clean -n win_exe_dump --distpath /opt/catherine/modules/formats/exe/ 2>/dev/null

    if [ -f "/opt/catherine/modules/formats/exe/win_exe_dump" ]
    then
        echo "[+] Windows Exe Dump module successfully built"
    else
        echo "[-] Windows Exe Dump module was not built properly"
    fi

    echo ""
}

# Redis
function db_redis() {
    echo "[+] Configuring Redis Database module..."
    pyinstaller src/modules/db/redis.py --onefile --clean -n redis --distpath /opt/catherine/modules/db/ 2>/dev/null

    if [ -f "/opt/catherine/modules/db/redis" ]
    then
        echo "[+] Redis Database module successfully built"
    else
        echo "[-] Redis Database module was not built properly"
    fi

    echo ""
}

# Mercy Extension
function mercy_ext() {
    echo "[+] Configuring Mercy Extension module..."
    pyinstaller src/modules/mercy/extension.py --onefile --clean -n extension --distpath /opt/catherine/modules/mercy/ 2>/dev/null

    if [ -f "/opt/catherine/modules/mercy/extension" ]
    then
        echo "[+] Mercy Extension module successfully built"
    else
        echo "[-] Mercy Extension module was not built properly"
    fi

    echo ""
}

# netscan
parsers
exec_dump
db_redis
mercy_ext
