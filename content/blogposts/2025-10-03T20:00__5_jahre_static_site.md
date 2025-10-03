---
title: Fünf Jahre static-site-generator
filename: 5_jahre_static_site
date: 2025-10-03T20:00:00+02:00
update-date:
tags: rails, rust, blog, security, sicherheitslücken
category: meta
summary: Vor fünf Jahren bin ich von meiner auf Ruby On Rails basierenden Blogsoftware auf einen ebenfalls selbstgeschriebenen static site generator umgestiegen. Ein Rückblick und ein Vergleich.
image:
image-alt:
language:
---

Vor fünf Jahren, am 3. Oktober 2020, [ist meine neue Blogsoftware online gegangen](/blogposts/neustart). Ich war schon einige Zeit unzufrieden mit der alten, in Ruby on Rails geschriebenen [Blogsoftware, die ich 2012 eingeweiht hatte](/blogposts/1). Dafür gab es mehrere Gründe. Aber hat sich der neue Ansatz bewährt? Zeit für einen Rückblick.

### Updates, Sicherheitspatches

Der Hauptgrund für meine Unzufriedenheit mit dem Rails-Blog war, dass es für mich [immer schwieriger geworden war, Updates und Änderungen an meiner Blogsoftware zu machen](/blogposts/227). Das schließt auch Sicherheitsupdates mit ein. So hatte zum Beispiel der HTML-Sanitizer, den ich verwendete, eine bekannte Sicherheitslücke. Die neuere Version des Sanitizers funktionierte aber nur mit neueren Ruby-Versionen. Neuere Ruby-Versionen erforderten neuere Rails-Versionen (und umgekehrt: neuere Rails-Versionen erforderten neuere Ruby-Versionen). Zwischen den Rails-Versionen gab es aber breaking changes, also war jedes größere Update (und ich hätte mehrere machen müssen) eine Gefahr.

Zur Ehrenrettung von Ruby On Rails muss ich sagen, dass ich 2012 einfach noch nicht so ein erfahrener Entwickler gewesen bin. Sonst hätte ich vielleicht vieles besser gemacht, z.B. mehr Tests geschrieben, die mir Vertrauen gegeben hätten, dass auch nach dem Update noch alles gut funktioniert.

