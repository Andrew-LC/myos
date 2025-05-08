#!/usr/bin/env bash

set -ex

cargo build

# rm -fr isodir
# mkdir -p isodir/boot/grub
# cp target/x86_64-blog_os/debug/myos isodir/boot/myos.bin
# cp grub.cfg isodir/boot/grub/grub.cfg
# grub-mkrescue -o myos.iso isodir

qemu-system-i386 -kernel target/x86_64-blog_os/debug/myos
