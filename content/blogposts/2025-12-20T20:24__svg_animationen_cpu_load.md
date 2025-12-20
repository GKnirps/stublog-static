---
title: Animierte SVG und CPU-Last
filename: svg_animationen_cpu_load
date: 2025-12-20T20:24:47+01:00
update-date:
tags: svg, optimierung, animation, napstablook, mdn, browser, svgo
category: hoellenmaschinen
summary: Animierte SVG erzeugen hohe CPU-Last. Ich versuche, eine Lösung dafür zu finden, aber zufriedenstellend ist das nicht.
image:
image-alt:
language:
---

Ich habe ja vor ein paar Monaten schon davon geschrieben, wie [ein Kollege einen animierten Github/Gitlab contribution graph](/blogposts/pacman_contribution_graph) gebastelt hat, in Form einer animierten SVG. Ich finde das nach wie vor cool, aber ich habe keine solche Datei in den Blogpost eingebunden. Zum einen, weil die Dateigröße trotz alle Optimierungen noch ziemlich heftig war (250 kiB), zum anderen, weil alleine das Anzeigen der Animation bei mir 50% CPU-Last auf einem Kern erzeugt.

In diesem Artikel geht es um animierte SVG und deren CPU-Last. Eigentlich wären animierte SVG eine super Gelegenheit, mal wieder ein paar Bilder in das Blog zu bringen… allerdings möchte ich keine riesige CPU-Auslastung bei allen verursachen, die meine Seite besuchen, also verlinke ich die Animationen nur.

Angefangen hat meine neuerliche Besessenheit mal wieder mit Napstablook.

### Napstablook, animiert und tanzend

