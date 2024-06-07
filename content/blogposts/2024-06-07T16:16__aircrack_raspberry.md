---
title: WiFi-Sniffing auf einem Raspberry Pi 5
filename: aircrack_raspberry
date: 2024-06-07T16:16:37+02:00
update-date:
tags: raspberry pi, wifi, sniffing, linux, aircrack-ng, debian
category: hoellenmaschinen
summary: WiFi-Sniffing von einem Raspberry Pi 5 ist nicht ganz so einfach wie gedacht. Wie es bei mir funktioniert hat.
image:
language:
---

### Das Problem

Für mein aktuelles Forschungsprojekt muss ich ein bisschen WiFi-Sniffing machen. Also einfach den Raspberry Pi ausgepacken und darauf [airodump-ng](https://www.aircrack-ng.org/) laufen lassen?

Grundsätzlich würde das so funktionieren: Erst die WLAN-Karte in den monitor-mode versetzen, dann mit `airodump-ng` sniffen. Für den ersten Teil bringt aircrack-ng netterweise auch ein tool mit, dass anbietet, andere prozesse zu töten, die gerade die WLAN-Karte benutzen (wichtig: wenn ihr das zu Hause macht: Damit schaltet ihr die reguläre WLAN-Verbindung auf eurem Rechner aus, sorgt also ggf. dafür, dass ihr noch per Kabel am Netz seit).

```
sudo airmon-ng start wlan0 
```

`wlan0` ist hier die WLAN-Schnittstelle. Normalerweise sieht das dann so aus:

```
PHY     Interface       Driver          Chipset

phy0    wlan0           brcmfmac        Broadcom 43455
        wlan0 is soft blocked, please run "rfkill unblock 1" to use this interface.
rfkill error, unable to start wlan0

Would you like to try and automatically resolve this? [y/n] y
```

danach stellt er das Interface in den monitor mode. Auf meinem Raspberry Pi mit Debian 12 kam nach den Zeilen da oben aber das hier:

```
ERROR adding monitor mode interface: command failed: Operation not supported (-95)
```

### Lösungssuche

Huh. Das überrascht mich jetzt. Geht das wirklich nicht? Schauen wir mal in die [aircrack-doku](https://www.aircrack-ng.org/doku.php?id=airmon-ng#error_adding_monitor_mode_interfacecommand_failedoperation_not_supported_-95). Da steht zu diesem Problem unter Anderem:

> Even though dmesg says the interface is already in monitor mode and “iw dev wlan0 info” confirms it is, airodump-ng will fail and report the interface data linktype is Ethernet. This is a bug in the driver and/or firmware, and the workaround is to reboot the system or to reload the driver: 
> ```
> rmmod brcmfmac
> modprobe brcmfmac
> ```

Problem: Module neu laden hilft nicht. Systemneustart hilft nicht… Firmwareupdates helfen nicht. Doof. Dann bin ich auf [diesen Forumsthread gestoßen](https://forums.raspberrypi.com/viewtopic.php?t=253695). Dort hatte jemand das gleiche Problem, die Lösung war, „nexmon“ neu zu bauen. Nur was ist „nexmon“?

### Nexmon to the rescue!

[Nexmon](https://github.com/seemoo-lab/nexmon) ist

> Nexmon is our C-based firmware patching framework for Broadcom/Cypress WiFi chips that enables you to write your own firmware patches, for example, to enable monitor mode with radiotap headers and frame injection.

Klingt genau nach dem, was ich brauche. Und da ist sogar ein ausführliches Readme dabei! Also an die Arbeit!

Leider ist es nicht ganz so einfach, denn nicht alles ist auf dem aktuellsten Stand. Mein WiFi-Chipset wird zwar erwähnt, aber nur im Zusammenhang mit Raspberry Pi 3 und 4. Außerdem nur mit Kernelversionen 4.x und 5.x. Ich bin bei 6.6.31. Ach ne, da ist auch eine Eintrag für den Raspberry Pi 5. Aber ohne Monitor-Mode. Ok, probieren wir es einfach einmal mit der Version für den Raspberry Pi 4 und Kernel 5.4. Im Großen und Ganzen folge ich der Anleitung, aber einige wichtige Abweichungen erwähne ich unten.

Die [Anleitung im Readme](https://github.com/seemoo-lab/nexmon?tab=readme-ov-file#build-patches-for-bcm43430a1-on-the-rpi3zero-w-or-bcm434355c0-on-the-rpi3rpi4-or-bcm43436b0-on-the-rpi-zero-2w-using-raspbianraspberry-pi-os-recommended) gibt mir erst einmal ein paar Sachen, die installiert sein müssen. Was fehlt: `xxd`, wie ich kurz danach feststelle. Also wichtig: das Paket `xxd` auch noch installieren.

Danach sagt das Readme, dass ich einige symbolische links auf Bibliotheken setzen soll. Die Ziele dieser Links existieren bei mir aber so nicht. Stattdessen habe ich leicht andere (aber laut Versionsnummer kompatible) Versionen. Wenn ich das nicht mache, kommt später der Fehler `ERR: ram file empty or unavailable.` Für mich sieht das also so aus:

```
sudo ln -s /usr/lib/arm-linux-gnueabihf/libisl.so.23.2.0  /usr/lib/arm-linux-gnueabihf/libisl.so.10
sudo ln -s /usr/lib/arm-linux-gnueabihf/libmpfr.so.6.2.0 /usr/lib/arm-linux-gnueabihf/libmpfr.so.4
```

Der geneigte Leser möchte die Versionen vermutlich der eigenen Installation anpassen. Auch wichtig: wenn man schon einmal ein `make` ausgeführt hat wurden hier teilweise schon Sachen falsch gebaut. Dummerweise gibt es hier kein `make clean`, also habe ich einfach ein paar Sachen auf Gutdünken gelöscht (`patches` zum Beispiel, oder `firmwares`, und den Grundzustand dann mit `git restore .` wiederhergestellt.

Dann ins Unterverzeichnis unter `patches`. Da werden mehrere Optionen angegeben. Für mich ist

```
cd patches/bcm43455c0/7_45_206/nexmon
```

die richtige Option.

Dann wieder `make`. Aber was müssen meine müden Augen lesen?

```
libm.so.6: ELF load command address/offset not page-aligned
```

Es ist immer irgendetwas. [Stellt sich heraus](https://www.reddit.com/r/raspberry_pi/comments/1an4a96/noip_duc_giving_an_error_on_raspberry_os_64_lite/): es gibt unterschiedliche memory-alignments, und nexmon kommt nicht damit klar. Lösung: in `/boot/firmware/config.txt` ganz oben die Zeile

```
kernel=kernel8.img
```

eintragen. Danach natürlich booten. Wichtig: Durch das rebooten muss man natürlich das `source setup_env.sh` noch einmal machen-

Die Anleitung hat auch eine Anweisung für das Backup der alten Firmware. Nur fürs Protokoll, `make backup-firmware` macht das hier:

```
cp /lib/firmware/brcm/brcmfmac43455-sdio.bin brcmfmac43455-sdio.bin.orig
```

Dann die Firmware mit `make install-firmware` installieren. Doch halt! Sollte mit root-Rechten gemacht werden. Aber dann geht ja das `source setup_env.sh` verloren. Also lieber:

```
sudo -E make install-firmware
```

mit `-E` bleibt die environment erhalten. Ist aber eventuell nicht überall erlaubt.

Danach weiter der Anleitung folgen, nicht von Fehlermeldungen abschrecken lassen und vielleicht manches einfach noch einmal probieren. Bei mir läuft es momentan. Wie es langfristig läuft, muss ich noch schauen. Wenn es noch weitere wichtige Hinweise gibt, schreibe ich das hier später noch einmal.
