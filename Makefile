# Variables  
SUI_VERSION := v1.38.3
SUI_URL:= https://github.com/MystenLabs/sui/releases/download/mainnet-$(SUI_VERSION)/sui-mainnet-$(SUI_VERSION)-ubuntu-x86_64.tgz
SUI_DIR := $(HOME)/sui
WALRUS_REPO := https://github.com/walrus-storage/walrus.git  
SITE_BUILDER_REPO := https://github.com/walrus-storage/walrus-site-builder.git  
SYSTEM := ubuntu-x86_64

# Default target  
all: setup-sui setup-walrus setup-site-builder  

# Target to set up Sui  
setup-sui:  
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
		echo 'export PATH=\$$PATH:$(SUI_DIR)' >> ~/.zshrc; \
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
	@echo "Adding Walrus to PATH..."
	@if ! grep -q 'export PATH=.*$(HOME)/walrus' ~/.zshrc; then \
		echo 'export PATH=$$PATH:$(HOME)/walrus' >> ~/.zshrc; \
		. ~/.zshrc; \
	fi
	@echo "Walrus setup complete."

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

# # Clean up all downloaded and cloned files  
# clean:  
#   @echo "Cleaning up..."  
#   rm -rf $(SUI_DIR)  
#   rm -rf walrus  
#   rm -rf walrus-site-builder  
#   @echo "Cleanup complete."  

# # Help target  
# help:  
  # @echo "Available targets:"  
  # @echo "  all                 - Set up Sui, Walrus, and Walrus Site-Builder"  
  # @echo "  setup-sui           - Set up Sui binaries"  
  # @echo "  setup-walrus        - Clone and set up Walrus repository"  
  # @echo "  setup-site-builder  - Clone and set up Walrus Site-Builder repository"  
  # @echo "  clean               - Remove all downloaded and cloned files"  
  # @echo "  help                - Show this help message"  