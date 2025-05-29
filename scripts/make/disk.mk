# disk.mk - Makefile fragment for disk image creation and management
# Uses create_disk.sh script to handle disk image operations

# Default parameters
BLK ?= y                     # Enable block device support by default
DISK_IMG ?= $(PWD)/disk.img # Default disk image file
DISK_SIZE_MB ?= 100         # Default disk size in MB
FS_TYPE ?= ext4             # Default filesystem type for disk images
DISK_SOURCES ?=             # Input sources, format as "source1:output1:fs_type1,source2:output2:fs_type2"

# Path to the disk creation script
DISK_SCRIPT := $(dir $(lastword $(MAKEFILE_LIST)))create_disk.sh

# Ensure the script is executable
$(shell chmod +x $(DISK_SCRIPT))

# Target: Create disk images from specified sources
disk_img:
	@if [ -n "$(DISK_SOURCES)" ]; then \
		$(DISK_SCRIPT) -i $(DISK_SOURCES) -s $(DISK_SIZE_MB); \
	elif [ -n "$(DISK_IMG)" ]; then \
		$(DISK_SCRIPT) -o $(DISK_IMG) -fs $(FS_TYPE) -s $(DISK_SIZE_MB); \
	else \
		echo "Warning: No disk images or sources specified in DISK_IMG or DISK_SOURCES!"; \
	fi

# Clean up disk images
clean_disk:
	@echo "Cleaning disk images..."
	@if [ -n "$(DISK_SOURCES)" ]; then \
		IFS=','; \
		for entry in $(DISK_SOURCES); do \
			out_img=$$(echo $$entry | cut -d: -f2); \
			if [ -f "$$out_img" ]; then \
				rm -f "$$out_img"; \
				echo "Removed $$out_img"; \
			fi; \
		done; \
	elif [ -n "$(DISK_IMG)" ]; then \
		IFS=','; \
		for img in $(DISK_IMG); do \
			if [ -f "$$img" ]; then \
				rm -f "$$img"; \
				echo "Removed $$img"; \
			fi; \
		done; \
	fi

# Phony targets
.PHONY: disk_img clean_disk

