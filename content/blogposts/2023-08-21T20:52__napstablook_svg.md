---
title: Napstablook
filename: napstablook_svg
date: 2023-08-21T20:52:55+02:00
update-date:
tags: chaos communication camp, svg, napstablook, undertale, optimierung, inkscape
category: computerspiele
summary: Ich habe eine Vektorgrafik von Napstablook aus Undertale erzeugt. Hier eine viel zu detaillierte Entstehungsgeschichte.
image: /file/napstablook_original.webp
language:
---

> Here comes Napstablook

Es folgt eine viel zu detaillierte Darstellung, wie ich eine Vektorgrafik von Napstablook erstellt habe. Lesen auf eigene Gefahr.

![Ein traurig aussehender Geist. Das Bild hat eine sehr geringe Auflösung](/file/napstablook_original.webp "REALLY NOT FEELIN UP TO IT RIGHT NOW. SORRY.")

Dieses Bild stammt aus dem Spiel Undertale von Toby Fox. Es zeigt den Charakter Napstablook. Wer das Spiel gespielt hat, wird Napstablook erkennen. Wer nicht… nun ja, ich habe ja schon einmal [erwähnt](/blogposts/undertale), dass man Undertale besser ungespoilert spielt.

Nun bin ich ein bisschen ein Fan von Napstablook. Auch Napstablooks Kampfmusik ist durchaus zu empfehlen, am besten in der [Jazz-Cover-Version](https://www.youtube.com/watch?v=yeJx04_47Lc). 

Aber das originale Bild aus dem Spiel ist einfach so klein. Und verpixelt. Das passt zwar gut zum Stil von Undertale, ist aber unpraktisch, wenn ich es woanders verwenden möchte. Zwar gibt es jede Menge Fanart von Napstablook, aber da bin ich mir mit den Rechten nie sicher, und außerdem möchte ich eine Version, die möglichst nah am Original ist.

Also habe ich selber eine gemacht:

![Ein traurig aussehender Geist](/file/napstablook.svg "i'm fine, thanks.")

Man sieht es hier nicht, weil die Grundmaße dieselben sind wie bei der Rastergrafik oben, aber da es eine SVG ist, kann man sie beliebig skalieren.

### Entstehung

Wie bin ich dahin gekommen? Nun, ich habe mir einfach die Originaldatei geschnappt, und in [inkscape](https://inkscape.org) mit Bezierkurven nachgezeichnet.

Das war es eigentlich schon. Warum habe ich dann eine viel zu detaillierte Beschreibung versprochen?

Zuerst einmal, weil die SVG-Datei von inkscape ziemlich überladen ist. 6,2 kiB. Das geht doch kleiner. Also [`svgo`](https://github.com/svg/svgo) zum Optimieren genutzt: 3,0 kiB.

Besser. Aber schauen wir doch mal in die Datei herein. Da sind Dinge drin wie

```
<path style="fill:#fff;fill-opacity:1;stroke:none;stroke-width:1;stroke-linecap:round;stroke-dasharray:none;stroke-opacity:1" d="M44.22 39.117s.073 1.226 0 0z"/>
```

Wow. Das ist ne Menge CSS in dem `style`-Attribut. Das meiste davon unnütz. Hier hätte anstelle des `style`-Attributs einfac `fill="white"` nehmen können. Also habe ich das gemacht. Dann noch einmal `svgo` drüberlaufen lassen und… voilà: nur noch etwa 1,9 kiB.

Notiz am Rande: die Option, dass ein SVG auch `<script>`-Tags oder CSS enthalten kann ist ein [Grund, warum viele Seiten keine SVGs aus user-Uploads oder anderen unvertrauenswürdigen Quellen einbinden](https://research.securitum.com/do-you-allow-to-load-svg-files-you-have-xss/). Schon mal versucht, in Discord oder Slack eine SVG-Datei zu posten? Geht nicht.

Und noch ein Vergleich: Die Originalgrafik, als verlustfreies WebP abgespeichert, ist 244 *Bytes* groß. Die SVG liegt also um eine Größenordnung darüber. Dafür ist sie aber skalierbar.

### Genauigkeit

Natürlich möchte ich jetzt noch herausfinden, wie originalgetreu meine SVG-Datei ist. Die Idee: Ich könnte ja einfach die SVG-Datei in entsprechender Größe exportieren und mit der Originaldatei vergleichen.

Das Problem: Die Originaldatei ist strikt schwarz-weiß, keine Graustufen. Mit Standardeinstellungen wird die aus dem SVG exportierte Datei hingegen Graustufen enthalten, zur Kantenglättung. Das macht einen Vergleich schwieriger.

Also mal stöbern, wie man eine Rastergrafik mit harten Kanten exportiert… vielleicht die Antwort auf [diese neun Jahre alte stackexchange-Frage](https://graphicdesign.stackexchange.com/questions/32058/how-to-rasterize-an-svg-without-anti-aliasing)?

> The development version of Inkscape (upcoming 0.91 release) has a global anti-aliasing toggle in the Document Properties window, which should also work for export.

Nun, neun Jahre später muss ich mich nicht mehr auf die Development-Version verlassen. Die Checkbox ist einfach zu finden. Das Ergebnis ist absolut identisch. Kurze Suche zeigt, warum: die abgewählte Checkbox wird jedes Mal zurückgesetzt, wenn ich die Dokumenteneinstellungen verlasse.

Aber halt, man kann inkscape ja auch als Kommandozeilentool verwenden! Das sieht dann etwa so aus:

```
inkscape -C -w 90 -h 90 napstablook_black_bg.svg -o napstablook_no_anti_aliasing.png
```

Aber `man inkscape` zeigt keine Parameter, mit denen man anti-aliasing steuern kann. Also auch nicht der richtige Weg. Hmpf.

Ok, dann begnüge ich mich mit einer Krücke: Ich nehme die geglättete Version und lasse einfach einen Schwellwertfilter drüberlaufen. In diesem Fall habe ich [gimp](https://www.gimp.org/) dafür genommen. Ich muss den Threshold-Filter dort jedes Mal suchen. Der ist nämlich nicht im `Filters`-Menü sondern im `Colors`-Menü. Als Schwellwert abe ich 127 genommen, also genau die Mitte.

Jetzt muss ich die Bilder nur noch vergleichen. Das geht mit [`compare` von Imagemagick](https://imagemagick.org/script/compare.php):

```
compare napstablook_original.webp napstablook_not_anti_aliased.png diff.png
```

Das Ergebnis mit den Defaulteinstellungen ist dieses Bild (rote Farbe bedeutet Unterschiede):

![Unterschiede sind an vielen Kanten zu erkennen, aber nicht an allen. In der Regel sind sie nicht breiter als ein Pixel](/file/napstablook_diff.webp "spooky")

Meine Einschätzung: Es gibt schon Unterschiede, aber insgesamt ist es nah genug dran.

### Fazit

Ich hatte die SVG-Datei schon seit Monaten herumliegen, aber jetzt bin ich endlich dazu gekommen, sie zu optimieren. Ihr könnt sie gerne herunterladen und verwenden, aber beachtet, dass der eigentliche Urheber von Napstablook Toby Fox ist. Der sieht Fanart sehr locker, aber im großen Stil verkaufen bzw. Geld damit machen sollte man nicht.

Die SVG-Datei ist übrigens die, die ich auf [dem Chaos Communication Camp für meinen Napstablook-Anstecker verwendet habe](/blogposts/camp_2023).