Auf der anderen Seite habe ich nicht ständig mit Ruby On Rails gearbeitet, und da Rails eine Menge [Konvention vor Konfiguration](https://de.wikipedia.org/wiki/Konvention_vor_Konfiguration) betreibt, und ich die Konventionen immer wieder vergessen habe, war es ziemlich schwierig für mich, mich in dem Code zurechtzufinden. Viele Dinge, die ich lieber explizit gehabt hätte, gingen automagisch. Wenn ich etwas auergewöhnliche Dinge tun musste, musste ich dafür an Rails vorbeiarbeiten, was die Sache zusätzlich schwierig gemacht hat.

Aber schauen wir uns mal den Zeitverlauf an: Anfangs war es noch leicht [Sicherheitslücken zu patchen](/blogposts/26). Gut vier Jahre später hatte ich aber schon [Updateprobleme](/blogposts/199). Ich hatte [Spamprobleme](/blogposts/203) bei den Kommentaren, für die ich keine schnelle Lösung hatte, weil das Entwickeln so umständlich war. Also hatte ich die Kommentarfunktion zunächst abgeschaltet. Die [Absicherung der Kommentarfunktion zog sich hin](/blogposts/206) und war erst [Anfang 2020 fertig](/blogposts/226). Welch eine Ironie, dass noch im selben Jahr die ganze Rails-Software ersetzt werden würde. Oder vielleicht auch einfach der Tropfen, der das Fas zum Überlaufen brachte.

Im Vergleich dazu die neue Blogsoftware: Auch nach fünf Jahren finde ich mich im Code gut zurecht. Schwerwiegende Sicherheitslücken in den Abhängigkeiten gab es keine. Updates der Abhängigkeiten waren nie ein Problem. Ich habe immer wieder neue Features hinzufügen können, zum Beispiel [Bilddimensionsattribute](/blogposts/lang_und_breit), Opengraph-Attribute und ein paar andere Kleinigkeiten. Auch der [Import von Blogposts aus meinem ganz alten Blog](/blogposts/import_export), den ich lange vor mir hergeschoben habe, war ganz einfach.

Kurz: Rails war einfach zu Komplex für meinen use case.

### Komplexität

Das Rails-Blog zu betreiben war ein Schmerz. Ich musste mich mit der [MySQL/MariaDB-Anbindung herumschlagen](/blogposts/148), es gab tagelang Ausfälle. Ich hatte [mit Phusion Passenger zu kämpfen](/blogposts/159), es gab Aufälle. Und manchmal [gab es einfach so Ausfälle](/blogposts/172).

Die Stabilität des Blogs hing von vielen Faktoren ab. MariaDB wurde regelmäßig über den Debian-Paketmanager aktualisiert. Apache auch. Ruby nicht, da gibt es einen eigenen Paketmanager (oder man installiert das über Debian, oder macht beides und hat ein absolutes Chaos oder man vergisst, welchen Weg man gewählt hat und hat erst recht ein absolutes Chaos). Phusion passenger musste immer wieder manuell neu gebaut werden. Das war kein Spaß.

Im Gegensatz dazu: Die neue Blogsoftware ist ein static site generator. Zum Betrieb des Blogs muss nur der nginx-Server regelmäßig Sicherheitsupdates bekommen. Probleme mit der Blogsoftware selbst hätten nur verhindert, dass ich _neue_ Blogposts veröffentliche. Aber da der Generator mit git versioniert war, hätte ich in dem Fall einfach eine Version zurückgehen können. Es gab aber nie ein Problem. Nebenbei: Der Grund dafür, dass ich erst 2024 [Brotli-Unterstützung im Blog hatte](/blogposts/brotli) lag daran, dass ich vorher manuell ein nginx-Modul hätte kompilieren müssen. Nein danke.

Rails hat einen Haufen Abhängigkeiten, die ich nicht durchblicke und eine Hand voll Abhängigkeiten, die ich selber hinzugefügt habe. Die neue Blogsoftware hat nur eine Hand voll Abhängigkeiten, und ich könne alle ersetzen (auch wenn das zumindest in einem Fall bedeuten würde, dass ich alle HTML-Templates neu schreiben müsste). Auch das ist Komplexität, die verringert wurde.

Das hat mich natürlich die Kommentarfunktion gekostet. Aber das war es mit wert. Ich kriege keine Panikschübe mehr, wenn ich die Blogsoftware anfassen muss, und die Kommentarfunktion wurde sowieso hauptsächlich von Spambots benutzt. Vielleicht kommen die Kommentare ja mal wieder, aber manuell, z.B. per Mail.

### Performance

Vermutlich kann nichts die Performance statisch ausgelieferter Dateien übertreffen. Zumal die Resourcen auch mit hoher Kompressionsrate vorkomprimiert sind. Die Rails-Software hingegen war immer ein bisschen langsam, besonders [bevor ich 2014 den BB-Code durch rohes HTML ersetzt habe](/blogposts/132). Hey, ich war unerfahren. Ich hätte von Anfang an Markdown verwenden sollen, aber das kannte ich damals noch nicht. BB-Code erschien mir sinnvoll. Bis es mir zu langsam wurde. Dann habe ich mir einen HTML-Sanitizer eingebaut und direkt HTML in die Beiträge geschrieben. Aber auch der Sanitizer war ein bisschen langsam. Ich habe mir überlegt, ob ich vielleicht die Ausgaben des Sanitizers noch cachen sollte.

Ich habe das nie umgesetzt, weil ich mich [Anfang 2020 für den static site generator-Ansatz entschieden habe](/blogposts/229). Ich hatte schon länger mit dem Gedanken gespielt, eine neue Blogsoftware (auch dynamisch) in rust zu schreiben, war aber davor zurückgeschreckt, weil es eine Menge Arbeit gewesen wäre. Ein Vortrag auf dem 36C3 hat mich dann auf die Idee gebracht, es doch lieber mit statischen Seiten zu versuchen.

### Fazit

Die neue Blogsoftware ist jetzt fünf Jahre alt und ich stehe um Größenordnungen besser da, als ich mit der alten Software dastand, als diese fünf Jahre alt war. Ich habe nie bereut, umgestiegen zu sein. Ich sehe keine Probleme darin, dass die aktuelle Software auch noch in zehn oder mehr Jahren laufen soll (vielleicht mit ein paar Änderungen oder Anpassungen, aber grundsätzlich dieselbe Software).
