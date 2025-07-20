---
title: Pacman contribution graph für GitHub und GitLab
filename: pacman_contribution_graph
date: 2025-07-20T16:22:23+02:00
update-date:
tags: pacman, github, gitlab, kunst, digitale kunst, digital art
category: hoellenmaschinen
summary: Ein Arbeitskollege hat ein Programm geschrieben, dass einen GitHub oder GitLab contribution graph in ein animiertes Pacman-Spiel verwandelt.
image:
image-alt:
language:
---

Wer mit GitHub oder GitLab arbeitet kennt diesen contribution graph: Das ist eine Matrix aus Feldern, mit einem Feld pro Tag des Jahres, in dem jedes Feld einen Farbton abhängig von der Anzahl commits hat, die ein Benutzer an diesem Tag gemacht hat.

Ein Arbeitskollege von mir hat [ein Programm geschrieben, dass diesen Graphen in ein animiertes Pacman-Spiel umwandelt](https://abozanona.github.io/pacman-contribution-graph/). Man kann es nicht spielen (es spielt sich von alleine) aber es ist animiert. Die Animation wird entweder auf ein canvas gezeichnet oder (eigentlich die coolere Variante) als animiertes SVG ausgegeben.

Der Kollege hat in einem Vortrag auch kurz erzählt, wie er das gemacht hat. So hat er zum Beispiel mit ein paar Tricks die Dateigröße der ausgegebenen SVG von über 5 MiB auf etwa 250 kiB verringert. Jemand anderes hat ihm außerdem ein paar Patches geschrieben, die das Verhalten der unterschiedlichen Geister mehr an das Originalspiel gebracht haben.

Ich würde hier gerne ein Beispiel zeigen, aber 250 kiB sind immer noch eine Menge, aber vor allem ziehen animierte SVG einiges an Rechenleiistung. Ich möchte nicht, dass jemand, der meine Seite aufruft, alleine deswegen 50% und mehr Auslastung auf einem CPU-Kern hat.

Trotzdem: ich liebe solche Projekte. [Der Code ist auf GitHub](https://github.com/abozanona/pacman-contribution-graph), da kann man sich auch eine Beispielanimation anschauen.
