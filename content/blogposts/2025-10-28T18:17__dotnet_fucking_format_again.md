---
title: dotnet fucking format again
filename: dotnet_fucking_format_again
date: 2025-10-28T18:17:14+01:00
update-date:
tags: c-sharp, dotnet, dotnet format, rant, code formatter, protobuf
category: rant
summary: dotnet format panikt und vergisst dabei alle Regeln
image:
image-alt:
language:
---

Ich habe heute einem Kollegen geholfen, der ein seltsames Problem hatte: Der Code-Formatter, den wir im Projekt verwenden, `dotnet format`, gibt einen Riesenhaufen an Fehlern aus und will alle Dateien reformatieren.

Nun [halte ich ja nicht viel von `dotnet format`](/blogposts/dotnet_fucking_format). Aber einfach alle Regeln vergessen und alle möglichen Regeln anwenden, die bei uns implizit oder explizit nicht aktiv sind? Das wäre doch ein bisschen viel. Also haben wir und die Sache angeschaut.

Zunächst: Das Problem trat nur auf dem Arbeitsbranch des Kollegen auf, nicht auf dem `main`-Branch. Also muss es irgendwas mit der Änderung zu tun haben. Die Änderung hat aber keine Konfigurationsdateien angefasst, nur ein paar C#-Quelldateien und zwei Protobuf-Definitionsdateien. Also Quellcode, der formatiert werden soll und Schnittstellendefinitionen, aus denen C#-Quellcode generiert werden soll. Nichts, was die Konfiguration von `dotnet format` beeinflussen sollte.

`dotnet format` hat aber auch eine Fehlermeldung ausgegeben, dass es irgendeine Projektdatei nicht lesen konnte, mit einem Hinweis, dass man mit der Option `-v diagnostic` mehr Informationen zu dem Fehler bekommt. Also haben wir das gemacht.

Das erste, was mir aufgefallen ist war, dass `dotnet format` durchaus alle unserer Konfigurationsdateien findet. Aber warum ignoriert es dann die Regeln? Dann kamen wir zu der Stelle, wo es sich beschwerte, irgendwelche Projektdateien in Verzeichnissen mit dem aus Protobuf generierten Code nicht finden zu können.

Sollte es doch Protobuf sein? Wir haben uns die Protobuf-Dateien noch einmal angeschaut und siehe da: Der Kollege hat beim Beheben eines Mergekonflikts versehentlich ein paar Typdefinitionen in der Protobuf-Datei dupliziert. Dadurch hatten wir dann doppelte Klassendefinitionen im generierten C#-Code. Durch den Compiler gegangen wäre das vermutlich nicht.

Trotzdem sehe ich hier einen großen Teil der Schuld an Protobuf. Ein einfacher Code-Formatter hätte sich nicht an einer doppelten Klassendefinition gestört. Ein Linter hätte gesagt: „Hey, das ist falsch, das geht so nicht.“. `dotnet format`, Formatter und Linter in einem, macht beides nicht. Stattdessen vergisst es alle Konfigurationen und wendet _jede einzelne Regel an, die es im System gibt_.
