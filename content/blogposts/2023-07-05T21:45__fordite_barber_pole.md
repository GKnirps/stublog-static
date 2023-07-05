---
title: Fordite-Update: Barber's Pole
filename: fordite_barber_pole
date: 2023-07-05T21:45:17+02:00
update-date:
tags: fordite, barber's pole, kunst, digitale kunst, rust
category: hoellenmaschinen
summary: Ich habe ein neues Farbschichtenmodell zu meinem Fordite-Programm hinzugefügt.
image: /file/fordite_barber_pole.webp
---

Es ist mal wieder soweit, ich habe mich mal wieder ein bisschen an mein [Fordite-Projekt](/blogposts/fordite) gesetzt. Ich hatte neulich die Idee, ein Farbschema basierend auf einem [Barber's Pole](https://de.wikipedia.org/wiki/Barber-Pole) anzuordnen. Als kleine Vorschau, hier ein Beispielbild für das, was am Ende herausgekommen ist:

![Fordite-Bild: Orange Verwirbelungen](/file/fordite_barber_pole.webp)

### Kurze Erklärung

Wie habe ich das gemacht? Vielleicht kann ich am besten im Querschnitt erklären. Die folgende Animation zeigt einen Querschnitt der Stange von oben betrachtet. Über die Animation hinweg bewegt sich die Kamera nach oben:

![Animation: zwei weiße, ein rotes und ein blaues Dreieck rotieren um den Mittelpunkt im Bild](/file/fordite_barber_pole_top_animated.png "Meine erste APNG-Datei")

Mit jedem Schritt nach oben sind die Farben ein Stück weiter rotiert. Von außen betrachtet ergibt das dann Spiralen. Von der Seite betrachtet, auf eine Ebene projeziert, sieht die gleiche Stange dann so aus:

![Animation: diagonale rote, weiße und blaue Streifen bewegen sich von rechts nach links](/file/fordite_barber_pole_side_animated.png)

Sieht ein bisschen langweilig aus und auch nicht wirklich nach sich drehendem Zylinder. Das liegt an der Projektion. Aber intern wird einfach der Winkel über die Zeit verändert, so dass der EIndruck entsteht, die Streifen wandern nach links.

### Wirbel-Animation

Ich habe auch wieder eine [Animation von einem interessanteren Motiv](https://www.youtube.com/watch?v=HFMb4e5tp6w) erstellt. Hat eine Weile gebraucht, bis ich eine brauchbare Kombination hingekriegt habe. Danke an alle, die mir Feedback und Farbideen gegeben haben.
