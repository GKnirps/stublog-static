---
title: PDF-Unterschriften
filename: pdf_unterschriften
date: 2024-04-18T10:37:07+02:00
update-date:
tags: rant, digitalisierung, pdf, adobe, bürokratie, verwaltung
category: rant
summary: Von Problemen, Unterschriften zu PDFs hinzuzufügen.
image:
language:
---

> Die Verwaltung sollte wegkommen von diesem PDF-Fetisch.
— [Casey Kreer](https://netzpolitik.org/2023/barrierefreiheit-verwaltungsdigitalisierung-mit-huerden/)

Für einen aktuellen Antrag an der Uni muss ich eine PDF-Datei „digital unterschreiben“. Und mit „digital unterschreiben“ meinen sie keine [digitale Signatur](https://de.wikipedia.org/wiki/Digitale_Signatur), also eine kryptographisches Verfahren, um sicherzustellen, dass tatsächlich _ich_ es war, der das Dokument bearbeitet/erstellt hat. PDF unterstützt das zwar, aber kaum jemand nutzt es, und es gibt (gab?) damit [auch gewisse Probleme](https://media.ccc.de/v/36c3-10832-how_to_break_pdfs).

Nein, gemeint ist, dass ich ein Bild meiner Unterschrift in eine PDF-Datei einfüge. Das erste Problem ist: PDF ist nicht dafür gemacht, bearbeitet zu werden. Es ist dazu da, ein Dokument, das _gedruckt_ werden soll genau so darzustellen, wie es gedruckt aussehen wird (inkl. aller Resourcen wie z.B. Schriftarten), so dass man das Dokument zwischen verschiedenen Systemen hin- und herschicken kann, und es überall gleich aussieht. Diese Aufgabe löst PDF ziemlich gut.

Nun hat aber der Erfolg von PDF Adobe dazu gebracht, noch einen Haufen weiterer Features hinzuzufügen. Zum Beispiel Formularfelder, oder allgemein: Annotationen, mit denen man Inhalt über PDF-Dokumente drüberlegen kann. Unterschriften zum Beispiel, aber auch Textmarkierungen, etc.

Damit kommen wir zum zweiten Problem: PDF ist kein freier Standard, und ein sehr komplexes Format noch dazu. So finden sich zwar ein Haufen PDF-Reader für verschiedene Systeme, die die meisten PDF-Dateien korrekt anzeigen können. Es finden sich auch viele Programme, die ihr Format in PDF exportieren können. Auch das funktioniert gut, zumal ja nur eine Untermenge der PDF-Features verwendet wird.

Problematisch wird es, wenn man Inhalte hinzufügen will. [`evince`](https://help.gnome.org/users/evince/stable/) zum Beispiel, ein PDF-Reader, der in Gnome standardmäßig verwendet wird, kann Formulare ausfüllen. Mehr oder weniger. In diesem Fall konnte ich die Formulare ausfüllen, aber der Inhalt der Felder wurde mir nicht angezeigt, es sei denn, ich hatte das entsprechende Feld markiert. Beim Einfügen von Bildern (der Unterschrift zum Beispiel) konnte mir `evince` aber nicht weiterhelfen.

Normalerweise nehme ich an dieser Stelle Xournal, ein Annotationsprogramm für PDFs. Das ging hier leider nicht, denn weder wurde das ausgefüllte Formular in Xournal angezeigt, noch wird nach dem Abspeichern der Inhalt des Forumlars behalten. Schlimmer noch, das Formular selbst wird unbrauchbar gemacht, so dass ich es nicht einmal neu ausfüllen kann.

Ich musste mir am Ende von meiner Professorin helfen lassen, die ein Programm hat, mit dem das geht. Das ist schon ein bisschen peinlich, weil ich ein erfahrener Informatiker bin, aber ich war am Ende echt am Rand eines Nervenzusammenbruchs.

Können wir bitte aufhören, für diesen Bürokratiekrams ein Format zu verwenden, das einfach nicht dafür geeignet ist? Ein Format, dass Benutzer praktisch dazu zwingt, die Software eines bestimmten Herstellers (Adobe) zu verwenden? Ein Format, das _nicht barrierefrei_ ist (dazu gerne den Artikel auf netzpolitik.org lesen, den ich oben verlinkt habe)?

Ich habe die Schnauze voll von dem Scheiß. Die Digitalisierung in Deutschland (oh, in den Niederlanden hatte ich denselben Ärger) geht in die falsche Richtung. Digitalisierung bedeutet nicht, die analogen Vorgänge 1:1 in einen Computer zu übertragen. Automatische Datenverarbeitung kriegen wir so auch nicht hin, PDF ist kein maschinenlesbares Format. Das heißt, am Ende muss doch wieder jemand manuell die Daten aus dem PDF in ein anderes System eintragen. Dann kann man auch lieber beim Papier bleiben, da ist es einfacher, Unterschriften drunterzusetzen.
