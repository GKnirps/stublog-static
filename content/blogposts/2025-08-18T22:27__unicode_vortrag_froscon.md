---
title: Unicode-Vortrag auf der FrOSCon
filename: unicode_vortrag_froscon
date: 2025-08-18T22:27:12+02:00
update-date:
tags: unicode, froscon, vorträge, homoglyphen, homographen
category: hoellenmaschinen
summary: Jemand hat einen Unicode-Vortrag auf der FrOSCon gehalten.
image:
image-alt:
language:
---

Die letzten Tage über wurde ich via Mastodon auf einige Vorträge der diesjährigen [FrOSCon](https://froscon.org/) aufmerksam. Darunter auch der Vortrag „[Unicode für Anfänger](https://media.ccc.de/v/froscon2025-3392-unicode_fur_anfanger)“.

Ich betrachte mich nicht als Anfänger, was Unicode angeht, aber ich habe mir den Vortrag trotzdem angeschaut. Zum Einen, damit ich weiß, ob ich ihn weiterempfehlen kann (kann ich). Zum Anderen, weil ich immer gerne mal wieder daran erinnert werde, welche ungeahnten Probleme es mit Schriften, Sprachen und Texten gibt, an die man üblicherweise nicht denkt. Und zuletzt, weil ich ja vielleicht doch noch etwas Neues lernen kann.

Und das habe ich. So habe ich zum Beispiel gelernt, dass es getrennte Code Points für das groe Omega (U+03A9, GREEK CAPITAL LETTER OMEGA, Ω) und das Zeichen für die Maßeinheit für elektrischen Widerstand, Ohm (U+2126, OHM SIGN, Ω) gibt. Nebeneinander: „ΩΩ“. Interessant auch: Nach jeder im Standard definierten Normalisierung wird das Ohm-Zeichen zu einem großen Omega normalisiert. Hat mich ein bisschen an die [römischen Zahlen in Unicode](/blogposts/stenberg_idn) erinnert.

Überhaupt legte der Vortrag einen gewissen Fokus auf Normalisierung, Homoglyphen/Homographen und alphabetische Sortierung, und Andeutungen auf deren Sicherheitsrelevanz. Weit in die Tiefe ging der Vortrag allerdings nicht, sollte aber ausreichen, damit man sich bewusst wird, dass alles schwieriger ist, als man auf Anhieb denkt.

Der Vortragende ist selbst kein Unicode-Experte und musste sich dieses Wissen nur notgedrungen aneignen (eine verwandte Seele!). Ein paar kleine Korrekturen habe ich auch (der Autor meinte, er habe immer gesagt bekommen: „Wenn du dir bei einem Thema nicht 100% sicher bist, schreib's im Internet, irgendeiner wird dich korrigieren“).

Zum Einen: Obwohl er die Uneinheitlichkeit der lexikographischen Sortierung als schwierig darstellt, [behauptet er, dass es im Deutschen eine Sortierung gibt](https://media.ccc.de/v/froscon2025-3392-unicode_fur_anfanger#t=1574). Es gibt aber allein in der Deutschen Sprache mindestens zwei offizielle Standards, und einer davon, der DIN 5007-Standard kennt zwei Varianten. Es ist also, selbst wenn man die Sprache kennt, nicht eindeutig.

Zum Anderen sagt er, dass [Python UTF-32 verwendet, wenn mindestens ein nicht-ASCII-Zeichen im String vorkommt, sonst ASCII](https://media.ccc.de/v/froscon2025-3392-unicode_fur_anfanger#t=1366). Das ist nicht ganz korrekt. Zwischen diesen Extremen gibt es nämlich noch eine 16-Bit-Codierung, wenn alle vorkommenden Code points in 16 Bit codiert werden können. Ich hatte das vor Jahren mal für einen kleinen Vortrag recherchiert [und diesen Blogpost dazu](https://rushter.com/blog/python-strings-and-memory/) gefunden.

Immerhin [haut der Vortrag zu Recht auf UTF-16 drauf](https://media.ccc.de/v/froscon2025-3392-unicode_fur_anfanger#t=1379): „Es hätte eigentlich deprecated gehört“. Nur die Einschätzung, dass UTF-16 kaum noch verwendet wird, kann ich nicht teilen. Zugegeben, als Transportencoding sieht man es selten (mir ist _ein Mal_ eine UTF-16-codierte XML-Datei untergekommen, die hatte kein [BOM](https://de.wikipedia.org/wiki/Byte_Order_Mark) und unser Decoder hat sie direkt falsch geparsed), aber an vielen Stellen ist es als internes Encoding noch weite Verbreitet. Der Vortrag nennt nur Windows und Javascript, aber auch Java und C# haben UTF-16 als Stringencoding (fun fact: keines der vier Beispiele behandelt UTF-16 korrekt).

Genug des Erbsenzählens. Das ist ein guter Einsteigervortrag zu Unicode. Schaut ihn euch an.
