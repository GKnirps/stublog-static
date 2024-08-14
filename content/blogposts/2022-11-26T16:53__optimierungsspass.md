---
title: Optimierungsspaß
filename: optimierungsspass
date: 2022-11-26T16:53:13+01:00
update-date: 2022-11-26T19:45:00+01:00
tags: rust, python, dga, optimierung, python
category: hoellenmaschinen
summary: Ich habe mal wieder nutzlose Optimierungsexperimente angestellt.
image:
---

Zu meinem momentanen Job gehört es unter anderem, Übungsaufgaben für Studenten eines Kurses zu erstellen. Das mache ich zusammen mit einem Kollegen, und für die nächste Aufgabe kommt etwas mit DGAs dran.

[DGA steht für „domain-name generation algorithm“](https://en.wikipedia.org/wiki/Domain_generation_algorithm) und bezeichnet Algorithmen, die nach bestimmten Regeln domain names erstellen, unter denen Bots in Botnets dann ihre Kontrollserver erreichen. Die Idee dabei ist, dass diese Domains sich häufig ändern, damit das Botnetz vor Entdeckung geschützt wird.

Dazu gibt es eine ganze Menge Forschung, das ist aber nicht mein Forschungsbereich. Wenn es um IT-Security geht, wäre es natürlich schön, schnell erkennen zu können welche Geräte infiziert und somit Teil eines Botnetzes sind. Ein Weg dazu ist, auf verdächtige Domainanfragen zu achten.

### Klassifizierung

Dafür braucht man einen automatischen Weg, reguläre Domains von DGA-generierten Domains zu unterscheiden. Da gibt es eine Menge Methoden, darum geht es hier aber nicht. Für unsere Aufgabe haben wir einen sehr simplen Klassifikator gebaut, der von harmlosen Domains weniger als 5% als generiert einstuft (false positive), von einfachen generierten Domains hingegen mehr als 50% (true positive).

Das ist nicht super gut, aber wir brauchen es ja nur für die Übungsaufgabe und es hat wirklich kaum Zeit gebraucht, das zu implementieren. Die Idee ist, dass die Studenten einen DGA entwickeln, dessen Domains nicht so leicht erkannt werden.

Aber nun genug zu DGAs. Wichtig ist: Wir haben ein Python-script geschrieben, dass über eine Liste von Domainnamen geht und für jeden Namen entscheidet, ob er generiert ist oder nicht. Dazu haben wir vier Kriterien verwendet.

1. Anzahl der Ziffern im Domainnamen
2. längste Folge von Konsonanten (also Zeichenfolgen ohne Vokale)
3. Verhältnis von Vokalen zu Konsonanten
4. [Entropie](https://de.wikipedia.org/wiki/Entropie_(Informationstheorie)) der Domain

Wenn irgendeine der Kategorien über einem gewissen Schwellwert liegt, wird die Domain als DGA-generiert eingestuft.

Zum Testen haben wir das ganze über ein paar Testlisten laufen lassen. Eine davon ist die [Tranco-list](https://tranco-list.eu/), eine Liste von einer Million populärer Websites (nach irgendeinem Ranking, dass ich mir nicht näher angeschaut habe). Wie oben schon erwähnt, gibt es hier knapp 5% false positives.

Die andere Testliste ist eine Liste von knapp 3,2 Millionen arithmetisch generierter Domainnamen, also praktisch zufällige Buchstaben-Zahlenkombinationen Hier haben wir eine Erkennungsrate vonetwa 55%.

Minimal raffiniertere DGAs, die z.B. darauf basieren, Wörter aus Wörterbüchern aneinanderzuhängen, erkennt unser Script praktisch überhaupt nicht. Wie gesagt, das war in diesem Kontext auch nicht wichtig.

### Optimierungen

Kommen wir nun zum eigentlichen Kern dieses Eintrags: Diese Klassifizierung über 3,2 Millionen Einträge laufen zu lassen kostet Zeit. Mit dem Python-Script etwa 30 Sekunden (oder 20 Sekunden auf meinem Arbeitsrechner).

Ich habe mit meinem Kollegen gescherzt, dass ich eine Rust-Version davon implementieren möchte, nur um zu sehen, wie viel schneller das sein kann. Nun war ich ein paar Tage krank und das war genau die Art von geistig weniger Anstrengender Tätigkeit, die ich noch machen konnte.

Außerdem haben ich ja schon [ein paar](/blogposts/benchmark) [Male](/blogposts/optimize_rust_build_time) [erwähnt](/blogposts/blog-optimization), dass es mir gelegentlich Spaß macht, an mehr oder weniger nutzlosen Dingen herumzuoptimieren.

Also habe ich einen Haufen Varianten davon in Rust geschrieben und getestet. Der gesamte [Code ist hier zu finden](https://gitlab.com/GKnirps/dga_detect). Als Input habe ich zum einen die [tranco-List](https://tranco-list.eu/) und zum Anderen eine Liste von arithmetisch generierten Domains, die mir einer der Professoren, die die Vorlesung halten, gegeben hat. Diese Liste stammt aus dem [DGArchive](https://dgarchive.caad.fkie.fraunhofer.de/welcome/), allerdings mit einiger Vorauswahl, so dass am Ende 3,2 Millionen Domains auf der Liste standen.

#### Messungen

Zum Messen der Zeit habe ich einfach nur `time` verwendet, die Ergebnisse waren hier recht stabil (im Gegensatz zum Beispiel zu [diesem Eintrag](/blogposts/benchmark)).

Allerdings nicht das bash-eigene `time` sondern `/usr/bin/time`, weil das mit dem `-v`-Flag noch einige weitere Informationen, zum Beispiel den maximalen Speichergebrauch, liefert.

Die Ausgaben mit `-v` sind recht lang, ich habe sie hier immer auf diese Zeilen zusammengekürzt:

```
User time (seconds):
System time (seconds):
Maximum resident set size (kbytes):
```

#### Testfall 1: Das python-Script

Um einen Vergleich zu haben, hier zunächst die Laufzeit des Python-Scriptes (`base.py` im Repo).

Für die generierten domains:
```
User time (seconds): 30.47
System time (seconds): 0.11
Maximum resident set size (kbytes): 270856
```

Und für die Tranco-Liste:
```
User time (seconds): 8.27
System time (seconds): 0.03
Maximum resident set size (kbytes): 86848
```

Der Speicherverbrauch kann sich sehen lassen. Die Eingabedateien sind 62 MiB bzw. 15 MiB groß, der Speicherbedarf des Scriptes ist ein Vielfaches davon.

#### Testfall 2: Das Gleiche in Rust

Ich habe die genaue Funktionsweise in Rust nachprogrammiert (`base.rs` im Repo). Die Laufzeiten hier sind für die Compileroptimierte Version (die Debug-Version ist sogar noch langsamer als die Python-Version).

Für die generierten Domains:
```
User time (seconds): 1.45
System time (seconds): 0.06
Maximum resident set size (kbytes): 64972
```

Und die Tranco-Liste:
```
User time (seconds): 0.40
System time (seconds): 0.03
Maximum resident set size (kbytes): 17256
```

Ohne großen Aufwand direkt zwanzig Mal schneller. Auch der Speicherbedarf ist deutlich geringer. Ich weiß nicht, wo der ganze Overhead in Python herkam (möglicherweise war der Garbage-Collector nicht schnell genug mit dem Aufräumen). Nichtsdestoweniger ist hier noch Optimierungspotential.

#### Testfall 3: Streaming

Die bisherigen Varianten haben immer die ganze Datei geladen und dann darauf gearbeitet. Das sorgt natürlich für hohen Speicherbedarf. Also habe ich es anders gemacht und einen buffered Reader verwendet, der die Datei [Zeilenweise einliest](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line), und zwar so, dass die gelesene Zeile jedes Mal in denselben Buffer geschrieben wird. Sonst hätten wir mehrere Millionen Speicherallokationen, und das wäre kontraproduktiv.

(Datei `streaming.rs` im Repo.)

Für die generierten Domains:
```
User time (seconds): 1.53
System time (seconds): 0.00
Maximum resident set size (kbytes): 2420
```

Und für die Tranco-Liste:
```
User time (seconds): 0.43
System time (seconds): 0.00
Maximum resident set size (kbytes): 2564
```

Wir sehen: Es ist ein bisschen langsamer, aber der Speicherverbrauch ist deutlich geringer. Also behalten wir das mal bei und versuchen, die Geschwindigkeit anderswo wieder hereinzuholen.

#### Testfall 4: floating point precision

Bisher wurden alle Berechnungen mit 64-Bit-Fließkommazahlen (`f64` in Rust) vorgenommen. Eine solche Präzision brauchen wir hier nicht, also habe ich mal ausprobiert was passiert, wenn ich nur 32-Bit-Fließkommazahlen (`f32` in Rust) nehme (Datei `f32.rs` im Repo).

Für die generierten Domains:
```
User time (seconds): 1.36
System time (seconds): 0.01
Maximum resident set size (kbytes): 2544
```

Für die Tranco-Liste:
```
User time (seconds): 0.38
System time (seconds): 0.01
Maximum resident set size (kbytes): 2492
```

Wow. Ich hatte nicht wirklich gedacht, dass das so einen Unterschied macht. Das Klassifizierungsergebnis war übrigens das Gleiche wie in der 64-Bit-Fließkommazahl-Version, also keine Verluste hier.

Dennoch, die Verbesserung war eher klein. Wie weit kommen wir noch runter (ohne viel Aufwand zu betreiben)?

#### Testfall 5: Rohe bytes

Mein nächster Gedanke war: Die Domains sind alle mehr oder weniger ascii, und wenn ich alles in einen `String` lade muss Rust muss einen kleinen Overhead betreiben um sicherzustellen, dass alles gültiges Unicode ist, außerdem wird das Iterieren über die [scalar values](/blogposts/unicode_scalar_value) mühsamer, weil die Strings intern in utf-8 codiert sind.

Also die f32-Version genommen, aber alles auf byte arrays umgestellt (Datei `bytes.rs` im Repo).

Die generierten Domains:
```
User time (seconds): 1.91
System time (seconds): 0.00
Maximum resident set size (kbytes): 2620
```

Die Tranco-Liste:
```
User time (seconds): 0.52
System time (seconds): 0.00
Maximum resident set size (kbytes): 2536
```

Huch. Das ist deutlich langsamer. Ich weiß nicht genau, warum. Vermutlich wird schon recht gut optimiert und ich habe irgendwo eine Entscheidung getroffen, die alles langsamer macht. Auf jeden Fall werde ich diesen Ansatz nicht länger verfolgen.

#### Testfall 6: Weniger `chars`-Iteratoren

Ok, die Idee mit dem bytearrays war ein Irrweg. Das ändert aber nichts an der Sache, dass ich den [`chars()`-Iterator](https://doc.rust-lang.org/std/primitive.str.html#method.chars) über den String verwende. Das ist mehr Aufwand als einfach über ein Array zu iterieren. Und ich benutze diesen Iterator gleich fünf Mal:

- um die Ziffern zu zählen
- um die längste Konsonantenfolge zu zählen
- um die Vokale zu zählen
- um die Konsonanten zu zählen
- um die Entropie zu berechnen

Ich lasse die Entropieberechnung mal außen vor, die steckt isoliert in ihrer Funktion und das ist gut so. Die anderen Sachen kann ich aber bequem in einer Schleife erledigen. Das ist vielleicht nicht mehr so funktional, aber vielleicht macht es die Sache ja schneller (Datei `one_char_iterator.rs` im Repo).

Generierte Domains:
```
User time (seconds): 1.06
System time (seconds): 0.02
Maximum resident set size (kbytes): 2484
```

Tranco-Liste:
```
User time (seconds): 0.31
System time (seconds): 0.00
Maximum resident set size (kbytes): 2548
```

Wow. Nach dem Fiasko mit den byte-arrays hätte ich nicht gedacht, dass aus dieser Optimierung so viel herauszuholen ist. Aber so langsam geht mir die Puste aus. Also nur noch ein weiteres Experiment.

#### Testfall 7: Simplify

Diese Optimierung ist nicht ganz so neutral wie die anderen. Die anderen Optimierungen haben im schlimmsten Fall den Code etwas unschöner gemacht. Diese Optimierung hingegen entfernt ein paar Features, die aber ungenutzt waren.

Hintergrund ist: Im Python-Script haben wir ein paar Mechanismen eingebaut, mit denen die Studenten einfacher in die Bewertung eingrätschen konnten um den Klassifikator anzupassen. Das hat u.a. zur Folge, dass mehr Berechnungen durchgeführt werden, aber viel wichtiger, dass es keinen early exit gibt.

Die Entropieberechnung zum Beispiel enthält eine Menge Fließkommaoperationen (u.a. Logarithmusberechungen). Wenn wir uns aufgrund eines der anderen Kriterien schon entschieden haben sollten, dass die Domain generiert ist, müssen wir die Entropie nicht mehr berechnen.

Also habe ich diese ungenutzen Parameter herausgeworfen (Datei `less_flexible.rs` im Repo).

Generierte Domains:
```
User time (seconds): 0.71
System time (seconds): 0.01
Maximum resident set size (kbytes): 2524
```

Tranco-Liste:
```
User time (seconds): 0.31
System time (seconds): 0.01
Maximum resident set size (kbytes): 2484
```

Was sagt man dazu? Das Ergebnis wurde noch einmal ordentlich verbessert. Allerdings nur dort, wo es eine nennenswerte Anzahl von early-Exits gab. Bei der Tranco-Liste, wo die meisten Domains korrekterweise als harmlos eingestuft wurden, muss für alle diese Domains auch die Entropie berechnet werden.

### Fazit

Was habe ich daraus gelernt? Dass eine Rust-Implementierung schneller sein wird als eine Python-Implementierung ist keine große Überraschung.

Aber es gab ein paar Überraschungen. Manche Optimierungen waren unerwartet effektiv. Andere waren hingegen Kontraproduktiv.

Also gilt, denke ich mal, nach wie vor die Regel: Optimiere nicht zu früh, und wenn du optimierst, stelle sicher, dass die Änderungen auch wirklich etwas verbessern.
