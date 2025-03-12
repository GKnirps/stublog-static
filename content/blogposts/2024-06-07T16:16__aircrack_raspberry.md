---
title: WiFi sniffing on a Raspberry Pi 5
filename: aircrack_raspberry
date: 2024-06-07T16:16:37+02:00
update-date:
tags: raspberry pi, wifi, sniffing, linux, aircrack-ng, debian
category: hoellenmaschinen
summary: WiFi-Sniffing on a Raspberry Pi 5 is not as easy as I thought. How I made it work.
image:
language: en
---

Note: I usually write stuff in German here, but people who are interested in this may not speak German, so I'll do this one in English.

### The Problem

For my current research project I need to do some WiFi sniffing. So I just set up a Raspberry Pi and run `airodump-ng` (part of the [aircrack-ng](https://www.aircrack-ng.org/) project) on it, right?

Wrong. The basic idea is simple: put the WiFi card into monitor mode, then run `airodump-ng`. For the first part, aircrack-ng provides a tool that does this for me. It even kills other processes that use the card (important: You won't be able to use your WiFi on that device until you undo this, so make sure you have a separate network connection if you need one):

```
sudo airmon-ng start wlan0 
```

`wlan0` is the WiFi interface in this case. Usually, it looks something like this:

```
PHY     Interface       Driver          Chipset

phy0    wlan0           brcmfmac        Broadcom 43455
        wlan0 is soft blocked, please run "rfkill unblock 1" to use this interface.
rfkill error, unable to start wlan0

Would you like to try and automatically resolve this? [y/n] y
```

After that, the interface is in monitor mode. On my Raspberry Pi 5 with Debian 12 however, the above output was followd by this:

```
ERROR adding monitor mode interface: command failed: Operation not supported (-95)
```

### Searching for a solution

This was a bit surprising. That seriously won't work? Let's take a look at the [aircrack-docs](https://www.aircrack-ng.org/doku.php?id=airmon-ng#error_adding_monitor_mode_interfacecommand_failedoperation_not_supported_-95):

> Even though dmesg says the interface is already in monitor mode and “iw dev wlan0 info” confirms it is, airodump-ng will fail and report the interface data linktype is Ethernet. This is a bug in the driver and/or firmware, and the workaround is to reboot the system or to reload the driver: 
> ```
> rmmod brcmfmac
> modprobe brcmfmac
> ```

The Problem persists, however. Reloading the modules does not work. Rebooting does not work. Firmware updates do not work. Then I stumbles upon [this thread in the Raspberry Pi forums](https://forums.raspberrypi.com/viewtopic.php?t=253695). Someone has the same problem that I have and suggeted to rebuild “nexmon”. But what is “nexmon”?

### Nexmon to the rescue!

[Nexmon](https://github.com/seemoo-lab/nexmon) is:

> our C-based firmware patching framework for Broadcom/Cypress WiFi chips that enables you to write your own firmware patches, for example, to enable monitor mode with radiotap headers and frame injection.

Sounds like this is exactly what I need. They even have a detailed review on how to build and use it! So let's go!

Unfortunately, it's not that easy. The Raspberry Pi's WiFi chipset is mentioned, but only in the context of a Raspberry Pi 3 or 4. Also: kernel versions 4.x and 5.* are supported, but I'm running 6.6.31. Oh wait. There is also an entry for a Raspberry Pi 5. But that one does not support monitor mode. Whatever. I'll just try the version for the Raspberry Pi 4 with kernel 5.4.

In the following, I'll post a modified version of the [Readme](https://github.com/seemoo-lab/nexmon?tab=readme-ov-file#build-patches-for-bcm43430a1-on-the-rpi3zero-w-or-bcm434355c0-on-the-rpi3rpi4-or-bcm43436b0-on-the-rpi-zero-2w-using-raspbianraspberry-pi-os-recommended). There will be comments on where I strayed from the original version and why I did it. I tested the stuff below with commit `c381091d679993a0535082943d9b74a20f92b9f1` in the nexmon projecton a 64 bit system.

### How to build and install nexmon on a Raspberry Pi 5

First, add this to your `/boot/firmware/config.txt` (and then reboot to load the kernel with the correct parameters):

```
kernel=kernel8.img
reboot
```

This is not mentioned in the tutorial, but I got the following error message:

```
libm.so.6: ELF load command address/offset not page-aligned
```

[Turns out](https://www.reddit.com/r/raspberry_pi/comments/1an4a96/noip_duc_giving_an_error_on_raspberry_os_64_lite/): there are different memory alignments and nexmon cannot deal with that. Ok, now on to the actual installation guide

> - Make sure the following commands are executed as root: `sudo su`
> - upgrade your Raspbian installation: `apt-get update && apt-get upgrade`
> - Install the kernel headers to build the driver and some dependencies: `sudo apt install raspberrypi-kernel-headers git libgmp3-dev gawk qpdf bison flex make autoconf libtool texinfo xxd`
> - Clone our repository: git clone https://github.com/seemoo-lab/nexmon.git
> - Go into the root directory of our repository: cd nexmon

**Notes**: I added `xxd` to the list of dependencies, because it was not installed on my newly installed Raspberry Pi. Also: It is not necessary to run everything as root, just the installation stuff. However, since the build process requires some messing with environment variables, it is probably easier to just run everything as root. Next come some library installations:

> - `sudo dpkg --add-architecture armhf`
> - `sudo apt-get update`
> - `sudo apt-get install libc6:armhf libisl23:armhf libmpfr6:armhf libmpc3:armhf libstdc++6:armhf`

And then some symbolic links. **Important**: Don't run these command just like that. I for example had different (but compatible) versions installed, i.e. `libisl.so.23.2.0` and `libmpfr.so.6.2.0`. So check your library versions and adjust the commands below accordingly. If you don't, you will get the error `ERR: ram file empty or unavailable.` when you run `make`.

> - `sudo ln -s /usr/lib/arm-linux-gnueabihf/libisl.so.23.0.0  /usr/lib/arm-linux-gnueabihf/libisl.so.10`
> - `sudo ln -s /usr/lib/arm-linux-gnueabihf/libmpfr.so.6.1.0 /usr/lib/arm-linux-gnueabihf/libmpfr.so.4`
> 
> Then you can setup the build environment for compiling firmware patches
> - Setup the build environment: source `setup_env.sh`
> - Compile some build tools and extract the ucode and flashpatches from the original firmware files: `make`

In the following part, the readme gives you some options. In my case, only the one I present below works:

> Go to the patches folder for the bcm43455c0 chipset: `cd patches/bcm43455c0/7_45_206/nexmon`
> 
> - Compile a patched firmware: `make`
> - Generate a backup of your original firmware file: `make backup-firmware`
> - Install the patched firmware on your RPI3: `make install-firmware`

It says “RPI3”, but in this case it is the RPI5, obivously. Also: if you ran most of these parameters as non-root and used `sudo` to execute the ones that required root access: `make install-firmware` required the setup environment, so use `sudo -E make install-firmware`.

From here on, you can continue with the official nexmon readme file (it continues with “Install nexutil: from the root directory of our repository switch to the nexutil folder”). It explains you how to set up a separate network interface that is in monitor mode and so on, all of that worked for me more or less without problems. I think.

### Conclusions

This is a setup that worked for me, but I have not tested it thouroughly yet. I can capture WiFi data over the air, but I have not let it run for a long time, so I do not know how stable it is. When I find some more issues (and solutions), I will post that here.
