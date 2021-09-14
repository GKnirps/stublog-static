---
title: Unicode code point vs. scalar value
filename: unicode_scalar_value
date: 2021-09-14T20:40:33+02:00
update-date:
tags: unicode, code point, scalar value, definition, UTF-16, surrogate
category: hoellenmaschinen
summary: Was zur Hölle ist eigentlich der Unterschied zwischen einem „unicode code point“ und einem „unicode scalar value“? Die meisten Antworten darauf habe ich nur mit Mühe verstanden. Mal sehen, ob ich es selber besser erklären kann.
image:
---

Mit kommt in letzter Zeit immer wieder der Begriff „unicode scalar value“ unter. Zum Beispiel in der [rust-Dokumentation zum Typ `char`](https://doc.rust-lang.org/std/primitive.char.html):

> The `char` type represents a single character. More specifically, since ‘character’ isn’t a well-defined concept in Unicode, char is a [Unicode scalar value](https://www.unicode.org/glossary/#unicode_scalar_value)’, which is similar to, but not the same as, a ‘[Unicode code point](https://www.unicode.org/glossary/#code_point)’.

Hmm… aber was ist denn da nun der Unterschied? Zum code point steht, wenn man dem Link folgt:

> Any value in the Unicode codespace; that is, the range of integers from 0 to 10FFFF. […]

Ok, das deckt sich soweit mit meinen Erwartungen. Und zu scalar value?

> Any Unicode code point except high-surrogate and low-surrogate code points. In other words, the ranges of integers 0 to D7FF and E000 to 10FFFF inclusive.

Das ist schon weniger klar. Was sind denn diese high-surrogate und low-surrogate code points? Fehlt einem irgendwas, wenn man einen `char`-Typ hat, der die nicht enthält? Kann rust vielleicht mit seinem `char` nicht alles darstellen, was unicode zu bieten hat? (Strings sollten kein Problem sein, oder? schließlich sind Strings in rust immer UTF-8-codiert, und UTF-8 kann ja alle code points darstellen, richtig?)

### Why bother?

Warum ist mir das jetzt wichtig? Weil ich in Kürze einen kleinen Vortrag u.a. über gotchas im Zusammenhang mit unicode halten will. Aber wie soll mir das gelingen, wenn ich nicht einmal den Unterschied zwischen einem code point und einem scalar value kenne?

### Surrogates

Ok, also was hat es mit diesen oben erwähnten surrogates auf sich? Im Unicode-Glossar steht direkt unter „Code Point“ „[Code Point Type](https://www.unicode.org/glossary/#code_point_type)“:

> Any of the seven fundamental classes of code points in the standard: Graphic, Format, Control, Private-Use, Surrogate, Noncharacter, Reserved. (See definition D10a in [Section 3.4, Characters and Encoding](http://www.unicode.org/versions/latest/ch03.pdf#G2212).) 

Aha! Also sind Surrogate Code Points eine spezielle Unterart der Code Points? Folgen wir mal dem Link. *clicK* … *scroll scroll scroll* … da ist es:

> High-surrogate code point: A Unicode code point in the range U+D800 to U+DBFF.

> High-surrogate code unit: A 16-bit code unit in the range D800 to DBFF, used in
UTF-16 as the leading code unit of a surrogate pair.

> Low-surrogate code point: A Unicode code point in the range U+DC00 to U+DFFF.

> Low-surrogate code unit: A 16-bit code unit in the range DC00 to DFFF, used in
UTF-16 as the trailing code unit of a surrogate pair.
> - High-surrogate and low-surrogate code points are designated only for that use.
> - High-surrogate and low-surrogate code units are used only in the context of the
UTF-16 character encoding form.

Nun weiß ja jedes Kind, dass UTF-16 aus 16-Bit-Einheiten („code units“) besteht. Was viele allerdings gerne ignorieren (hier kommt wieder mein unicode-gotcha-Vortrag ins Spiel) ist, dass ein code point in UTF-16 aus einem _oder zwei_ dieser code units bestehen kann. Man muss also irgendwie erkennen, dass es nach der ersten code unit noch weitergeht.

Nun hätte ich ja erwartet, dass sie das hier so ähnlich wie [bei UTF-8](https://de.wikipedia.org/wiki/UTF-8#Algorithmus) machen (das erkläre ich jetzt nicht, aber auf Wikipedia ist es recht gut dargestellt). Machen sie aber nicht. Vermutlich aus historischen Gründen.

Bei code points, die durch zwei 16-bit code units dargestellt werden, beginnt der höherwertige (also „High-Surrogate“) immer mit dem Binärstring `110110` und der niederwertige mit dem Binärstring `110111`. Code points, die durch nur eine 16-bit code unit dargestellt werden, beginnen *nie* mit einem dieser Bitsrings. Man kann also (ebenso wie bei UTF-8 immer unterscheiden ob man eine Codeeinheit hat, die für sich alleine steht, oder ob sie Teil eines code points ist, der in mehreren Einheiten encodiert wird. Man kann sogar erkennen, ob man sich am Anfang eines code points befindet oder nicht. Praktisch.

### Aber warum die code points?

So, das ist geklärt. Aber warum gibt es _eigene code points_ für diese Surrogates? Die Surrogates sind doch etwas encoding-spezifisches! Bei UTF-8 braucht man so etwas doch auch nicht. Scrollt man im Unicode-Standard noch ein bisschen weiter nach unten, steht da auch, dass die drei unicode encodings UTF-8, UTF-16 und UTF-32 alle nur die unicode scalar values codieren, nie die surrogate code points.

Also warum der Kram? Welchen Grund hat es, den Standard so zu bauen? man könnte das UTF-16 encoding doch genau so gut ohne diese Surrogate-Code-Points bauen, man kann die surrogate-code-points ja sogar theoretisch in UTF-16 codieren (wobei das, wie gesagt, nicht gültig wäre, weil ja nur scalar values encodiert werden).

Einige Kommentare an verschiedenen Stellen haben mich auf [UCS-2](https://de.wikipedia.org/wiki/Universal_Coded_Character_Set). Davon hatte ich vorher schon einmal gehört, das war so etwas wieder unicode-Vorgänger, als man glaubte, 2¹⁶ code points würden locker ausreichen. Im Gegensatz zu UTF-16 hat in UCS-2 jeder code point exact 16 bit Länge.

In dem Wikipedia-Artikel steht:

> Der UCS wird entwickelt von ISO/IEC/JTC1/SC2/WG2. Die Gruppe arbeitet sehr eng mit dem Unicode-Konsortium zusammen, das die Standards ständig in neuen Versionen synchronisiert. Aufgrund dessen sind alle Kodierungen aus Gründen der Interoperabilität beschränkt auf die bei Unicode erlaubten 1.112.064 Zeichen (= 220+216, abzüglich 211 = 2048 Surrogate von UTF-16), nämlich von U+00000 bis U+0D7FF sowie von U+0E000 bis U+10FFFF.

Es ist also irgendein Rückwärtskompatibilitätskrams, damit man UTF-16 nicht mit UCS-2 verwechseln kann.

### TL;DR

- für beliebige Unicode-Texte braucht man nicht alle code points, sondern nur die Unicode scalar values
- die übrigen Werte sehen so aus wie die Werte, die die code units in UTF-16 surrogate pairs annehmen können
- das ist vermutlich aus Rückwärtskompatibilitätsgründen drin.

Habe ich schon einmal erwähnt, dass ich UTF-8 für das [bessere Encoding](https://utf8everywhere.org/) halte? Und siehe da, das hat eine Rückwärtskompatibilität mit ASCII und man muss überhaupt keine code points für das Encoding reservieren und braucht auch nicht auf byte order zu achten.
