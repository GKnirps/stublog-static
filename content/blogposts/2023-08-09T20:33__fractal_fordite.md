---
title: Fractal Fordite
filename: fractal_fordite
date: 2023-08-09T20:33:47+02:00
update-date:
tags: fordite, kunst, digital art, diamond-square-Algorithmus, digitale kunst, rust, fractal landscape, computerspiele, breaking changes
category: hoellenmaschinen
summary: Es gibt einige neue Features in meinem Fordite-Programm, z.B. fraktale Höhenkarten. Allerdings gibt es auch ein paar breaking changes.
image: /file/fordite_blue_corner_fuzzy.webp
---

Nachdem ich neulich [ein neues Farbschichtenschema für Fordite](/blogposts/fordite_barber_pole) gebaut habe, habe ich mich letzte Woche an eine neue Höhenkarte gesetzt.

Kurze Erinnerung: [Fordite](/blogposts/fordite) ist ein Programm, das nach der Inspiration von echtem [Fordit](https://en.wikipedia.org/wiki/Fordite) Bilder erstellt. Für nähere Informationen bitte die verlinkten Blogposts lesen, [durch die Tags stöbern](/tags/fordite) und den [Code](https://gitlab.com/GKnirps/fordite) anschauen. Für mehr Beispielkonfigurationen gibt es [ein eigenes Repo](https://gitlab.com/GKnirps/fordite-configurations).

### Fraktale Landschaften

Es fing damit an, dass mir die bisherigen Höhenkarten zu gleichmäßig waren. Paraboloide, Sinuskurven, schiefe Ebenen… damit kann man schöne Bilder machen, aber die Linien zwischen zwei Farben sind immer sehr ordentlich. Saubere Kurven, geraden, etc. Echtes Fordit hat mehr Unregelmäßigkeiten.

Nun, wie erstellt man zufällige Höhenkarten? Einfach zufällige Werte nehmen, nach welcher Verteilung auch immer, gibt keine natürlich aussehende Höhenkarte und würde im Bild hinterher einfach Farben durcheinander werfen.

Glücklicherweise bin ich nicht der erste, der so ein Problem hat. In der Computerspieleentwicklung gibt es schon seit Jahrzehnten das Konzept von [fractal landscapes](https://en.wikipedia.org/wiki/Fractal_landscape) um halbwegs natürlich aussehende Landschaften zu generieren.

Ich habe mir auf Gutdünken einen Algorithmus herausgegriffen, den [diamond-square-Algorithmus](https://en.wikipedia.org/wiki/Diamond-square_algorithm). Nach einam ganzen Haufen off-by-one-Fehlern hat das auch wunderbar funktioniert:

![Ein schwarz-weiß-Bild. Manche Regionen sind eher dunkel, andere sind hell. Abgesehen davon ist kein Muster erkennbar](/file/fordite_diamond_square_hm.webp)

So weit, so gut. Ich habe ein paar Bilder damit erstellt, war aber nicht wirklich zufrieden. Warum? Darauf komme ich später zurück.

### Breaking Changes: Farbschichten

Um besser darstellen zu können, warum ich nicht zufrieden war, wollte ich Bilder mit verschiedenen Farbschichten bauen, die alle dieselbe diamond-square-Algorithmus-Höhenkarte verwenden. Für manche war das kein Problem, andere hatten aber bis dato keine Möglichkeit, die Dicke der Schichten einzustellen.

Die Dicke war standardmäßig dünn, für einfache, ebene Schichten war es nur `1`. Wenn man da jetzt eine zufällige Höhenkarte draufpackt, hat man sehr viele Farbwechsel, denn obwohl die Höhenkarte nicht komplett wild hin- und herspringt unterscheidet sich jeder Punkt mit hoher Wahrscheinlichkeit von seinem Nachbar.

Also mussten diese Konfigurationen angepasst werden. Ich habe mir erst einmal auf die waagerechten Ebenen (`horizontal_planes` in den config-Dateien) und die schrägen Ebenen (`inclined_planes` in den config-Dateien) konzentriert. Für die anderen kommt das vielleicht später.

Ich hatte gehofft es so einrichten zu können, dass, wenn man den neuen Parameter weglässt, alles weiterhin funktioniert, so dass alte Konfigurationen rückwärtskompatibel sind. Das habe ich mit [serde](https://serde.rs/) aber nicht hingekriegt. Also gibt muss man jetzt alte configs anpassen, damit sie wieder laufen.

Beispiele dafür, wie die neuen configs aussehen müssen, findet man [hier](https://gitlab.com/GKnirps/fordite/-/tree/master/sample_cfg). Wenn man möchte, dass die Bilder so aussehen wie bisher, empfehle ich für `horizontal_planes` den Wert `1` für den Parameter `layer_thickness`. Für `inclined_planes` ist es unterschiedlich, weil die Dicke bisher von der maximalen Höhe der Höhenkarte abhing. Da muss man ein bisschen herumprobieren.

### Langweilige Bilder

Warum war ich also nicht zufrieden mit den diamond-square-Bildern? Hier zur Veranschaulichung mal Bilder mit verschiedenen Farbschichtenschemata und derselben Höhenkarte:

![zerfranste Farbstreifen durchziehen unregelmäßig das Bild](/file/fordite_ds_barber_pole.webp "Barber Pole")
![zerfranste Farbstreifen durchziehen unregelmäßig das Bild, im großen und ganzen aber von oben links nach unten rechts](/file/fordite_ds_cylinder.webp "Zylinder")
![ein Netz aus zerfransten Farbstreifen umschließt unregelmäßige farbige Flächen](/file/fordite_ds_h_planes.webp "waagerechte Ebenen")
![links ist eine durchgehend grüne Fläche, rechts sieht das Bild so ähnlich aus wie die anderen](/file/fordite_ds_i_planes.webp "schiefe Ebenen")
![zerfranste Streifen… ach, ihr wisst schon](/file/fordite_ds_sphere.webp "Kugel")

Versucht mal zu raten, welches Bild mit welchem Farbschichtenschema erzeugt wurde. Die Höhenkarte ist überall gleich. Die Lösung steht im Titel des Bilders (darüberhovern auf Geräten mit Maus, antippen für Mobile Browser, wenn ich mich recht entsinne).

Worauf ich hinaus will: Man kann zwar Unterschiede zwischen den Farbschichtenschemata erkennen (besonders die schiefen Ebenen stechen wegen des große-einfarbige-Fläche-quirks heraus), aber ansonsten sehen die sich vom Prinzip her alle recht ähnlich.

### Breaking Changes: Die Rettung naht, doch vorher noch ein paar Umbauten

Um die Lösung, die mir vorschwebte zu bauen, musste ich ein bisschen Umstrukturieren. Insbesondere habe ich die Höhenkarten-Transformation (bisher Verschiebung und Rotation) aus der allgemeinen Konfiguration in die einzelnen Höhenkartenkonfigurationen verschoben.

Das zerstört mal wieder bestehende Konfigurationsdateien. Die zu reparieren ist nervig, aber nicht sehr kompliziert. Damit waren nun alle Voraussetzungen geschaffen für das:

### Addieren von Höhenkarten

Meine Idee war: Vielleicht ist es zu viel, die ganze Höhenkarte zufällig zu machen. Vielleicht kann man ja mit einer regelmäßigen Höhenkarte anfangen, und dann eine zufällige Höhenkarte dazuaddieren.

Mit den im vorherigen Abschnitt erwähnten Änderungen an den Konfigurationsdateien war das sehr einfach umzusetzen. Dieses Mal macht es sogar keine configs kaputt! Es gibt einen neuen Höhenkartentyp, `sum`, der als Parameter eine Liste von Höhenkarten und Gewichten entgegennimmt.

Das Addieren funktioniert wie folgt:

1. generiere alle Höhenkarten
2. normalisiere alle Höhenkarten auf denselben Wertebereich
3. multipliziere alle Höhenkarten mit ihren Gewichten
4. addiere die Höhenkarten
5. skaliere das Ergebnis auf die gewünschte Höhe

Und _damit_ habe ich endlich das, was ich will. Und nicht nur das: Ich habe mir direkt einen ganzen Haufen zusätzlicher Optionen geschaffen, schließlich kann man beliebige Höhenkarten addieren.

Hier ein Beispiel, wie das aussehen könnte. Zuerst ein Bild, dass ich früher schon einmal geposted habe (bevor ich [die Höhenkarten auf float umgestellt habe](/blogposts/fordite_update_2), deswegen hier eine neue Version):

![ein blaues Fordite-Bild. Die Kanten sind zwischen zwei Farbschichten sind gleichmäßig geschwungen](/file/fordite_blue_corner_smooth.webp)

Im Vergleich dazu: die gleiche Konfiguration, aber mit 10% Gewichtung eine zufällige Höhenkarte draufaddiert:

![ein ähnliches Bild wie vorher, aber die Kanten sind nicht mehr so glatt und gleichmäßig. Sie sehen eher aus, als hätte jemand sie von Hand gezeichnet](/file/fordite_blue_corner_fuzzy.webp)

Ich will nicht zwingend sagen, dass eine Version besser ist als die andere, aber es ist definitiv ein Unterschied.

### Breaking changes: `two_sine`

Zu guter Letzt gibt es noch einen dritten breaking change: Die `two_sine`-Höhenkarte fällt weg. Ihren Effekt kann man jetzt schließlich mit dem Addieren von zwei `sine`-Höhenkarten erreichen.

### Schlusswort

Ich werde jetzt erst einmal mit den neuen Optionen herumspielen und vielleicht auch noch ein Video erzeugen. Mir ist auch die Idee gekommen, dass ich vielleicht Höhenkarten nicht nur addieren, sondern auch multiplizieren kann. Mir sind auch schon ein paar Ideen gekommen, wozu das nützlich sein kann.

Was außerdem noch ansteht: Ich möchte separate Zufalls-seeds für die Farbschichten anbieten. Ich habe schon für den diamond-square-Algorithmus einen (optionalen) eigenen seed eingeführt. Hätte ich das nicht, wären die Farbschichten anders als ohne diamond-square. Aber all das drängt nicht.
