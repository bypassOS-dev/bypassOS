#!/bin/bash

my_fun() {
    echo "================================================================================="
    echo "                           BypassOS 0.1 Installer                                "
    echo "================================================================================="
    echo ""
    echo ""
    echo ""
    echo ""
}

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
read -p "Choose the target disk from output above (e.g., /dev/sdb): " TARGET_DISK

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
    exit 1
fi


echo ""
echo ""
echo "Selected disk: $TARGET_DISK"
read -p "Press Enter to start formatting and installing, or Ctrl+C to CANCEL..."

clear

echo ""
echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
echo "!!!                            FINAL WARNING                              !!!"
echo "!!!                                                                       !!!"
echo "!!!  You have selected disk: $TARGET_DISK                                 !!!"
echo "!!!  ALL DATA ON THIS DISK WILL BE PERMANENTLY DESTROYED.                 !!!"
echo "!!!                                                                       !!!"
echo "!!!  If you are sure, type 'YES' (in uppercase) and press Enter.          !!!"
echo "!!!  To cancel, press Ctrl+C.                                             !!!"
echo "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
echo ""

read -p "Type 'YES' to continue: " CONFIRM

if [ "$CONFIRM" != "YES" ]; then
    echo "Installation cancelled by user."
    exit 1
fi

clear

echo "================================================================================="
echo "                           BypassOS 0.1 Installer                                "
echo "================================================================================="
echo ""
echo ""
echo ""
echo "Starting installation..."
echo ""
echo "* Checking required utilities..."

REQUIRED_TOOLS="parted mkfs.fat mkfs.ext4 tar chroot blkid lsblk"

for tool in $REQUIRED_TOOLS; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo ""
        echo "========================================================="
        echo " Error: Required utility '$tool' is not found!"
        echo " Please install it before running the installer."
        echo "========================================================="
        exit 1
    fi
done
clear
my_fun
echo ""
echo "All utilities found. Proceeding..."

clear
my_fun
echo ""
echo "* Create a new partition table in GPT format..."

parted -s "$TARGET_DISK" mklabel gpt


clear
my_fun
echo ""
echo "* Create first partition with size 512MiB..."

parted -s "$TARGET_DISK" mkpart primary fat32 1MiB 513MiB

clear
my_fun
echo ""
echo "* Set ESP-flag..."
parted -s "$TARGET_DISK" set 1 esp on

clear
my_fun
echo ""
echo "* Create second partition..."

parted -s "$TARGET_DISK" mkpart primary ext4 513MiB 100%

clear
my_fun
echo ""
echo "* waiting for Linux to update the files..."
sleep 2


if [[ "$TARGET_DISK" == *"nvme"* ]]; then
    PART_EFI="${TARGET_DISK}p1"
    PART_ROOT="${TARGET_DISK}p2"
else
    PART_EFI="${TARGET_DISK}1"
    PART_ROOT="${TARGET_DISK}2"
fi


clear
my_fun
echo ""
echo "* Formatting partitions..."

mkfs.fat -F32 "$PART_EFI"

mkfs.ext4 -F "$PART_ROOT"


clear
my_fun
echo ""
echo "* Mounting partitions..."

mount "$PART_ROOT" /mnt

mkdir -p /mnt/boot/efi
mount "$PART_EFI" /mnt/boot/efi


clear
my_fun
echo ""
echo "* Copying system files. This takes a few minutes..."

tar -cf - --exclude=/proc --exclude=/sys --exclude=/dev --exclude=/run --exclude=/tmp --exclude=/mnt --exclude=/lost+found --exclude=/home / | tar -C /mnt -xpf -


clear
my_fun
echo ""
echo "* Generating fstab (disk table)..."

UUID_ROOT=$(blkid -s UUID -o value "$PART_ROOT")
UUID_EFI=$(blkid -s UUID -o value "$PART_EFI")

cat > /mnt/etc/fstab <<EOF
UUID=$UUID_ROOT / ext4 defaults 0 1
UUID=$UUID_EFI /boot/efi vfat umask=0077 0 1
EOF


clear
my_fun
echo ""
echo "* Preparing chroot environment..."

mkdir -p /mnt/dev /mnt/proc /mnt/sys

mount --bind /dev /mnt/dev
mount --bind /proc /mnt/proc
mount --bind /sys /mnt/sys

echo "* Installing GRUB and creating user..."

chroot /mnt /bin/bash -c "grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=BypassOS"
chroot /mnt /bin/bash -c "grub-mkconfig -o /boot/grub/grub.cfg"
chroot /mnt /bin/bash -c "useradd -m -s /bin/bash user"

clear
echo ""
echo ""
echo "========================================="
echo " Please set a password for user 'user'   "
echo "========================================="
echo ""
echo ""
chroot /mnt passwd user

sync
sleep 2

clear
my_fun
echo ""
echo "* Finishing up..."

umount /mnt/dev
umount /mnt/proc
umount /mnt/sys

umount /mnt/boot/efi
umount /mnt

clear
my_fun
echo ""
echo ""
echo "                                           INSTALLATION COMPLETE!                                 "
echo "                             You can now reboot and remove the installation media.  "
echo ""







