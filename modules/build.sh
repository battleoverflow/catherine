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

# Web parsers
function parsers() {
    cd web/parsers

    # Link parser
    echo "[+] Configuring Link parser module..."

    pyinstaller links.py --onefile --clean -n links 2>/dev/null
    
    if [ -f "dist/links" ]
    then
        echo "[+] Link parser successfully built"
    else
        echo "[-] Link parser was not built properly"
    fi
    echo ""
    
    rm -r build/ links.spec
}

parsers
set_dir

# Exec Dump
function exec_dump() {
    cd data/exe
    echo "[+] Configuring Exec Dump module..."

    pyinstaller exec_dump.py --onefile --clean -n exec_dump 2>/dev/null

    if [ -f "dist/exec_dump" ]
    then
        echo "[+] Exec Dump successfully built"
    else
        echo "[-] Exec Dump was not built properly"
    fi
    echo ""
    
    rm -r build/ exec_dump.spec
}

exec_dump
set_dir

# Hex
function c_hex() {
    cd data/hex/c
    echo "[+] Configuring Hex 'C' module..."

    make 2>/dev/null

    if [ -f "dist/hex.so" ]
    then
        echo "[+] Hex 'C' successfully built"
    else
        echo "[-] Hex 'C' was not built properly"
    fi
    echo ""
}

c_hex
set_dir

# Redis Analysis
function redis_analysis() {
    cd db/redis
    echo "[+] Configuring Redis Analysis module..."

    pyinstaller redis_analysis.py --onefile --clean -n redis_analysis 2>/dev/null
    
    if [ -f "dist/redis_analysis" ]
    then
        echo "[+] Redis Analysis module successfully built"
    else
        echo "[-] Redis Analysis module was not built properly"
    fi
    echo ""
    
    rm -r build/ redis_analysis.spec
}

redis_analysis
set_dir

# Mercy Extension
function mercy_ext() {
    cd  mercy
    echo "[+] Configuring Mercy Extension module..."

    pyinstaller extension.py --onefile --clean -n mercy_ext 2>/dev/null

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