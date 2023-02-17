---
title: Verschloptimierung
filename: optimization_fail
date: 2023-02-17T20:45:11+01:00
update-date:
tags: optimierung, rust, curl, brainfuck
category: hoellenmaschinen
summary: Wie eine versuchte Code-Optimierung alles schlimmer machte. Zusätzlich dazu ein paar Resourcen zu Code-Optimierung.
image:
---

Diesen Blogpost schiebe ich schon eine Weile vor mir her. Glücklicherweise hat sich in der Zeit mehr Material für den Blogpost angesammelt, so dass ich es nicht bereue, ihn jetzt erst zu schreiben.

### Resourcen zu Optimierungen

Kurz nachdem ich meinen [letzten Blogpost über Optimierung](/blogposts/optimierungsspass) geschrieben habe, bin ich auf einen [Vortrag aus 2019](https://www.youtube.com/watch?v=r-TLSBdHe1A) aufmerksam geworden. Darin geht es auch um Optimierung von Programmen, und dass es nicht immer offensichtlich ist, ob eine Verbesserung statistisch signifikant ist und ob die vorgenommene Änderung wirklich Ursache für die bessere Performance war. Oft genug sind es einfach Nebeneffekte der Compileroptimierung, die halbzufällig anders arbeiten.

Wenn man ernstzunehmende Optimierung machen will, ist der Vortrag sehenswert. Da die Optimierungen, die ich bisher in diesem Blog beschrieben habe mehr aus Spaß als aus ernstzunehmender Notwendigkeit entsprungen sind, habe ich mir so aufwändige Analysen gespart.

Noch ein paar Tage später hat dann Daniel Stenberg, der Maintainer von curl [einen Blogpost](https://daniel.haxx.se/blog/2022/12/06/faster-base64-in-curl/) über eine Optimierung des base-64 encoders in curl geschrieben. Auch durchaus lesenswert.

Und dann, noch ein bisschen später, habe ich [diesen Vortrag](https://www.youtube.com/watch?v=JRMOIE_wAFk) über code-profiling in rust gelesen (gefunden dank [This Week in Rust](https://this-week-in-rust.org/). Ein schöner Vortrag, die meisten Methoden habe ich noch nicht angewandt.

### Fallstudie: Ein Brainfuckinterpreter

Nun zu dem, was ich gemacht habe. Irgendwann Ende Januar ist mir ein Repo mit einem rust-Brainfuckinterpreter aufgefallen, der noch bei mir herumlag. Aus dem Jahr 2016, also kurz nachdem ich überhaupt mit rust angefangen habe. Rust-Edition: pre-2018.

Vermutlich etwas, was ich geschrieben habe, um mich in rust einzuarbeiten. Der Code war nicht das, was ich als „idiomatisches rust“ bezeichnen würde. Aber die erste Verbesserung war, einen Bug in der Schleifenbehandlung zu beheben.

#### Ausgangslage

Dannn konnte ich die Zeit auf einem in brainfuck geschriebenen Primzahlenberechner messen. Unglücklicherweise war das Ergebnis ungefähr auf einem Niveau des [Brainfuckinterpreters, den ich 2009 in C geschrieben habe](/blogposts/old_1489570). Und der benutzte eine doppelt [verkettete Liste](https://www.youtube.com/watch?v=YQs6IC-vgmo) als Band, weil ich das damals für eine gute Idee hielt.

Glücklicherweise konnte ich die mittlere Laufzeit (von etwa 13,5 Sekunden) um etwa zwei Sekunden verbessern, indem ich einfach das tat, was ich sowieso wollte: idiomatisches Rust schreiben. Vorher hatte ich irgendwelche umständlichen Dinge mit closures gemacht, jetzt parse ich den Input als Liste von `enum`s und lasse den Interpreter dann darüber laufen.

#### Die Optimierung, die alles schlimmer machte

Der nächste Schritt war dann, ein bisschen Optimierung zu betreiben. Meine Idee war: Hey, wir haben häufig viele gleichartige Operationen hintereinander. `+` und `-` zum Beispiel. Ich kann mehrere `+`- und `-`-Operationen zusammenfügen, anstatt also zum Beispiel für `+++++` fünf mal die aktuelle Zelle zu inkrementieren, addiere ich ein Mal `5` zur aktuellen Zelle.

Gesagt, getan. Das Ergebnis: Ich bin wieder auf dem Niveau meines C-Brainfuckinterpreters. Warum? Ich weiß es nicht. Meine Vermutung: Irgendwelche Compileroptimierungen, die jetzt nicht mehr so gut greifen. Oder irgendwelche CPU-Optimierungen, die für die unoptimierte Variante besser greifen. Hier sei noch einmal auf den ersten Vortrag, den ich oben verlinkt habe, verwiesen.

Vielleicht war es inlining? Nein, selbst wenn ich den Compiler mit der Nase darauf stoße, dass er inlinen soll, verändert sich nichts. Vermutlich inlined er hier sowieso. Ich sollte mir also den zweiten Vortrag, den ich oben verlinkt habe, zu Herzen nehmen und ein gründliches Profiling machen. Wäre eine nette Übung. Aber nicht jetzt.
