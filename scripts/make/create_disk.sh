#!/bin/sh
################################################################
# Script for creating disk images from directories
# Usage:
#   ./create_disk.sh -i source:output:fs_type[,source2:output2:fs_type2...]
#   ./create_disk.sh -o output.img -fs ext4 -s 100 -i /path/to/source
################################################################

# Default settings
SIZE_MB=100
FS_TYPE="ext4"
OUTPUT=""
INPUT_DIR=""
SOURCES_LIST=""

display_help() {
  echo "Usage:"
  echo "  ./create_disk.sh -i source:output:fs_type[,source2:output2:fs_type2...]"
  echo "  ./create_disk.sh -o output.img -fs ext4 -s 100 -i /path/to/source"
  echo ""
  echo "Options:"
  echo "  -i | --input     Input format: either source_dir:output_img:fs_type (comma-separated for multiple)"
  echo "                   or just a source directory (requires -o option)"
  echo "  -o | --output    Output disk image path (used with -i when it's just a directory)"
  echo "  -fs | --fstype   Filesystem type: ext4|fat32, default is ext4"
  echo "  -s | --size      Size of disk image in MB, default is 100"
  echo "  -h | --help      Display help"
  echo ""
  exit 1
}

# Parse arguments
while [ "$1" != "" ]; do
  case $1 in
    -i | --input )    shift
                      if [ "$1" = "" ]; then
                        echo "Error: No input specified"
                        display_help
                      fi
                      if echo "$1" | grep -q ':'; then
                        SOURCES_LIST=$1
                      else
                        INPUT_DIR=$1
                      fi
                      ;;
    -o | --output )   shift
                      OUTPUT=$1
                      ;;
    -fs | --fstype )  shift
                      FS_TYPE=$1
                      ;;
    -s | --size )     shift
                      SIZE_MB=$1
                      ;;
    -h | --help )     display_help
                      ;;
    * )               echo "Unknown option: $1"
                      display_help
                      ;;
  esac
  shift
done

# Function to create a disk image
create_disk_image() {
  SRC_DIR=$1
  IMG_FILE=$2
  FS=$3
  
  echo "Creating disk image: $IMG_FILE (filesystem: $FS, size: $SIZE_MB MB)"
  
  # Create disk image
  dd if=/dev/zero of="$IMG_FILE" bs=1M count=$SIZE_MB status=none || {
    echo "Error: Failed to create disk image $IMG_FILE"
    return 1
  }
  
  # Format disk image
  if [ "$FS" = "ext4" ]; then
    mkfs.ext4 -O ^metadata_csum -F "$IMG_FILE" >/dev/null 2>&1 || {
      echo "Error: Failed to format $IMG_FILE as ext4"
      return 1
    }
  elif [ "$FS" = "fat32" ]; then
    mkfs.vfat -F 32 "$IMG_FILE" >/dev/null 2>&1 || {
      echo "Error: Failed to format $IMG_FILE as fat32"
      return 1
    }
  else
    echo "Error: Unsupported filesystem: $FS"
    return 1
  fi
  
  # Create temporary mount point
  MOUNT_DIR=$(mktemp -d)
  
  # Mount disk image and copy files
  OS=$(uname -s)
  if [ "$OS" = "Darwin" ]; then
    # macOS specific commands
    hdiutil attach "$IMG_FILE" -mountpoint "$MOUNT_DIR" >/dev/null 2>&1 || {
      echo "Error: Failed to mount $IMG_FILE"
      rmdir "$MOUNT_DIR"
      return 1
    }
    
    if [ -d "$SRC_DIR" ]; then
      echo "Copying files from $SRC_DIR to $IMG_FILE"
      cp -r "$SRC_DIR"/* "$MOUNT_DIR"/ 2>/dev/null || true
    fi
    
    hdiutil detach "$MOUNT_DIR" >/dev/null 2>&1
  else
    # Linux commands
    if command -v sudo >/dev/null 2>&1; then
      sudo mount -o loop "$IMG_FILE" "$MOUNT_DIR" || {
        echo "Error: Failed to mount $IMG_FILE"
        rmdir "$MOUNT_DIR"
        return 1
      }
      
      if [ -d "$SRC_DIR" ]; then
        echo "Copying files from $SRC_DIR to $IMG_FILE"
        sudo cp -r "$SRC_DIR"/* "$MOUNT_DIR"/ 2>/dev/null || true
      fi
      
      sudo umount "$MOUNT_DIR"
    else
      mount -o loop "$IMG_FILE" "$MOUNT_DIR" || {
        echo "Error: Failed to mount $IMG_FILE"
        rmdir "$MOUNT_DIR"
        return 1
      }
      
      if [ -d "$SRC_DIR" ]; then
        echo "Copying files from $SRC_DIR to $IMG_FILE"
        cp -r "$SRC_DIR"/* "$MOUNT_DIR"/ 2>/dev/null || true
      fi
      
      umount "$MOUNT_DIR"
    fi
  fi
  
  # Clean up
  rmdir "$MOUNT_DIR"
  chmod 644 "$IMG_FILE"
  echo "Disk image $IMG_FILE created successfully!"
  return 0
}

# Process the input
if [ -n "$SOURCES_LIST" ]; then
  # Process comma-separated list of source:output:fs_type
  IFS=','
  for entry in $SOURCES_LIST; do
    # Parse entry
    SRC_PATH=$(echo "$entry" | cut -d: -f1)
    OUT_IMG=$(echo "$entry" | cut -d: -f2)
    FS=$(echo "$entry" | cut -d: -f3)
    
    # Use default filesystem if not specified
    if [ -z "$FS" ]; then
      FS="$FS_TYPE"
      echo "Warning: No filesystem specified for $SRC_PATH, using default $FS_TYPE"
    fi
    
    # Validate source path
    if [ ! -d "$SRC_PATH" ]; then
      echo "Error: Source path $SRC_PATH does not exist or is not a directory!"
      exit 1
    fi
    
    # Create disk image
    create_disk_image "$SRC_PATH" "$OUT_IMG" "$FS" || exit 1
  done
elif [ -n "$INPUT_DIR" ] && [ -n "$OUTPUT" ]; then
  # Single input directory and output image
  if [ ! -d "$INPUT_DIR" ]; then
    echo "Error: Source path $INPUT_DIR does not exist or is not a directory!"
    exit 1
  fi
  
  create_disk_image "$INPUT_DIR" "$OUTPUT" "$FS_TYPE" || exit 1
else
  echo "Error: Either provide -i with source:output:fs_type format or both -i with source directory and -o with output image"
  display_help
fi

exit 0


