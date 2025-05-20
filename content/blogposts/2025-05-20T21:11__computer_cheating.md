---
title: Der Computer schummelt!
filename: computer_cheating
date: 2025-05-20T21:11:15+02:00
update-date:
tags: tvtropes, machine learning, schummeln, glitch
category: hoellenmaschinen
summary: Machine learning ist in der Regel ein Optimierungsproblem in irgendeiner Form. Wenn wenn man nicht aufpasst, dann finden gute machine learning-Algorithmen Schlupflöcher in den Randbedingungen und „schummeln“. Ein paar Beispiele.
image:
image-alt:
language:
---

Durch TVTropes ist es bekannt: [The Computer Is a Cheating Bastard](https://tvtropes.org/pmwiki/pmwiki.php/Main/TheComputerIsACheatingBastard). Gut, da geht es um computergesteuerte Spieler in Computerspielen ([z.B. in der Anno-Reihe](/blogposts/old_1609799)), aber der Titel passt einfach zu gut.

2019 habe ich bei meinem damaligen Arbeitgeber einen kleinen, unterhaltsamen Talk gegeben. Ich habe ja im Studium einen Fokus auf Neuroinformatik (inkl. maschinelles Lernen) gelegt und mich auch nachher noch dafür interessiert. Was ich jetzt hier darstellen möchte ist hauptsächlich aus diesem Talk. Das bedeutet, das ist alles aus der vor-ChatGPT-Zeit. Über ChatGPT, die Probleme damit und dem aktuellen „AI“-Hype und warum ich den Begriff „AI“ (bzw. „KI“ auf deutsch) nicht mag, rede ich ein anderes Mal.

Hier geht es darum, wie Maschinenlernverfahren sehr gut funktioniert haben und trotzdem nicht das Ergebnis lieferten, dass die Menschen wollten. Das ist manchmal lustig aber auf jeden Fall lehrreich, und vielleicht kann man auch ein bisschen darüber philosophieren. Los geht's!

### Exkurs: Maschinelles Lernen

Erst einmal eine kurze Zusammenfassung, was ich überhaupt meine, wenn ich „Maschinenlernen“ schreibe. Ganz grob gesagt: Bei Maschinenlernverfahren gibt man einem Algorithmus Daten aus einem Szenario, die „Trainingsdaten“. Der Algorithmus verarbeitet die Daten dann (er „trainiert“ auf den Daten) und gibt ein Modell aus. Dieses Modell kann dann genutzt werden, um über neue Daten, die aus einem vergleichbaren Szenario stammen, irgendeine Aussage zu treffen. Grundsätzlich kann verschiedene Ansätze unterscheiden, die für verschiedene Anwendungsfälle nützlich sind.

Da wäre zunächst *überwachtes Lernen* (supervised learning). Hier hat man Trainingsdaten, bei denen man für jeden Datenpunkt ein gewünschtes Ergebnis hat. Das können diskrete Unterscheidungen sein (Klassifizierung), aber auch kontinuierliche, mehrdimensionale Werte (Regression). Beispiel: Eine Menge von Bildern und die (korrekte) Angabe darüber ob auf einem gegebenen Bild eine Katze zu sehen ist oder nicht. Im Idealfall kommt ein Modell heraus, dem man unbekannte Bilder füttern kann und dass dann korrekt entscheidet, ob auf dem Bild eine Katze ist oder nicht.

Dann ist da *unüberwachtes Lernen* (unsupervised learning). Hier hat man nur die Daten, aber kein gewünschtes Ergebnis. Das Verfahren muss dann selber entscheiden, wie es die Daten einteilt bzw. gruppiert. Das kann man zum Beispiel verwenden, um in großen Datenmengen Gruppierungen zu finden.

Als letztes gibt es da noch *reinforcement learning* (ich fasse das jetzt mal mit evolutionären Algorithmen zusammen). Hier hat man einen Agenten, der in einem Szenario eine Aufgabe durchführen soll. Macht er seine Aufgabe gut, kriegt er Punkte. Macht er sie schlecht, kriegt er Punkte abgezogen. Der Agent versucht, seine Punkte zu maximieren. Also im Prinzip klassische Konditionierung für Computer. Beispiel: Ein Agent läuft durch einen Irrgarten und muss ein Ziel finden. Je länger er braucht, desto mehr Punkte verliert er. Tritt er auf eine Falle, verliert er besonders viele Punkte.

Alle diese Verfahren haben zwei Dinge gemein: erstens lernen sie _Korrelationen_. Wenn sie zudem auch noch kausale Zusammenhänge lernen, dann ist das gut, aber vorrangig lernen sie Korrelationen. Zweitens funktionieren sie nur so gut, wie die Eingabedaten sind. [Garbage In, Garbage Out](https://de.wikipedia.org/wiki/Garbage_In,_Garbage_Out).

### Fahrzeugerkennung

Ein ganz [klassisches Beispiel](https://neil.fraser.name/writing/tank/) für ein Maschinenlernverfahren, das bei der Klassifizierung eine Abkürzung genommen hat, stammt aus den 80ern und wurde auch noch 2010 in einem meiner Kurse an der Uni erwähnt.

Das US-Militär wollte damals Panzer automatisch erkennen. Dazu haben sie ein entsprechendes Maschinenlernverfahren mit den Bildern von Panzern und Bildern ohne Panzer gefüttert. Getestet haben sie das Verfahren auch, indem sie nicht alle verfügbaren Bilder für das Training verwendeten, sondern einen Teil zurückhielten um zu testen, wie das Modell mit unbekannten Bildern klar kam.

Die Testergebnisse waren gut, also wollten sie das ausrollen. Weitere Tests ergaben dann aber, dass die Ergebnisse völlig zufällig waren. Was war passiert?

Alle Trainingsilder mit Panzer wurden an bei bewölktem Himmel aufgenommen. Alle Trainingsbilder ohne Panzer hatten blauen Himmel. Der Algorithmus hatte es also einfach und nur gelernt, blauen von grauem Himmel zu unterscheiden. Das hat sehr gut funktioniert, war aber nicht gewollt.

Ein ähnliches Problem ergab sich viel später in einem [Agenten, der darauftrainiert wurde, Bilder zu beschreiben](https://www.aiweirdness.com/do-neural-nets-dream-of-electric-18-03-02/). Der hatte zwar jede Menge Bilder zum Lernen gekriegt, aber wenn eine Wiese zu sehen war, waren da auch meist Schafe. Also hat er gelernt: Grüne Wiese = Schafe. Und wenn man ihn dann ein Foto einer grünen Wiese (ohne Schafe) vorlegte, behauptete er steif und fest, Schafe zu sehen. Warum auch nicht? Aus seiner Sicht waren dort Schafe. Nicht das, was _wir_ mit „Schaf“ meinen, aber das ist ja zweitrangig.

### Glitches in Physik-Engines

Aber nicht nur unausgewogene Trainingsdaten bringen Computer dazu, zu schummeln. Wenn man einen Algorithmus in einer virtuellen Umgebung trainiert, können die physikalischen Eigenschaften der Umgebung ausgenutzt werden. Und solche Physik-Engines (wie sie u.a. auch in Computerspielen eingesetzt werden) nehmen in der Regel grobe Vereinfachungen in Kauf, um eine plausible Physik in Echtzeit zu berechnen. Wäre ja auch doof, wenn man in einem Spiel für jedes Frame 10 Sekunden warten müsste, weil irgendwelche Details perfekt ausgerechnet werden müssen.

Diese Vereinfachungen sorgen natürlich dafür, dass sich die Physik unter bestimmten Bedingungen sehr, sehr falsch verhält. Ein bekanntes Beispiel ist [diese Reihe von Clips aus dem Spiel „Skate 3“](https://www.youtube.com/watch?v=vfl33Tn0pYc). Und oft genug finden Maschinenlernverfahren (meist irgendeine Form von Reinforcement Learning) Wege, diese Glitches auszunutzen um ihr Ziel einfacher zu erreichen.

Ein Beispiel ist [dieses Video](https://www.youtube.com/watch?v=ppf3VqpsryU). Der Autor hat einen genetischen Algorithmus darauf trainiert, eine Struktur möglichst lange an der Decke kleben zu lassen. Ergebnis: Die Struktur no-clipped durch die Decke und verabschiedet sich nach oben.

Oder [dieses Video](https://www.youtube.com/watch?v=Lu56xVlZ40M), wo Agenten darauf trainiert wurden, miteinander Verstecken zu spielen. Die eine Seite hatte irgendwann gelernt, ein Element out-of-bounds zu glitchen, dass die Sucher benötigten, um die sich Versteckenden zu finden. Bis die Sucher dann ihrerseits einen Weg gefunden haben, sich mit einem Glitch in die Luft zu schleudern um die anderen doch noch zu erreichen

In einem anderen Fall sollte ein Körper gebaut werden, der sich möglichst effizient fortbewegt. Ergebnis: Ein Körper, der sich in den Boden glitched, durch den Rückstoß Energie gewinnt und so mehr Energie herausbekommt, als er reinsteckt.

Es gibt viele Beispiele davon. Einige sind im Paper [The Surprising Creativity of Digital Evolution: A Collection of Anecdotes from the Evolutionary Computation and Artificial Life Research Communities](https://arxiv.org/pdf/1803.03453) zusammengefasst.

### Steganographie

Ob das bisher „Schummeln“ war, darüber kann man streiten. Das eine ist einfach eine schlechte Datenlage, das andere löst seine Aufgabe, wenn auch auf einem ungewollten Weg. Ich habe aber auch noch einen Fall, indem ein Verfahren im Training Daten in der Ausgabe versteckt hat, damit ein anderer Teil des Verfahrens sie wieder aufbereiten konnte, und zwar ohne, dass ein Teil dazwischen es bemerkt. Davon berichtet wurde im Paper [Cyclegan, a master of steganography.](https://arxiv.org/pdf/1712.02950)

Kurz zusammengefasst: Ziel war, ein Modell zu haben, dass aus Luftaufnahmen abstrakte Landkarten erstellt. Aber wie macht man das, wenn man nur begrenzt Trainingsdaten hat? Ganz einfach: Man trainiert zwei Modelle. Eins versucht, möglichst gute Karten aus Luftaufnahmen zu erstellen. Das andere wird darauf trainiert, handgenerierte Karten von Karten des ersten Modells zu unterscheiden. Die Idee: die beiden müssen quasi wettrüsten: die eine Seite muss immer besser im Generieren werden, die andere immer besser im Erkennen.

Problem: Die Ausgabe des ersten Models muss nichts mit der Eingabe zu tun haben. Einziges Ziel ist es, ein Bild zu erstellen, das das zweite Modell als echte Karte erkennt. Also kann es auch immer dasselbe Bild liefern.

Die Lösung: Man fügt noch ein drittes Modell hinzu, dass aus dem Kartenmaterial wieder ein Luftbild macht, das dem ursprünglichen Luftbild möglichs ähnlich sieht. So kann man sicherstellen, dass auch wirklich eine Karte zum aktuellen Luftbild erstellt wurde.

Das ganze funktioniert gut. Zu gut, wie die Forscher feststellten. Denn auf der Karte waren z.B. von Gebäuden nur die Umrisse zu sehen, Details (Strukturen auf dem Dach) gingen verloren. So weit, so gut, das erwartet man ja. Aber: auf dem wiederhergestellten Luftbild des dritten Modells _waren die Strukturen wieder da_ Die Strukturen, von denen dieses Modell überhaupt nichts wissen konnte, weil sie in den Eingabedaten nicht mehr vorkamen!

Oder waren die Daten vielleicht doch da, nur versteckt? Genau das war der Fall. Das erste Modell hatte grobe Informationen über die Strukturen in niederwertigen Bits versteckt. Gut genug, dass das zweite Modell sie nicht finden konnte, aber so, dass das dritte Modell sie nutzen konnte.

### Philosophie

Ich finde solche Beispiele immer schön. Denn sind wir alle nicht auch aus Glitches entstanden? Irgendwann mal in der fernen Vergangenheit haben Bakterien einen Glitch in der Physik-Engine unserer Welt genutzt, um Sonnenlicht viel effizienter zu nutzen (nur, dass dabei [ein giftiges Gas namens Sauerstoff freigesetzt wurde, war nicht geplant](https://de.wikipedia.org/wiki/Gro%C3%9Fe_Sauerstoffkatastrophe)). Die Evolution findet viele Lücken in der Physik der Welt und nutzt sie eiskalt aus. Wenn man Federn und Muskeln so und so anordnet? Hey, dann kann man fliegen! Dieses komische nutzlose Molekül? Wenn ich das produziere sterben die Tiere, die mich fressen! Hüpfen ist so anstrengend! Aber durch diesen Glitch mit elastischen Sehnen kann ich einen Teil der Energie zurückkriegen, wenn ich ihn wie in einer Feder speichere!

Auch wir Menschen sind gut darin, das Physik-Engine zu exploiten. Von prähistorischen [Speerschleudern](https://de.wikipedia.org/wiki/Speerschleuder), die die Wurfkraft eines Menschen vervielfachen können bis hin zu einer seltsamen Eigenschaft von Silizium die dafür sorgt, dass es [manchmal Strom leitet und manchmal nicht](https://de.wikipedia.org/wiki/Halbleiter), woraus wir dann Rechenmaschinen bauen können. Was ist das anderes, als Eigenschaften des Systems zu unseren Vorteilen zu nutzen?

Ich finde es schön, dass Maschinenlernverfahren das auch tun. Auch, wenn sie dann nicht das machen, was wir wollten. Aber sie funktionieren.
