---
title: Neue PNG-Spezifikation
filename: neue_png_spezifikation
date: 2025-06-28T20:27:04+02:00
update-date:
tags: png, apng, kompression, webp, optimization, zopfli, internet explorer
category: hoellenmaschinen 
summary: Nach etwa 22 Jahren hat die PNG-Spezifikation ein Update bekommen.
image:
image-alt:
language:
---

Vor ein paar Tagen kam [die Meldung, dass es eine aktualisiert PNG-Spezifikation gibt](https://www.programmax.net/articles/png-is-back/). Darin enthalten: verschiedene Standardisierungen, oft von ohnehin schon so benutzten Features, wie z.B. [Exif-Metadaten](https://de.wikipedia.org/wiki/Exchangeable_Image_File_Format) oder dem APNG-Format.

Letzteres habe ich hier auch schon verwendet, in dem [Blogpost über eine Barber's Pole als Farbschichtschema für Fordite](/blogposts/fordite_barber_pole). Ich wollte eine Animation haben, aber eine GIF-Datei wollte ich nicht benutzen, also habe ich mir angeschaut, was es sonst so gibt. Es gibt wohl das MNG-Format, das von niemandem unterstützt wird und die Konkurrenz in form von APNG, die bis vor ein paar Tagen nicht standardisiert war.

Ich wollte dann direkt mal nachschauen, ob meine APNG-Dateien eigentlich standardkonform sind. Bei der Suche nach einem Tool bin ich dann aber abgelenkt worden und auf `apngopt` gestoßen. Wer dieses Blog verfolgt, weiß, dass ich nahezu zwanghaft alle Bilder optimal komprimieren möchte. Momentan benutze ich für die meisten Bilder in diesem Blog das WebP-Format (je nach Anwendung in verlustfreier oder verlustbehafteter Version). Verlustfrei komprimiert WebPs sind in den meisten relevanten Fällen kleiner als vergleichbare PNGs. PNGs sind aber immer noch ein Ding, nicht zuletzt wegen der Animationen.

Also musste ich das tool natürlich ausprobieren. Und siehe da: es kriegt meine APNG auch ein bisschen kleiner. Die eine Datei schrumpfte von 13546 byte zu 12206 byte, also um mehr als ein kilobyte oder um fast zehn Prozent. Die andere Datei schrumpfte von 46743 auf 43840 byte also um knapp drei kilobyte oder gut 6%.

Was mich an diesen ganzen Bildoptimierungsprogrammen irritiert ist, wie unterschiedlich das Benutzerinterface ist. Beispiele

- `optipng`, `jpegoptim` und `svgo` ersetzen per default die Originaldatei mit der kleineren Version (nur, wenn die wirklich echt kleiner ist), lassen sich aber dazu überreden, eine andere Datei als Zieldatei zu verwenden.
- `zopflipng` braucht explizit eine Ausgabedatei. Ist das dieselbe wie die Originaldatei, wird nachgefragt. Die Nachfrage kann man mit einem Flag unterdrücken
- `apngopt` legt neben der Originaldatei eine zweite Datei an, die so heißt wie das Original, nur das (vor der Dateiendung) noch In `_opt` angefügt wurde. Es lässt sich nicht dazu bringen, es anders zu tun.

Immerhin kann ich  bei `apngopt` zwischen verschiedenen Kompressionsbackends wählen, unter Anderem auch zopfli.

Eine kleine Anekdote noch: PNG, geschaffen für das Internet, wurde in den Neunzigerjahren als Alternative für GIF entwickelt, weil GIF von Patenten belastet war (die 2006 ausgelaufen sind). Herausgekommen ist ein Format, das Bilder kleiner und in besserer Qualität als GIF ausliefert.

Nur Microsoft hatte es lange nicht hingekriegt, PNGs vollständig zu unterstützen. Im berüchtigten Internet Explorer 6 zum Beispiel wurden keine transparenten PNGs unterstützt. Anstatt das zu beheben [empfahl Microsoft, eine proprietäre Erweiterung im HTML-Code zu verwenden](https://web.archive.org/web/20090203210750/http://support.microsoft.com/kb/294714). Es gab auch Javascript von Drittparteien, die das Problem gelöst haben. Und wie das nun einmal so ist, hat man einen solchen Workaround einmal eingebaut, bleibt er _lange_. In einem Projekt, in dem ich gearbeitet habe, war bis 2018 noch Code, der Javascript enthielt, um dem zu diesem Zeitpunkt schon zum Teufel gejagten Internet Explorer 6 auf die Sprünge zu helfen.
