---
title: Noch ein Fordite-Update
filename: fordite_update_2
date: 2022-11-06T19:21:39+01:00
update-date: 2022-11-07T20:45:00+01:00
tags: fordite, kunst, digitale kunst, digital art, rust
category: hoellenmaschinen
summary: Ich habe ein paar kleine Verbesserungen an Fordite vorgenommen und eine neue Animation hochgeladen.
image: /file/fordite_fire_god_float.webp
image-alt: Ein fordite-Bild, das eine abstrakte Darstellung eines Feuergottes sein könnte.
---

Ich bin endlich mal wieder dazu gekommen, an [Fordite](/blogposts/fordite) weiterzuarbeiten.

Es gab nicht viele Verbesserungen (ich warte auch immer noch auf Rückmeldung von jemandem, der mir etwas über Berechnungen von zueinander passenden Farben sagen wollte), aber dafür gibt es auch eine neue Animation.

### Verbesserung 1: Fix für Zufallsauswahl von Farben

Ich habe einen kleinen Bug behoben, der bei ein paar Farbschichtschemata dafür gesorgt hat, dass die Farben nicht komplett gleichverteilt waren. Die Änderungen sind vermutlich kaum sichtbar, aber vermutlich werden einige Konfigurationen mit demselben seed jetzt anders aussehen als vorher.

### Verbesserung 2: Höhe ist jetzt Fließkommazahl statt Ganzzahl

Für die Entscheidung, welche Farbe für einen Pixel gewählt wurde habe ich ursprünglich eine ganzzahlige Höhenkarte verwendet.

Das war am Anfang auch gut, aber dann habe ich Fordite umgebaut und einige Farbschichtenschemata hinzugefügt, mit denen das Probleme bereitet. Die Rundungsfehler haben zu seltsamen Artefakten geführt.

Jetzt habe ich die Höhenberechnungen auf Fließkommazahlen umgestellt, und die Verbesserung ist deutlich sichtbar.

Vorher:

![Das alte Feurgott-Fordite. Viele Kanten sind gezackt, andere Stellen blockiger als nötig](/file/fordite_fire_god.webp "Der Feuergott flimmert ein bisschen")

Nachher:

![Das neue Feuergott-Fordite. Die Kanten sehen deutlich runder und natürlicher aus](/file/fordite_fire_god_float.webp)

Reguläre Kantenglättung habe ich absichtlich weiterhin nicht eingebaut. Erstens sind die pixeligen Kanten Teil des Kunstwerks, zweitens habe ich keine Lust, das einzubauen und drittens lassen sich die Bilder so viel besser komprimieren.

### Verbesserung 3: Nebenläufigkeit

Bisher hat Fordite immer nur einen Thread auf einmal verwendet. Das war für viele Anwendungen gut, wenn ich allerdings die Frames für eine lange Animation erstellen wollte, dauerte mir das zu lang.

Nun gibt es vermutlich eine Menge Optimierungspotential, aber das Einfachste war vorerst, die Bildgenerierung zu parallelisieren. Da jedes Bild unabhängig von den anderen gerendert wird, und das Rendern der Bilder am meisten Zeit kostet, kann man so die Zeit auf einen Bruchteil verkürzen, so lange man entsprechend viele Prozessorkerne hat.

Die Anzahl der Threads wird hier automatisch ausgewählt, maximal das Minimum aus Anzahl zu rendernder Bilder und der auf diesem System möglichen Parallelität. Letztere habe ich mit der Rust-Funktion [available_parallelism](https://doc.rust-lang.org/stable/std/thread/fn.available_parallelism.html) bestimmt. Das mag nicht _immer_ das erwartete Ergebnis liefern, sollte aber in den meisten Fällen brauchbar sein.

Irgendwann baue ich vielleicht noch ein, dass man die Zahl der Threads selber bestimmen kann (als Kommandozeilenparameter oder so). Das hat aber keine Priorität. Außerdem gibt ersetzt das keine anderen möglichen Optimierungen.

Aufpassen sollte man allerdings, wenn man mehrere Konfigurationen hat, die in dieselbe Datei schreiben. Nach wie vor überprüft Fordite nicht, ob das der Fall ist. Bisher wurde dann einfach nur das letzte Bild mit einem entsprechendem Namen erstellt, alle anderen wurden überschrieben. Das ist jetzt nicht mehr stabil und kann je nach Timing auch zu Fehlern führen. Also aufpassen!

### Tanz eines Feuergottes

Zuletzt habe ich endlich eine Animation fertiggestellt, die mir schon lange im Kopf herumschwirrte. Als Basis dient das oben gezeigte Feuergott-Fordite.

Der Feuergott besteht aus einer kreisförmigen Sinuskurve als Höhenkarte und zylinderförmigen Farbschichten. Im Gegensatz zu den anderen Animationen habe ich hier nicht die Höhenkarte über die Zeit verändert, sondern wie die Farbschichten erstellt werden.

Zur Erinnerung: Die zylinderförmigen Farbschichten sind Schichten, die sich zylinderförmig um eine zentrale Achse wickeln. Ich habe mir jetzt zwei Punkte genommen, die sich jeweils in einem gewissen quaderförmigen Bereich herumbewegen und von den Wänden dieses Bereiches abprallen. Durch diese zwei Punkte läuft die Achse des Zylinders.

[Das Ergebnis ist wieder auf Youtube](https://www.youtube.com/watch?v=Nb-YtaRq5mo) und gefällt mir recht gut.

Ursprünglich wollte ich auch die Höhenkarte verschieben. Das hat aber zu seltsamen Bildsprüngen geführt, deswegen habe ich das wieder rausgenommen. Ich hätte der Sache auch auf den Grund gehen können, aber dazu hat mir der Nerv gefehlt.

Dadurch ergibt sich eine Auffälligkeit: in der Bildmitte ist eine Art Fokuspunkt. Ob das jetzt gut ist oder nicht, lasse ich mal dahingestellt.

Für das Script zu dieser Animation gibt es noch eine ganze Menge Stellschrauben, mit denen man herumspielen kann. Wer Lust dazu hat (oder den seltsamen Bildsprüngen bei bewegter Höhenkarte auf den Grund gehen möchte), [findet die Konfiguration hier](https://gitlab.com/GKnirps/fordite-configurations/-/tree/master/fire_god_video).

### Update 2022-11-07:

Nach kurzer Suche bin ich darauf gekommen, was beim Bewegen der Höhenkarte schiefgelaufen ist: Ein Vorzeichenfehler. Den habe ich behoben, daneben noch ein paar andere Tweaks gemacht und ein weiteres [Feuergott-Video](https://www.youtube.com/watch?v=gb2jcQbivro) hochgeladen.
