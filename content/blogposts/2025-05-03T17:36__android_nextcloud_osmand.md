---
title: Android, Nextcloud OsmAnd und ein GPX-Track
filename: android_nextcloud_osmand
date: 2025-05-03T17:36:24+02:00
update-date:
tags: android, osmand, openstreetmap, nextcloud, gps, rant
category: hoellenmaschinen
summary: Ich will nur automatisch eine aufgezeichnete Route hochladen. Und wenn der Prophet nicht zum Berg kommt, muss der Berg halt zum Propheten kommen. Dummerweise will sich auch der Berg keinen Millimeter bewegen.
image:
image-alt:
language:
---

Ich war kürzlich im Urlaub. Wir haben einige schöne Wanderungen gemacht, und ich habe für ein paar dieser Wanderungen auch die Geokoordinaten der Wanderung aufgezeichnet. Dafür habe ich [OsmAnd](https://osmand.net/) benutzt, eine Android-App, die Kartenmaterial von [Openstreetmap](https://www.openstreetmap.org) verwendet (es gibt auch Tracker ohne Kartenmaterial, aber ich fand es praktisch, die Route dann direkt auf der Karte zu sehen).

Damit ich diese Aufzeichnungen nicht verliere, wenn meinem Phone etwas zustößt, habe will ich sie zu meiner privaten [Nextcloud](https://nextcloud.com/)-Instanz hochladen, und zwar automatisch. Auch dazu gibt es eine praktische Android-App. Dort kann man einen Ordner angeben, aus dem alle erstellten Dateien automatisch hochgeladen werden.

OsmAnd speichert die Tracks im [GPS Exchange Format](https://de.wikipedia.org/wiki/GPS_Exchange_Format), einem verbreiteten Format. Sollte also alles eigentlich einfach sein, oder? Aber die Tatsache, dass ich diesen Blogpost schreibe ist ja ein klarer Hinweis, dass dem nicht so ist.

OsmAnd speichert die Daten in seinem App-Verzeichnis, also `/Android/data/net.osmand.plus/files/tracks/rec`. Auf dieses Verzeichnis dürfen aber keine anderen Apps zugreifen. Ist vermutlich ein Sicherheitsfeature und [das Problem ist den Nexcloud-Entwicklern bekannt](https://github.com/nextcloud/android/issues/10500), sie können aber nichts machen:

> Not even file managers (including the android built-in Files) can see inside `Android/data`. There may be some phones in which this works, but generally it is not expected to work. Additionally it would take a significant work for us to even start integrating this, as it would mean interacting with the filesystem in a different way than we do currently.

Nun gut, mein file manager kann das, aber ich habe auch /e/OS als Android-Variante. Andere Apps können dort trotzdem nicht zugreifen.

Ärgerlich. Benutzerdaten im Verzeichnis des Programms anstatt im allgemeinen Benutzerverzeichnis abzulegen hat selbst die Windows-Welt schon 2001 mit Windows XP hinter sich gelassen (was zu einigem Ärger mit älteren Spielen führte, die ihre Daten unbedingt im – nun schreibgeschützten – Programmverzeichnis ablegen wollten).

Also, was tun? Man kann doch sicher in OsmAnd umstellen, wo wie Tracks abgelegt werden sollen. Also Menü → Trip recording… ne, da ist nichts. Vielleicht Menu → Settings → [Profil auswählen] → Trip recording? Ah, da ist es! „Track storage folder“. Allerdings… kann man da _keinen beliebigen Ordner einstellen_. Der größte Teil des oben genannten Pfades bleibt konstant, nur den Ordner `rec` ganz am Ende kann man ändern in „Store recorded tracks in monthly folders“. Will ich nicht.

Ein verzweifelter Versuch, einen Ordner im allgemein zugänglichen (sofern Erlaubnis dafür erteilt wurde) Bereich anzulegen und dann per Kommandozeilen-App einen Symlink dorthin in das OsmAnd-Verzeichnis zu legen scheiterte daran, dass die Kommandozeilen-App natürlich auch keinen Zugriff auf `/Android/data` kriegt.

Die letzte Rettung wäre vielleicht, unter Menü → Settings → OsmAnd settings → Data storage Folder den Speicherort für _alle_ OsmAnd-Daten zu verschieben, aber ich will ja nur die Tracks, außerdem muss ich dann vielleicht ja alle Karten neu herunterladen oder manuell verschieben oder so, weil die ja auch in dem Ordner gespeichert sind.

Also bleibt mit nur, alle tracks manuell zu exportieren und zu importieren, damit sie dann hochgeladen werden. Doof.
