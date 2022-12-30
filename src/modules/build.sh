#!/bin/bash

# Project: Catherine (https://github.com/CatherineFramework)
# Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
# License: BSD 2-Clause

currentDir=$(pwd)

function set_dir() {
    cd $currentDir
}

# NetScan
function netscan() {
    cd net/netscan/src
    echo "[+] Configuring NetScan module..."
    
    go build -o netscan

    if [ -f "netscan" ]
    then
        echo "[+] NetScan successfully built"
    else
        echo "[-] NetScan was not built properly"
    fi
    echo ""

    mv netscan ../dist
}

netscan
set_dir

# Web Parser
function parser() {
    cd web/web_parser
    echo "[+] Configuring Web Parser module..."

    pyinstaller parser.py --onefile --clean -n parser 2>/dev/null
    
    if [ -f "dist/parser" ]
    then
        echo "[+] Web Parser successfully built"
    else
        echo "[-] Web Parser was not built properly"
    fi
    echo ""
    
    rm -r build/ parser.spec
}

parser
set_dir

# Executable Dump
function exe_dump() {
    cd data/exe
    echo "[+] Configuring Executable Dump module..."

    pyinstaller exe_dump.py --onefile --clean -n exe_dump 2>/dev/null

    if [ -f "dist/exe_dump" ]
    then
        echo "[+] Executable Dump successfully built"
    else
        echo "[-] Executable Dump was not built properly"
    fi
    echo ""
    
    rm -r build/ exe_dump.spec
}

exe_dump
set_dir

# Hex
function hex() {
    cd data/hex
    echo "[+] Configuring Hex module..."

    make 2>/dev/null

    if [ -f "dist/hex.so" ]
    then
        echo "[+] Hex successfully built"
    else
        echo "[-] Hex was not built properly"
    fi
    echo ""
}

hex
set_dir

# Redis Analyzer
function redis_db() {
    cd db_analysis/redis
    echo "[+] Configuring Redis Analysis module..."

    pyinstaller redis_db.py --onefile --clean -n redis_db 2>/dev/null
    
    if [ -f "dist/redis_db" ]
    then
        echo "[+] Redis Analyzer successfully built"
    else
        echo "[-] Redis Analyzer was not built properly"
    fi
    echo ""
    
    rm -r build/ redis_db.spec
}

redis_db
set_dir

# Mercy Extension
function mercy_ext() {
    cd  mercy
    echo "[+] Configuring Mercy Extension module..."

    pyinstaller utils/mercy_ext.py --onefile --clean -n mercy_ext 2>/dev/null

    if [ -f "dist/mercy_ext" ]
    then
        echo "[+] Mercy Extension successfully built"
    else
        echo "[-] Mercy Extension was not built properly"
    fi
    echo ""

    rm -r build/ mercy_ext.spec
}

mercy_ext
set_dir