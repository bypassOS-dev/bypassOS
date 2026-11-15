#!/bin/bash

clear

echo "================================================================================="
echo "                           BypassOS 0.1 Installer                                "
echo "================================================================================="
echo ""
echo ""
echo ""

if command -v lsblk >/dev/null 2>&1; then
    echo "Available disks in the system:"
    echo "---------------------------------------------------------"
    lsblk -d -o NAME,SIZE,MODEL,TYPE
    echo "---------------------------------------------------------"
else
    echo "                          * * * Error * * *                                  "
    echo ""
    echo "Utility 'lsblk' is not found. Please install it."
    echo ""
    echo ""

    read -p "Press Enter to exit..."

    exit 1
fi

echo ""
echo "" 
echo "                         Now you must select disk                     "
echo "                              But BE CAREFUL:                         "
echo "                    ALL DATA ON THE DISK WILL BE ERASED               "
echo "                                                                      "
echo "                                                                      "
echo ""
echo ""
read -p "Choice the target disk from output above (e.g., /dev/sdb): " TARGET_DISK

if [ -z "$TARGET_DISK" ]; then
    echo ""
    echo ""
    echo ""
    echo "You didn't enter anything. Exiting."
    exit 1
fi

if [ ! -b  "$TARGET_DISK" ]; then
    echo ""
    echo ""
    echo ""
    echo "Error: Disk '$TARGET_DISK' not found  or isn't a block device."
    exit 1
fi

if mount | grep -q "^$TARGET_DISK"; then
    echo ""
    echo ""
    echo ""
    echo "         Error: Disk '$TARGET_DISK' is currently mounted/in use!"
    echo "*** You cannot install the system on the disk you are currently using ***"
fi


echo ""
echo ""
echo "Selected disk: $TARGET_DISK"
read -p "Press Enter to start formatting and installing, or Ctrl+C to CANCEL..."
