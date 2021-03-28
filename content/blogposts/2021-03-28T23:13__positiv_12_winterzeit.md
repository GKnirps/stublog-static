---
title: Positivprojekt 12: Zeitumstellung
filename: positiv_12_zeitumstellung
date: 2021-03-28T23:13:35+02:00
update-date:
tags: zeitumstellung, sommerzeit, winterzeit, eu, xkcd
category: positiv
summary: Es ist gut, dass die Zeitumstellung abgeschafft werden soll. Es ist auch gut, dass das nicht übereilt passiert.
---

Ich bin ja froh, dass in der 2019 in der EU entschieden wurde, dass die EU-weite verpflichtende Zeitumstellung abgeschafft werden soll. Die Zeitumstellung bringt [meiner Ansicht](https://xkcd.com/1883/) nach mehr Ärger als Nutzen, alleine schon weil so viele Softwareentwickler es nicht hinkriegen, ihre Timestamps ordentlich zu verarbeiten. [Zeitzonen sind schwierig](https://www.zainrizvi.io/blog/falsehoods-programmers-believe-about-time-zones/), aber das ist keine Entschuldigung, nicht zumindest einen offset an den Timestamp zu schreiben.

Ich bin aber ebenso froh, dass die Sommerzeit nicht sofort abgeschafft wurde. Ich hatte ja ein bisschen Sorge, dass das zu schnell passieren würde. Das wäre zum einen doof, weil es unweigerlich zu einem europäischen Flickenteppich an Sommerzeit/nicht Sommerzeit geführt hätte (was die ganze Lager verschlimmert hätte).

Zum anderen wäre eine zu schnelle Umstellung problematisch, weil ein riesiger Haufen Software von Zeitzonenberechnungen abhängt. Wenn man jetzt die Zeitzohnen ändert, muss man erst einmal die Bibliotheken, die damit umgehen, anpassen. Dann muss man in allem, was direkt davon abhängt die Abhängigkeit aktualisieren. Dann muss man das Ganze noch für alles machen, was _davon_ abhängt, und so weiter.

Bei vielen Programmen und Bibliotheken sollte das ganz schnell gehen. Aber selbst dann muss die aktualisierte Software auch überall ausgespielt werden, und wir wissen alle, wie schwer sich gerade Unternehmen und Ämter damit tun, Updates zu installieren. Aber selbst wen da alles schnell läuft, gibt es immer noch einen Haufen Software, der schlecht gepflegt wird oder bei dem es aufgrund von Versionskonflikten nicht einfach sein wird, die aktualisierten Bibliotheken einzubinden.

Von daher sollten wir uns alle freuen, dass die Mühlen der Legislative so langsam laufen. Mein Vorschlag ist: Mitteleuropäische Sommerzeit in allen beteiligten Ländern abschaffen, zum gleichen Zeitpunkt, und mit einer Vorlaufzeit von mindestens fünf Jahren. Wer dann seine Software immer noch nicht aktualisiert hat, darf berechtigt verspottet werden.
