---
title: Die laufende Figur in Unicode 16 🮲🮳
filename: running_man_unicode
date: 2024-06-26T18:53:22+02:00
update-date:
tags: unicode, macintosh II
category: hoellenmaschinen
summary: Die nächste Unicode-Version hat auch einen alten Macintosh II-Zeichensatz übernommen.
image:
image-alt:
language:
---

Das Ziel von [Unicode](https://home.unicode.org/about-unicode/) ist es ja, einen universellen [Zeichensatz](https://de.wikipedia.org/wiki/Zeichensatz) bereitzustellen. Dazu gehört, dass alle anderen Zeichensätze verlustfrei in Unicode übersetzt werden können. Das führt mitunter auch dazu, dass auch sich auch Zeichen, dessen [Herkunft und Bedeutung ein Mysterium sind](/blogposts/angzarr) in Unicode wiederfinden.

Unicode kriegt auch immer wieder Updates, weil immer wieder auffällt, dass manche [Schriften noch nicht repräsentiert sind](https://modelviewculture.com/pieces/i-can-text-you-a-pile-of-poo-but-i-cant-write-my-name). Oder dass wir anscheinend noch mehr Emojis brauchen. Oder auch, weil die [Zeichensätze von irgendeinem alten System](https://www.unicode.org/charts/PDF/U1FB00.pdf) noch nicht in Unicode abbildbar waren.

So zum Beispiel der Zeichensatz des [Macintosh II](https://de.wikipedia.org/wiki/Macintosh_II). Ach, und welche Zeichen sollen da schon drin sein, die es nicht auch anderswo gibt? Naja, wie eine paar [Leute auf Mastodon herausgefunden haben](https://mastodon.social/@mcc/112674803793613153) gab es da auch Zeichen, die zusammengesetzt eine laufende Figur abbilden.

Nun ist das [Unicode 16](https://www.unicode.org/versions/Unicode16.0.0/), das ist noch nicht veröffentlich und kann sich noch ändern. Ich hoffe aber, das ändert sich nicht, ansonsten habe ich in ein paar Monaten ein Problem mit ungültigem Unicode oder falschen Zeichen in diesem Blogpost. Die laufende Figure ist diese hier:

```
🮲🮳
```

Bei mir wird das schon korrekt angezeigt, auf den meisten Windows/MacOS/Android-Geräten wohl noch nicht. Das ist aber einfach nur ein Problem mit der Schriftart. Wenn keine installierte Schriftart diese Glyphen enthält, dann kann das nicht angezeigt werden (und da Unicode 16 noch nicht veröffentlicht ist, kann man auch niemandem einen Vorwurf machen).

Für die, bei denen das noch nicht dargestellt wird:

![Eine rennende Strichfigur](/file/unicode_running_man.webp)

Und das Schöne ist: Weil man den alten Zeichensatz ja verlustfrei nach Unicode convertieren können muss (ein Text nach Unicode convertiert muss also zurückconvertierbar sein und wieder den Ursprungstext ergeben), besteht diese rennende Figur, wie im Original, aus zwei code points, die man beliebig voneinander trennen kann. Ich kann die Figur also auch durch ein Portal gehen lassen:

```
🮲▏   ▕🮳
```

### Technische Details

Anscheinend hatte der Macintosh II auch das Apple-Logo im Zeichensatz, das konnte aber aus Markengründen nicht in Unicode aufgenommen werden.

Die zwei Codepoints für diese Figur sind übrigens

```
U+1FBB2 LEFT HALF RUNNING MAN
U+1FBB3 RIGHT HALF RUNNING MAN
```

Die legacy-Symbole sind eine wahre Fundgrube für außergewöhnliche Zeichen. Ich kann nur empfehlen, sich da einmal durchzuwühlen.
