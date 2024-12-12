# Variables  
SYSTEM := ubuntu-x86_64

OS_NAME := $(shell uname -s)
CPU_ARCH := $(shell uname -m)
# Map OS and CPU architecture to SYSTEM variable
ifeq ($(OS_NAME),Linux)
	ifeq ($(CPU_ARCH),x86_64)
		SYSTEM := ubuntu-x86_64
	else
		SYSTEM := ubuntu-x86_64-generic
	endif
else ifeq ($(OS_NAME),Darwin)
	ifeq ($(CPU_ARCH),arm64)
		SYSTEM := macos-arm64
	else ifeq ($(CPU_ARCH),x86_64)
		SYSTEM := macos-x86_64
	endif
else ifeq ($(OS_NAME),Windows_NT)
	SYSTEM := windows-x86_64.exe
else
	$(error "Unsupported OS: $(OS_NAME)")
endif

SUI_URL := $(shell curl -s https://api.github.com/repos/MystenLabs/sui/releases/latest | grep "browser_download_url.*$(SYSTEM).tgz" | cut -d '"' -f 4)
SUI_DIR := $(HOME)/sui
WALRUS_REPO := https://github.com/walrus-storage/walrus.git  
SITE_BUILDER_REPO := https://github.com/walrus-storage/walrus-site-builder.git  

# Default target  
all: setup-sui setup-walrus setup-site-builder get-balance

# Target to set up Sui  
setup-sui:  
	@echo "Downloading Sui binaries from $(SUI_URL)"
	@echo "Setting up Sui..." 
	@if [ ! -d "$(SUI_DIR)" ]; then \
		echo "Downloading Sui binaries..."; \
		curl -L $(SUI_URL) -o sui.tgz; \
		mkdir -p $(SUI_DIR); \
		tar -xvzf sui.tgz -C $(SUI_DIR); \
		rm sui.tgz; \
	else \
		echo "Sui is already set up."; \
	fi
	@echo "Adding Sui to PATH..." 
	@if ! grep -q 'export PATH=.*$(SUI_DIR)' ~/.zshrc; then \
		echo "Adding to path..."; \
		echo 'export PATH=$$PATH:$(SUI_DIR)' >> ~/.zshrc; \
		. ~/.zshrc; \
	fi 
	@echo "Sui setup complete."

# Target to set up Walrus  
setup-walrus:
	@echo "Setting up Walrus..."
	@if [ ! -f "$(HOME)/walrus" ]; then \
		echo "Downloading walrus binary..."; \
		curl -L https://storage.googleapis.com/mysten-walrus-binaries/walrus-testnet-latest-$(SYSTEM) -o $(HOME)/walrus; \
		chmod +x $(HOME)/walrus; \
	else \
		echo "Walrus is already set up."; \
	fi
	@echo "Adding Walrus to /usr/local/bin"
	sudo mv ~/walrus /usr/local/bin/walrus

	@echo "Walrus setup complete."
	@echo "Downloading walrus config..."
	curl https://docs.blob.store/client_config.yaml --create-dirs -o ~/.config/walrus/client_config.yaml


# Target to set up Walrus Site-Builder  
setup-site-builder:
	@echo "Setting up Walrus Site-Builder..."
	@if [ ! -d "$(HOME)/walrus-sites" ]; then \
		echo "Cloning Walrus Sites"; \
		cd $(HOME) && git clone https://github.com/MystenLabs/walrus-sites.git; \
	else \
		echo "Walrus sites repository already exists."; \
	fi
	@echo "Building site-builder tool"
	@cd $(HOME)/walrus-sites && cargo build --release
	@echo "Adding site-builder to PATH..."
	@if ! grep -q 'export PATH=.*walrus-sites/target/release' ~/.zshrc; then \
		echo 'export PATH=$$PATH:$(HOME)/walrus-sites/target/release' >> ~/.zshrc; \
		. ~/.zshrc; \
	fi
	@echo "Walrus site-builder setup complete."


get-balance:
	# sui client faucet
	# @echo "waiting for sui..."
	# sleep 10
	walrus get-wal

clean:
	clean:
		@echo "Cleaning up..."
		@echo "Removing Sui directory..."
		rm -rf $(SUI_DIR)
		@echo "Removing Walrus binary..."
		sudo rm -f /usr/local/bin/walrus
		@echo "Removing Walrus config..."
		rm -rf ~/.config/walrus
		@echo "Removing Walrus Sites directory..."
		rm -rf $(HOME)/walrus-sites
		@echo "Cleanup complete."