Ich bin gestern Abend über ein [Github-Repo mit einem animierten Napstablook gestoßen](https://github.com/izdracula/napstablook-dance). Die Grafiken selber stammen von [Joel Erhart](https://joelerhart.com/), die er auch [in diesem Youtube-Video](https://www.youtube.com/watch?v=WKJxIYve4QA) veröffentlicht hat. Ob das Github-Repo von ihm ist oder von einem Fan, weiß ich nicht.

Dieses Repo enthält jedenfalls eine Napstablook-Animation. Allerdings nur, wenn man eine SVG-Datei mir SPrites der verschiedenen Animationsschritte mit ein bisschen HTML und CSS mischt. Und ich habe mir gedacht: Hey, das kann man doch sicher auch standalone machen. Also habe ich [das Repo geklont und mich an die Arbeit gemacht](https://github.com/GKnirps/napstablook-dance).

#### Einschub: SVG-Editoren und ihr komischer SVG-Code

Meistens wird der SVG-Code nicht von Hand geschrieben. Das ist verständlich, denn SVG von Hand zu bearbeiten ist eine Menge Arbeit und, um ehrlich zu sein, ich habe keine Intuition dafür, wie ich die Punkte in Bezierkuven setzen muss, damit sie so aussehen, wie ich es will. Also Benutzt man visuelle Editoren wie z.B. [Inkscape](https://inkscape.org/), diese eine Adobe-Tool dessen Namen ich vergessen habe oder einen Haufen anderer SVG-Editoren. Manche Editoren sind auch für bestimmte Zwecke gemacht (z.B. für Diagramme), lassen ihre Ergebnisse aber auch als SVG exportieren.

Und dieses exportierte SVG ist, meiner Erfahrung nach, oft sehr unschön. Zum Beispiel wird anstelle von SVG-Attributen wie `fill` und `stroke` eine Menge CSS eingefügt wie ich schon bei meinem [nichtanimierten Napstablook-SVG](/blogposts/napstablook_svg) feststellen musste. Dann werden Koordinaten und Skalierungen willkürlich gesetzt, um sie danach mit einem `transform`-Attribut auf die richtige größe zu skalieren und an die richtige Position zu schieben.

Oft werden auch Gruppierungs-Tags (`<g>`) recht willkürlich verwendet, um Elemente zu gruppieren. Und, was ich speziell in diesem Fall feststellen musste: Statt den `stroke` eines Pfades zu nehmen, um eine Linie zu zeichnen, wurde mit dem Pfad eine zweidimensionale Fläche erzeugt, die dann mit `fill` gefüllt wurde. Obwohl es in dem Bild eigentlich nur zwei etwa gleich starke Linien gibt. Wenn man das so macht, kann man das Element natürlich nicht mehr weiß füllen… also erstellte die Software einen zweiten Pfad, der die inneren Umrisse des ersten Pfades abbildet. Oh, und der beide Pfade sind auch sehr lang.

Ich habe eine ganze Menge Müll herausgeworfen und es geschafft, die Größe der Datei von etwa 200 kiB auf etwa 100 kiB zu reduzieren. Immer noch groß, aber ich habe auch nicht alle Optimierungsmöglichkeiten ausgenutzt.

Lustig ist, dass man an der Struktur der SVG-Datei gut erkennen kann, wie die erschaffende Person gearbeitet hat, vieles davon zeigt auf verschiedene Arbeitsschritte hin, z.B. die zeigen Translationen und Transformationen vermutlich, wie der Künstler in diesem Fall Objekte hin- und hergeschoben hat. Ist ja schön, aber es sollte in einem SVG-Editor eine Export-Funktion geben, die diesen ganzen Kram zusammenstreicht um ein einfacheres SVG zu erzeugen. Aber genug der Ausschweifung.

#### Der animierte Napstablook, standalone

Ich habe einige Zeit gebraucht, die größtenteils daraus bestand, mich in der Ursprungsdatei zurechtzufinden, unnützen krams wegzuwerfen und Translationen zu berechnen. Am Ende jedoch hatte ich die Animation, die ich auch zu meinem geklonten Repo auf Github gepushed habe. Die Sache hat nur einen Nachteil: Meine CPU-Lst geht auf 30% hoch, sobald ich die Animation im Browser betrachte.

### Die CPU-Last animierter SVG

30% auf einem halbwegs modernen CPU-Kern… das ist zu viel für einen kleinen animierten Geist in schwarz-weiß. Aber wie gesagt, die SVG-Pfade für den Geist sind sehr lang und umständlich. Also nehme ich eine einfachere Animation. So einfach wie möglich. Ich nehme das Beispiel von der [MDN-Dokumentation zum `<animate>`-Tag](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/animate). Das ist nur ein Quadrat, und die Animation ist so, dass sie langsam die Ecken immer weiter abrundet, bis ein Kreis dabei heraus kommt, und wieder zurück. Ad infinitum. Das kann doch nicht so schlimm sein, oder?

**70 Prozent?!?**

Mist. Dieses fast-Minimalbeispiel braucht sogar noch mehr CPU-Leistung. Und es macht auch keinen Unterschied, ob das Bild groß skaliert oder klein ist. Oder ob man die Animation verlangsamt oder beschleunigt. Es braucht.

Ich kann doch nicht die einzige Person mit diesem Problem sein, oder?

Nein, natürlich nicht. Vor über 7 Jahren schon hat zum Beispiel jemand [auf Stackoverflow um Hilfe gebeten](https://stackoverflow.com/questions/49906314/animated-svg-spinner-uses-excessive-cpu-and-gpu). Und das ist kein alleiniges Firefox-Problem. Chrome (bzw. Chromium) macht das Gleiche. Und damit in aller Wahrscheinlichkeit auch Safari und Edge.

Ich habe noch ein paar Experimente angestellt. Bewegen von Elementen (in diesem Fall eines Quadrats) hat auch für 70% Auslastung gesorgt (Beispiel von der [MDN-Seite über das `repeatCount`-Attribut](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/repeatCount)). Tatsächlich bin ich mit den 20% noch ganz gut dabei gewesen.

Ironischerweise hat [dieses Tutorial, um SVG-Animationen auf Page Speed zu optimieren](https://www.svgator.com/blog/svg-optimizations-improve-page-speed/) als Titelbild eine animierte SVG, die meine CPU-Last auf 200% bis 250% hochtreibt. Super hilfreich ist die Seite auch nicht. Ein paar gute Tipps (z.B. zum Verkleinern der SVG-Dateien) sind aber dabei.

### Andere Animationen

Auch CSS-Animationen können sehr teuer sein. Ich habe da nicht viel ausprobiert, aber ich vermute mal, dass Animationen, die das Seitenlayout verschieben, sehr teuer sind, weil dabei ständig alles neu berechnet werden muss. Animationen, die nur sich selbst betreffen, dürften weniger rechenlastig sein. [Die über CSS animierte Ursprungsversion des tanzenden Napstablook](https://izdracula.github.io/napstablook-dance/) zum Beispiel sorgt nur für eine 15% Last auf einem Kern.

Animierte Rastergrafiken (z.B. animierte GIF-Bilder) funktionieren hingegen sehr effizient. Ich vermute mal, dass dort einfach nur die Pixel ausgetauscht werden, während bei SVG potentiell eine Menge Fließkommazahlen berechnet werden müssen, um das Bild richtig darzustellen.

Vielleicht kann ich also hoffen, dass sich die SVG-Animations-Renderer mit der Zeit verbessern, und z.B. alte Zustände cachen, um deutlich effizienter zu rendern. Bis dahin muss ich selber herausfinden, wie ich SVG-Animationen möglichst effizient mache.

Wenn ich etwas herausfinden sollte, schreibe ich hier wieder etwas dazu.

### PS: svgo

Ach ja: Seit vorsichtig mit SVG-Optimierungstools wie `svgo`, wenn ihr animierte SVG-Dateien habt. SVGO fügt nämlich mehrere Pfade zu einem Pfad zusammen, entfernt Gruppen und IDs, und ignoriert dabei, dass die vielleicht von Animationen als getrennte, identifizierbare Elemente benötigt werden. Man kann das teilweise wegkonfigurieren, aber das ist ein bisschen aufwändig und man muss trotzdem aufpassen.
