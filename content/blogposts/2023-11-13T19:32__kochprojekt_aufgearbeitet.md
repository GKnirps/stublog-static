---
title: Das Kochprojekt in neuem Glanz
filename: kochprojekt_aufgearbeitet
date: 2023-11-13T19:32:16+01:00
update-date:
tags: kochprojekt, markdown, bbcode, html
category: meta
summary: Ich habe endlich den letzten Kochprojekteintrag überarbeitet.
image:
language:
---

Ich habe endlich den letzten Eintrag meines [Kochprojektes](/categories/kochprojekt) überarbeitet. Das war lange in Arbeit und ich habe es Stück für Stück veröffentlicht. Meist, wenn ich das betreffende Rezept gerade gekocht habe, aber manchmal auch so.

Für die meisten von euch wird der einzige sichtbare Unterschied sein, dass der Name des Gerichts jetzt endlich auch im Titel des Blogposts auftaucht. Aber dahinter steckt noch ein bisschen mehr. Aber dazu muss ich etwas weiter ausholen…

### Das Kochprojekt

Im Jahr 2014, das Jahr, nachdem ich meinen [Masterabschluss gemacht habe](/blogposts/71), hatte ich mir ein Projekt vorgenommen: Ich wollte jede Woche etwas kochen, was ich bis dahin noch nicht gekocht habe. Im Großen und Ganzen [ein Erfolg](/blogposts/163).

Doch ein paar Schönheitsmakel gab es schon. Das Sichtbarste waren natürlich die Blogpost-Titel: Die hießen einfach „Kochprojekt Woche 1“, „Kochprojekt Woche 2“ usw. Das ist natürlich unpraktisch, wenn man sich [die Übersicht](/categories/kochprojekt) anschaut und ein spezielles Rezept sucht oder auch nur sehen möchte, welche Rezepte es denn gibt. Das wäre recht einfach zu beheben gewesen, aber es hätte auch Arbeit bedeutet, die ich mir damals nicht machen wollte.

Der zweite Schönheitsfehler, der schon damals bemerkbar war, war, dass bis irgendwann im September die [alt-Texte von Bildern](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt#usage_notes) einfach nur „Image“ waren. Warum, erkläre ich unten. Das ist doof, insbesondere für sehbehinderte Leute, die so keine Ahnung haben, was dort eigentlich zu sehen sein soll.

### Der neue Bloggenerator

2020 habe ich dann [die Blogsoftware umgestellt](/blogposts/neustart). Das bedeutete mehrere Dinge:

1. waren die Blogpost von da an in Commonmark verfasst, anstatt wie vorher in reinem HTML. Die alten HTML-Blogposts habe ich aber als solche in die neue Software importiert, um den Umstieg zu erleichtern.
2. habe ich ein paar Meta-Elemente hinzugefügt, z.B. eine meta-description und OpenGraph-Bilder, so dass meine Blogposts, wenn sie an bestimmten Stellen geposted werden auch direkt eine kurze Zusammenfassung und ggf. ein Bild erhielten.
3. Haben sich die URLs für die Bilder geändert (die alten URLs funktionieren aber weiterhin).
4. Habe ich irgendwann [automatisch Bilddimensionen zu eingebundenen Bildern hinzugefügt](/blogposts/lang_und_breit).

Von dieser Liste war nur Punkt 2 einfach zu erledigen.

#### HTML zu Markdown

Die Umwandlung von HTML zu Markdown ist eigentlich erstaunlich einfach. Dazu gibt es das [Python-Paket `markdownify`](https://pypi.org/project/markdownify/). Hier und da muss manchmal noch ein bisschen nachgearbeitet werden.

Aber ganz so einfach ist es nicht, denn Punkt 4 hängt davon ab, dass die Blogsoftware die Bilder auch findet. Und da in den alten Posts noch die alten Pfade verwendet wurden, und die sich nach Punkt 3 geändert haben, schmiert die Blogsoftware dann einfach ab, weil sie die Bilder nicht finden kann (das ist Absicht, damit ich nicht aus Versehen ungültige Pfade einbaue). Der einzige Grund, warum das vor der Umwandlung von HTML in Markdown funktionierte ist, dass die Bildgrößen nur bei Markdown eingebunden werden. HTML wird einfach 1:1 übernommen, ohne auf die URLs zu schauen.

Die Pfade mussten also alle angepasst werden. Auf der positiven Seite konnte ich dadurch auch die alten Pfade, die bisher einfach dieselben Dateien wie die neuen Pfade ausgeliefert haben, mit einem `301 Moved Permanently` umleiten, was etwas sauberer ist. Das habe ich vorher sein lassen, damit nicht jeder Aufruf nach einem Bild zwei Mal durchgeführt werden muss.

#### BB-Code

Die Umstellung von HTML zu Markdown war übrigens nicht die erste Umstellung. Irgendwann mitten im Kochprojekt habe ich damals die ursprüngliche [BB-Code-Codierung](https://de.wikipedia.org/wiki/BBCode) der Blogposts [auf reines HTML umgestellt](/blogposts/132). Gründe dafür gab es mehrere, aber der Hauptgrund war die Performance. Der in Ruby geschriebene BB-Code-Parser war einfach entsetzlich lahm, was insbesondere ein Problem war, weil die blogposts just-in-time gerendert wurden.

Ein nebensächlicher Grund war, dass ich nicht zufrieden war mit dem HTML, dass aus dem BB-Code herauskam. Zum einen war es kein semantisches HTML. Ich konnte zum Beispiel keine echten Überschriften machen, sondern musste auf Elemente wie `[b]` zurückgreifen, was als `<strong>` gerendert wurde, also eine starke Betonung, aber definitiv keine Überschrift. Auch konnte man keine alt-Texte für Bilder angeben (wie oben erwähnt). All das musste ich manuell nacharbeiten.

### Viel Arbeit, verteilt auf viel Zeit

Seit Anfang 2021 habe ich dann die alten Kochprojekteinträge überarbeitet. Meist (aber nicht immer) wenn ich das Rezept sowieso kochen wollte. Jeder Eintrag war schon Arbeit, und so zog sich die Sache dahin. Vor Kurzem habe ich endlich in einer Hauruck-Aktion die letzten zehn Einträge fertig gemacht. Nebenbei hier und da auch noch einen anderen Artikel, der noch die alten Bildpfade enthielt. Die anderen Artikel waren aber weitaus weniger zahlreich und hatten meist nur ein Bild, so dass die Arbeit deutlich geringer war.

Jetzt muss ich nur noch die letzten 144 Blogposts auf Markdown umstellen. Und dann bleibt immer noch einer, in dem eine Tabelle ist. Was ich mit dem anstelle, weiß ich noch nicht.
