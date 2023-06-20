---
title: Das NRW-Abidebakel
filename: nrw_abidebakel
date: 2023-06-20T19:57:10+02:00
update-date:
tags: nrw, abitur, download, rant, sicherheitslücken, zerforschung
category: hoellenmaschinen
summary: Ich habe immer noch keine Erklärung für das Abiturklausurdownloaddebakel in NRW 2023
image:
---

Das ist jetzt natürlich ein paar Monate zu spät, aber ich wollte im Rahmen [der](https://blog.strangerthanusual.de/blogposts/logging) [Dinge](https://blog.strangerthanusual.de/blogposts/ms_preiserhoehung), über die ich nach meinem Urlaub schreiben wollte, auch über das [Abiturdownloaddebakel](https://www.heise.de/news/Technikpanne-bei-Abitur-in-NRW-Ueberlastete-Server-sollen-Ursache-gewesen-sein-8976516.html) in NRW schreiben. Dann mache ich das halt jetzt.

Über die [Sicherheitslücken](https://www.heise.de/news/Technikpanne-bei-Abitur-in-NRW-Ueberlastete-Server-sollen-Ursache-gewesen-sein-8976516.html), die im Anschluss gefunden wurden, lasse ich mich mal nicht aus. Das kann [Lilith Wittmann](https://twitter.com/LilithWittmann/status/1650586828099993602) viel besser.

Aber verdammt noch mal, ich will eine Erklärung, warum der Server beim Download der Abiturprüfungen zusammengebrochen ist. Mein erster Gedanke war, kurz zu überschlagen, wie viele Schulen da potentiell gleichzeitig etwas herunterladen wollten.

Anhand der Anzahl betroffener Schüler (die im Online-Artikel genannt wurde) bin ich auf etwa 1000 Schulen gekommen (damit lag ich erstaunlich nah an den später angegebenen ca. 900 Schulen). Also 1000 gleichzeitige Downloads, vielleicht 3000, wenn unterschiedliche Fächer separate Downloads haben.

### Warum ist das so unglaublich peinlich?

Nun ist es natürlich nichts Neues, wenn Server mal unter plötzlichem Ansturm zusammenbrechen. Wenn zehntausende bis Millionen gleichzeitig zugreifen und man nicht mit so einem Ansturm gerechnet hat.

In diesem Fall kann der Ansturm aber keine Entschuldigung sein. Denn erstens wussten die Verantwortlichen ziemlich genau, wie viele Anfragen kommen: schließlich kennen sie die Anzahl der Schulen, die die Aufgaben brauchen.

Und zweitens: 3000 gleichzeitige Anfragen sind _nichts_. Insbesondere, wenn der Server nur Daten rausschaufeln muss. Das belastet die CPU nicht einmal sonderlich, die ist den größten Teil der Zeit damit beschäftigt, zu warten, bis die Daten auf die Leitung geschoben sind. Die Daten sind statisch, die müssen nicht aufwändig aus Templates gerendert werden, brauchen keine teuren Datenbankoperationen. Einfach eine (einmalige) Authentifizierung des Clients, dann können die Daten rübergeschoben werden.

Und jetzt kommt mir nicht mit „aber das sind ja ein paar Gigabyte pro Aufgabe, da wird der Server halt überlastet”. Nein! Das bedeutet vielleicht, dass jeder einzelne Download langsam ist, aber der Server bricht davon nicht zusammen.

Der Notfallplan, die Daten woanders hochzuladen und einen Downloadlink herumzuschicken, ist dann anscheinend daran gescheitert, dass der Link herumgeschickt wurde, bevor der Upload abgeschlossen war und außerdem der Upload fehlschlug.

Auch hier: Warum? Und man hätte die Aufgaben zur Not auch in einem fucking Google Drive hochladen können. Verschlüsselt, den Schlüssel hätte man dann ja über einen anderen Kanal geschickt. Selbst Google kann dann nicht den Inhalt sehen. Und über Datenschutz hätte man sich hier auch keine Gedanken machen müssen: Schließlich laden aus Sicht von Google _Schulen_, nicht Personen die Daten runter. Und dass eine Schule die Daten herunterlädt, ist jetzt nicht überraschend.

### Aufklärung

Ich habe mich wieder an das Thema erinnert, weil [Zerforschung gestern einen Artikel](https://zerforschung.org/posts/freundschaftspass-de/) über die kaputte Plattform für die Verteilung des Freundschaftspass-Deutschland-Frankreich online gestellt haben (lesenswerter Artikel, aber ich schweife ab).

Nun sind seitdem Abiturdebakel zwei Monate vergangen und ich frage mich: Was ist mittlerweile dabei herausgekommen?

Nun, viel habe ich nicht gefunden. Der erste [Artikel](https://www.tagesschau.de/inland/regional/nordrheinwestfalen/wdr-abi-pannen-lehrer-finden-fehler-in-bio-und-informatik-klausuren-100.html), der die Ursachen erklären will, ist vom 21.4.2023, also kurz nach dem Debakel, und sagt nicht mehr, als ich oben schon beschrieben habe. Das ist keine Erklärung, was da schiefgelaufen ist. Das wirft nur noch mehr Fragen auf (Fragen à la „WTF WARUM?“).

Das nächste, was ich finden konnte, war [dieser Artikel](https://www.tagesschau.de/inland/regional/nordrheinwestfalen/wdr-schulministerin-feller-zieht-konsequenzen-aus-abi-panne-100.html), bei dem es aber nur darum geht, welche Konsequenzen gezogen werden. Aber immerhin:

> Der Vertrag mit dem externen Dienstleister […] laufe ohnehin im Sommer aus, man werde darum Aufgaben neu ausschreiben. Und zwar mit höheren Anforderungen an das Daten-Volumen […], die Testung neuer Verfahren und die Redundanz, also, dass es quasi einen Plan B für Störungen gibt.

Nur wirklich schade, dass die Ursache nie wirklich öffentlich geklärt wurde. Ich hätte da gerne mehr Details erfahren. Oder vielleicht gabe es die Details, aber ich habe sie im Internet einfach nicht gefunden.
