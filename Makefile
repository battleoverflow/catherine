# Project: Catherine Framework (https://github.com/azazelm3dj3d/catherine)
# Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
# License: BSD 2-Clause

NAME=catherine
PROJ_VERSION=0.6.0

run:
	@echo "Building $(NAME) v$(PROJ_VERSION)"
	make setup_env
	make build
	make modules

setup_env:
	sudo mkdir -p /opt/catherine/modules
	sudo mkdir -p /opt/catherine/modules/db
	sudo mkdir -p /opt/catherine/modules/formats/exe
	sudo mkdir -p /opt/catherine/modules/formats/hex
	sudo mkdir -p /opt/catherine/modules/mercy
	sudo mkdir -p /opt/catherine/modules/net/netscan
	sudo mkdir -p /opt/catherine/modules/web/parsers
	pip3 install -r requirements.txt

build:
	cargo check && cargo build

modules:
	chmod +x build_modules.sh && sudo ./build_modules.sh

	@echo "[+] Configuring Hex 'C' module..."
	sudo cc src/modules/formats/hex/c_hex_dump.c -Wall -shared -o /opt/catherine/modules/formats/hex/hex.so
	
	# Cleanup spec files from pyinstaller
	sudo rm *.spec
