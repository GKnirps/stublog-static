---
title: dotnet fucking format
filename: dotnet_fucking_format
date: 2025-08-25T17:28:44+02:00
update-date:
tags: rant, c-sharp, dotnet, dotnet format, code formatter, microsoft
category: rant
summary: dotnet format ist der schlechteste Code-Formatter, den ich je benutzt habe.
image:
image-alt:
language:
---

Code-Style-Regeln sind an sich ja eine nette Sache. Wenn man gute Regeln hat und die einhält, ist der Code überall einheitlich, einfach zu lesen und an findet sich schnell zurecht.

Das Einhalten von Regeln ist natürlich immer so eine Sache, und wer möchte schon mühsam Zeilen umbrechen und einrücken und nachschauen, dass alle Leerzeichen richtig gesetzt sind, wenn man eigentlich an der Software arbeiten möchte. Deswegen gibt es Code-Formatter. Das sind Programme, die diese langweilige Arbeit übernehmen und den Programmierern die Zeit geben, sich auf das eigentliche Problem zu konzentrieren.

Der klassische Ansatz von Code-Formattern ist, dass man denen einen Haufen von Regeln vorgibt, und die danach den Code formatieren. Das stammt noch aus der Zeit bevor es Code-Formatter gab und sich ein ganzer Haufen unterschiedlicher Code-Styles für zum Beispiel C entwickelt hat, jeder brauchte also etwas Anderes.

Moderne Sprachen ohne solchen Ballast haben mehr oder weniger von Anfang an ihren eigenen Code-Formatter, der meist ein ordentliches Regelwerk hat und wenig bis überhaupt nicht konfigurierbar ist. `gofmt` für Go war der erste dieser Art, der mir untergekommen ist, dicht gefolgt von `rustfmt` für rust.

Keine dieser Sprachen ist eine Sprache, mit der ich beruflich gerade arbeiten kann. Stattdessen muss ich C# benutzen. Und C# hat `dotnet format`. Es gibt noch andere Formatter, aber das ist der offizielle, der Standard. Und er ist Kacke.

Zum Einen ist er unglaublich langsam. Das ist mir schon bei den Vorbereitungen zum [Vergleichsoperator-Blogpost](/blogposts/gleich_ungleich_gleich) aufgefallen. Zum Anderen ist er nicht idempotent: Unter bestimmten Bedingungen muss man ihn mehr als ein Mal laufen lassen, damit er alles macht, was er machen will. Oh, und er fügt in solchen Situationen gerne ohne Warnung schlecht geschriebenen Code hinzu, der wild mit Exceptions um sich wirft.

Das alleine ist schon doof genug. Aber `dotnet format` _kann keine Zeilen umbrechen_. Wenn man also einen Funktionsaufruf oder eine Funktionsdefinition mir vielen oder langen Parametern hat, wird `dotnet format` die nicht in je eine eigene Zeile verschieben. Ketten von Funktionsaufrufen, wie sie bei Iteratoren vorkommen, werden auch nicht umgebrochen. Andere Formatter haben Angst davor, Zeilenumbrücke wegzunehmen. `dotnet format` fügt aber auch keine hinzu, auf wenn sie _wirklich, wirklich einfach_ und _wirklich, wirklich nützlich_ wären.

Dazu kommt dann, dass, wenn man in solchen Situationen dann manuell umbricht, sich `dotnet format` auch weigert, diese umgebrochenen Zeilen korrekt einzurücken. Man muss also für zwei der Hauptfälle, für die man einen Code-Formatter hat, manuell formatieren. Wer macht so einen Mist?

Ach ja. Microsoft natürlich.
