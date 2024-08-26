---
title: Die dreifache Hölle der Nextcloud-Konfiguration
filename: nextcloud_config
date: 2024-08-22T21:29:11+02:00
update-date:
tags: rant, nextcloud, php, nginx
category: hoellenmaschinen
summary: Nextcloud spuckt freundlicherweise Warnungen aus, wenn irgendwas in der PHP- oder der nginx-Konfiguration nicht so ist, wie erwartet. Eigentlich ganz gut, aber wie man das korrigiert ist schwierig herauszufinden. Und warum muss ich überhaupt für PHP Einstellungen machen?
image:
image-alt:
language:
---

Ich betreue auch eine kleine [Nextcloud](https://nextcloud.com/)-Instanz. Eigentlich ganz schön, so kommt man hier und da um große Anbieter wie Google herum.

Nextcloud hat auf der Admin-Übersichtsseite eine nette Funktion, die einem sagt, welche Konfigurationen alle falsch aussehen. Aber wenn die so falsch sind, warum kann Nextcloud die nicht selber korrigieren? Das liegt daran, dass diese Konfigurationen außerhalb von Nextclouds Einflussbereich liegen.

So beschwert es sich über einige Fehler in der nginx-Konfiguration. Da fängt es schon an. Ich brauche eine ziemlich komplizierte nginx-Konfiguration, um nextcloud überhaupt zum Laufen zu kriegen. Ein „leite einfach alles an Nextcloud weiter und lass Nextcloud entscheiden, was es macht“ reicht nicht. Ich muss einen Haufen Routen konfigurieren, Sicherheitsheader, Umleitungen, das ganze Programm.

Gut, das muss ich für z.B. dieses Blog hier auch machen. In diesem Blog gibt es aber einen wesentlichen Unterschied: Es ist nur ein Haufen Dateien, da muss man nginx halt sagen, was es damit machen soll. Und das ist auch nicht so schwierig, das läuft wesentlich einfacher als die Konfiguration für Nextcloud. Man könnte fast sagen, Nextcloud ist keine abgeschlossene Software, sondern ist für Grundfunktionalitäten von nginx (oder apache, den ich aber nicht benutze) abhängig.

Warum ist das so? Ich weiß es nicht, aber ich habe eine Vermutung. Nextcloud ist in PHP geschrieben, und PHP hat so eine seltsame Abhängigkeit zur FastCGI-Schnittstelle eines Webservers. Das ist eigentlich auch keine Entschuldigung, eigentlich müsste Nextcloud trotzdem seinen eigenen Kram regeln können. Tut es aber nicht. Stattdessen verlangt es sogar, dass ich auch noch in der PHP-Konfiguration herumschraube, damit es ordentlich läuft. Da stehen dann so Sachen drin, wie viel Arbeitsspeicher es maximal verwenden darf, oder timeouts für Requests.

Man stelle sich das mal bei einer anderen Scriptsprache vor. Wenn ich zum Beispiel bei Python erst einmal in einer globalen Konfigurationsdatei festlegen müsste, dass es mehr als 128MiB Speicher verwenden darf. Aber damit hört es natürlich nicht auf. In der PHP-Konfiguration gibt es nicht eins, nicht zwei, sondern _drei_ Verzeichnisse mit Konfigurationsdateien. Gut, `apache2` wird es nicht sein, ich nutze ja nginx. `cli` wohl auch nicht, das wird vermutlich nur für Kommendizeilenprogramme verwendet. Also bleibt eigentlich nur `fpm`.

Nur leider bringt das nichts. Nextcloud bleibt bei seinen 128MiB Speicher. Selbst wenn ich die Limits auch in den anderen Dateien anpasse. Online findet man jede Menge zu den diversen Problemen, nichts davon hilfreich. Di Dokumentation (sogar in der Warnung verlinkt) geht auf diese Details überhaupt nicht ein.

Nun will ich ja nicht sagen, dass diese Einschränkungen nicht wichtig sind. Aus Sicherheitsgründen oder so. Aber wenn es dafür notwendig ist, diese Einstellungen über drei verschiedene Programme hinweg zu machen, während es _grundsätzlich_ auch möglich wäre, das alles in einem Programm zu machen, dann ist beim Design der ganzen Sache irgendwo etwas schief gelaufen.

Ich habe jetzt mal wieder mehrere Stunden investiert und immer noch sind diese blöden Warnungen da, und niemand kann mir auch nur erklären, was die meisten davon eigentlich bedeuten. Also bleiben die jetzt erst einmal. Und das wirklich Nervige: _eigentlich_ wollte ich nur herausfinden, warum seit einiger Zeit Bilder nicht mehr in nextcloud selber angezeigt, sondern mit immer direkt zum Download angeboten werden, wenn ich auf sie klicke.
