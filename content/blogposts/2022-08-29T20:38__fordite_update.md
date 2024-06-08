---
title: Fordite-Update
filename: fordite_update
date: 2022-08-29T20:38:21+02:00
update-date:
tags: fordite, kunst, digital art, digitale kunst
category: hoellenmaschinen
summary: Meine Fordite-Software hat ein paar Updates erhalten.
image: /file/fordite_blob.webp
image-alt: ungleichmäßig aussehende, zentrierte Farbblobs
---

Vor kurzem habe ich ja endlich den [Blogpost zu meinem fordite-Programm](/blogposts/fordite) fertig gestellt. Das heißt, ich hatte danach endlich Zeit, mal wieder an der Software selber weiter zu arbeiten.

Das Schöne am momentanen Stand der Software ist, dass es recht einfach ist, neue Höhenkarten und neue Farbschichtschemata hinzuzufügen. Die eigentliche Arbeit ist hier eher, die Beispielkonfigurationen zu erstellen (bzw. die bestehenden Dateien anzupassen, wenn ich inkompatible Änderungen zu bestehenden Optionen gemacht habe).

### Die Kugel

Die erste Änderung ist, dass ich die Halbkugel zur Kugel umfunktioniert habe. Anstatt der (im Verhältnis zur Bildgröße) festen Position in der x-y-Ebene, kann man jetzt das Zentrum der Kugel beliebig einstellen.

Die Ergebnisse hier waren aber eher unspektakulär, deswegen gibt es dazu kein Bild.

### Neue Höhenkarte: zwei verschiedene Sinuskurven

Die erste neue Höhenkarte, die ich hinzugefügt habe, ist ähnlich der bisherigen Sinuskurve, allerdings zwei davon, in unterschiedlicher Ausrichtung, ggf. mit unterschiedlicher Wellenlänge, aufeinander addiert. Mit einer Halbkugel sieht das dann zum Beispiel so aus:

![ungleichmäßig aussehende, zentrierte Farbblobs](/file/fordite_blob.webp)

Das hat mir schon einmal gefallen. Sah deutlich mehr nach echtem Fordit aus als die anderen Sachen. Also habe ich auch versucht, eine Animation daraus zu machen.

Da ist mir aufgefallen, dass ich in der Ursprungsversion die Phase der Sinuskurven nicht unabhängig voneinander ändern konnte.

Beim einfachen Sinus habe ich dafür einfach die gesamte Höhenkarte verschoben. Hier musste ich hingegen noch ein paar Parameter hinzufügen. Sieht aber schön aus, ein bisschen, wie eine Wasserspiegelung auf leicht welligem Wasser. [Das Video ist wieder auf youtube](https://www.youtube.com/watch?v=OXFSry2vJfE).

### Kreisförmige Sinuswellen

Die bisherigen Sinuskurven liefen alle parallel. Als Alternative dazu habe ich die mal kreisförmig angeordnet, also von einem Mittelpunkt ausgehend in alle Richtungen gleichmäßig. Ein bisschen so, als ob man einen kleinen Stein auf eine glatte Wasseroberfläche geworfen hätte.

Und siehe da: Damit sehen selbst die schiefen Farbebenen gut aus:

![grün-schwarzes Fordit-Muster aus Wellen, die zum Rand hin kreisförmiger werden](/file/fordite_green_spider.webp)

### Zylinderfarbschichten

Das Schöne an neuen Höhenkarten und Farbschichtschemata ist, dass man jede neue Höhenkarte mit allen bisherigen Farbschemata kombinieren kann und umgekehrt. Je mehr Farbschichtschemata bzw. Höhenkarten man also hat, desto mehr bringt das Hinzufügen noch einer Höhenkarte bzw. eines Farbschichtschemas.

Nach den zwei neuen Höhenkarten musste ich also erst einmal ein neues Farbschichtschema hinzufügen. Ich hatte ja im letzten Blogpost schon die Idee von zylinderförmigen Schichten geäußert. Und was soll ich sagen? Damit sieht auch der hyperbolische Paraboloid als Höhenkarte wieder interessant aus:

![ein blaues Oval in der Mitte, rechts oberhalb davon verlaufen Farbstreifen fast gerade, während sie sich gegen die linke untere Ecke fast rechtwinkelig biegen](/file/fordite_blue_corner.webp)

Natürlich musste ich den Zylinder auch mal mit den kreisförmigen Sinuskurven kombinieren:

![gelb-orange-rote Wellellinien, die eine U-Form um mehrere Kreise in der Mitte bilden](/file/fordite_fire_god.webp "Könnte die Darstellung eines Fantasy-Feuergottes sein").

Ursprünglich verlief der Zylinder hier von der `(0,0,0)` diagonal in die gegenüberliegende Ecke, aber mit der Farbgebung hier gefiel es mir besser, ihn von der Mitte der X-Achse diagonal nach oben verlaufen zu lassen. Das erweckt irgendwie einen Eindruck von Feuer.

### Eine neue Test-Höhenkarte

Für die Farbschichten hatte ich ja schon eine Option, um die Höhenkarten testen zu können. Das Farbschichtschema `display_height_map` hat anstelle von Farben einfach die Höhenkarte in Graustufen ausgegeben. Sehr hilfreich.

Umgekehrt habe ich jetzt eine waagerechte, flache Höhenkarte eingefügt, damit man Farbschichten testen kann. Man kann natürlich immer nur einen Ausschnitt sehen, aber es hilft beim Debuggen.

### Fazit

Die neuen Höhenkarten und die Zylinderfarbschichten haben sie gelohnt, es sind interessante Bilder dabei herausgekommen.

Mir fällt es zunehmend schwerer, originelle, aber auch schöne Farbpaletten zu erfinden. Schwerer zumindest, als Höhenkarten mit Farbschichten zu kombinieren.

Die hier dargestellten Bilder stellen natürlich nur einen kleinen Teil des Parameterraumes dar. Es gibt sicher noch eine Menge cooler Bilder, die sich hier verstecken. Man muss sie nur finden. Ihr seid alle herzlich eingeladen.

Als nächstes werde ich mich mal daran machen, Streckungen und Zerrungen für die Höhenkarten einzubauen. So kann man sicher noch ein paar interessante Effekte aus den bestehenden Höhenkarten herausholen (naja, vermutlich nicht aus allen).
