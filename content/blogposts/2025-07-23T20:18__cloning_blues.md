---
title: Cloning Blues
filename: cloning_blues
date: 2025-07-23T20:18:38+02:00
update-date:
tags: programmieren, rust, java, c, c++, golang
category: hoellenmaschinen
summary: Eigentlich sollte hier ein langer Blogpost zu tiefen Kopien in verschiedenen Programmiersprachen hin. Ich habe aber keine Lust, also halte ich es relativ kurz.
image:
image-alt:
language:
---

Als ich einem Kollegen meine Idee für den kürzlich veröffentlichtem [Blogpost über Vergleichsoperationen in verschiedenen Programmiersprachen](/blogposts/gleich_ungleich_gleich) erzählt habe, meinte er, ein Post über tiefe Kopien in verschiedenen Sprachen fände er viel interessanter. Ich fand die Idee auch interessant, und hatte mir ein paar Pläne gemacht.

Nun hat der Vergleichsoperations-Blogpost so viel Zeit gekostet, dass ich keine Motivation habe, das zu wiederholen. Also hier eine Kurzabhandlung.

Tiefe Kopien von Datenstrukturen sind Kopien, in denen wirklich von allem in der Datenstruktur eine Kopie gemacht wird, rekursiv die Hierarchie hinab. Im Gegensatz zu einer flachen Kopie, wo nur auf oberster Ebene die Referenzen bzw. Pointer kopiert werden. Im zweiten Fall sollte man dan tunlichst nichts an den Daten in der Kopie verändern, wenn man will, dass das Original unverändert bleibt. Also genau das, was man vermutlich machen möchte, wenn man eine Kopie erzeugt.

Wie gehen verschiedene Sprachen das an? Für C muss man zum Beispiel alles Kopieren von Daten hinter Pointern von Hand machen. Als C-Entwickler_in ist man so etwas aber gewöhnt. C++ hat Standardkopieoperationen für Klassen, die je nachdem, was wie sich die Datentypen in der Klasse kopieren, tiefe oder flache Kopien machen. Will man gesichert eine tiefe Kopie, muss man den Zuweisungsoperator überschreiben. Und den Kopierkonstruktor. Und vermutlich auch den Dekonstruktor. Und dann ist es recht einfach, aus Versehen eine teure tiefe Kopie zu machen, wenn man eigentlich nur ein Objekt weiterreichen möchte.

In rust ist die Sache natürlich wieder schön gelöst. Standardmäßig keine tiefen Kopien, oder überhaupt Kopien. Entweder man gibt das Eigentum an einer Variable weiter, oder man verleiht die Variable. Wenn man kopieren will, muss man das `Clone`-trait implementieren. Ähnlich wie beim Gleichheitsoperator kann man das aber auch automatisch machen lassen, wenn alle Unterelemente des Types `Clone` implementieren. `clone()` muss explizit aufgerufen werden, also keine teuren, tiefen Kopien aus Versehen. Bonuspunkte für `Copy`, das man implementieren kann, wenn die flache Kopie identisch mit der tiefen Kopie ist, so dass man leichter Sachen kopieren kann anstatt das Eigentum weiterzugeben.

Die meisten heutigen Sprachen arbeiten aber grundsätzlich erst einmal mit Referenzen. Spezielle Kopieroprationen muss man den ganzen Baum hinab manuell implementieren. Java insbesondere hat es fies gemacht: Jedes Objekt hat die `clone()`-Funktion, die keine Typsicherheit bietet und immer eine Exception wirft, wenn man nicht das `Cloneable`-Interface implementiert, das lustigerweise keine Funktionen beinhaltet, auch nicht `clone()`. Ich glaube, es ist in Java Konsens, `clone()` nicht zu benutzen.

Ist aber auch egal. Ich würde für solche Sprachen (mit garbage collector und mit der zwanghaften Neigung, ausschließlich Referenzen herumzureichen) folgendes Vorschlagen: Macht alle Klassen immutable (unveränderlich, read-only). Dann macht es keinen Unterschied mehr zwischen flacher und tiefer Kopie, weil niemand in den Daten herumpfuschen kann.

Genau genommen würde ich diesen Ansatz auch für andere Sprachen empfehlen. Es gibt da noch ein paar Optimierungsmöglichkeiten (z.B. [Copy-On-Write](https://de.wikipedia.org/wiki/Copy-On-Write) für manche Anwendungsfälle, [Perstistent data structures](https://en.wikipedia.org/wiki/Persistent_data_structure) für andere), aber insgesamt ist man mit unveränderbaren Objekten auf der sicheren Seite. Nur in rust gibt der Compiler dir Rückendeckung im Umgang mit veränderbaren Daten. Das ist aber mit einigen Einschränkungen verbunden, deswegen ist es am Ende doch wieder leichter, nur unveränderliche Daten herumzureichen.

Kurz: Macht eure Daten immutable, dann braucht ihr sie nicht zu klonen. Wenn eure Sprache das nicht zulässt (ich schaue dich an, Go), dann habt ihr Pech gehabt.
