---
title: Optimier mich!
filename: blog-optimization
date: 2020-10-11T00:18:31+02:00
update-date:
tags: optimization, premature optimization, brotli, zopfli, kompression, make
category: meta
summary: Optimierungen in meinem Bloggenerator. Manche aus Spaß, manche aus Notwendigkeit.
---

> Premature optimization is the root of all evil.

— [angeblich C.A.R. Hoare, ist aber umstritten](https://en.wikiquote.org/wiki/C._A._R._Hoare#Attributed)

…aber manchmal macht vorzeitige Optimierung [einfach Spaß](https://tvtropes.org/pmwiki/pmwiki.php/Main/EvilFeelsGood "Evil feels good"). Ich habe auch so eine Art Kompressionsfetisch entwickelt (nein, nicht [so eine Art](https://xkcd.com/598/) Kompressionsfetisch). Zum Beispiel ist das favicon dieses Blogs eine auf 212 Byte herunterkomprimierte PNG-Datei. Der HTTP-Overhead diese Datei zu übertragen ist alleine größer als die Datei selbst. Doch zur Kompression später mehr.

Spätere Optimierung hingegen macht weniger Spaß, ist aber gelegentlich notwendig.

So auch bei der Blogsoftware, mit der ich dieses Blog hier generiere (seit Kurzem veröffentlicht [auf github](https://github.com/GKnirps/stublog-static). Und ich gebe zu, ich habe auch tatsächlich eine vorzeitige Optimierung eingebaut, die ich später bereut und wieder entfernt habe: Ich habe das Parsen und das in-html-Umwandeln des Markdowns von Blogpost an eine Stelle verlagert, wo es eigentlich nicht hingehört, damit es nur einmal durchgeführt werden muss (anstatt von bis zu drei Mal: einmal für den Blogpost selber, einmal für die Archivseite und einmal für die Homepage).

Allgemein ist es in Rust relativ verführend, Optimierungen einzubauen. Man kann es aber auch sein lassen, und ist damit trotzdem meistens schnell genug. In manchen Fällen, wie zum Beispiel [xml-rs](https://crates.io/crates/xml-rs) vs. [quick-xml](https://crates.io/crates/quick-xml) habe ich in der Vergangenheit feststellen müssen, dass ich bei einer mehreren hundert MB großen Datei einen signifikanten Unterschied (ein paar Minuten vs. ein paar Sekunden) beim Parsen habe.

Ich habe an einigen Stellen aus Spaß optimiert, an anderen Stellen aber auch Sachen einfach kopiert. Im Endeffekt macht es wahrscheinlich keinen großen Unterschied: Ich lade mehrere hundert kleine Dateien und schreibe noch mehr kleine Dateien. Der größte Teil der Laufzeit dürfte also für IO draufgehen. Trotzdem läuft der Generator in ein paar Millisekunden durch:

    $ time target/release/generator ../content ../dist

    real    0m0,045s
    user    0m0,028s
    sys     0m0,017s

Und selbst wenn die Compileroptimierung abgeschaltet wird, läuft das Ding in einer Viertelsekunde durch:

    time target/debug/generator ../content ../dist

    real    0m0,244s
    user    0m0,192s
    sys     0m0,024s

Die Zahlen schwanken natürlich recht stark, je nachdem wie sich der Rechner gerade fühlt. Aber die Größenordnung bleibt gleich. Kurz gesagt, der HTML-Generator ist schnell genug. Und der ist nicht einmal parallelisiert.

### Kompression

Ganz anders sieht es aus, wenn ich die Schritte danach betrachte: Da wird nämlich jede einzelne HTML-Datei (und auch ein paar andere Dateien) komprimiert, damit sie später bei der Übertragung per HTTP kleiner ist. Ich unterstütze hier zwei Kompressionsverfahren: gz und [brotli](https://de.wikipedia.org/wiki/Brotli).

Aber aufgrund des oben genannten Kompressionsfetisches kann ich natürlich nicht einfach gz nehmen und die Dateien komprimieren. Oh nein, ich muss [Zopfli](https://de.wikipedia.org/wiki/Zopfli) nehmen. Zopfli optimiert recht gut, braucht aber gefühlt ewig pro Datei. Ich könnte vermutlich sogar noch ein paar Bytes weniger herauskitzeln, aber das ist mir mein Kompressionsfetisch dann doch nicht wert. Das macht die gz-Komprimierung tatsächlich langsamer als die Brotli-Komprimierung, wobei letztere immer noch kleinere Ergebnisse liefert.

Die Sache mit Brotli hat nur einen Haken: Ich habe bisher noch keine einfache Möglichkeit gefunden, Brotli stabil als transfer-encoding für nginx einzusetzen. Es gibt dafür ein [nginx-Modul von Google](https://github.com/google/ngx_brotli), aber das muss man manuell bauen, und zwar mit genau der richtigen nginx-Version und der richtigen nginx-Konfiguration. Ich müsste also vermutlich nach jedem nginx-Update manuell dieses Modul neu bauen, sonst segfaultet mir das noch um die Ohren. Alternativ gibt es ein [offiziell unterstütztes Modul](https://www.nginx.com/products/nginx/modules/brotli/), aber nur für die Nutzer der kommerziellen nginx-Variante.

Ich erstelle also Haufenweise brotli-komprimierte Dateien ohne sie zu nutzen. Da haben wir sie wieder, die vorzeitige Optimierung, die am Ende nichts bringt.

### Make to the rescue

Aber wie gesagt, die Kompression verschlingt eine Menge Zeit. Glücklicherweise benutze ich make, was anhand des Zeitstempels von Quelldateien und Zieldatei entscheiden kann, ob ein target (zum Beispiel eine Kompression) neu erstellt werden muss. Unglücklicherweise erstellt der HTML-Generator einfach immer alle Dateien neu, was dazu führt, dass make alles neu komprimieren will.

Oder sagen wir lieber „erstellte“. Hier kommen wir zur nachträglichen Optimierung. Ich habe dann den Generator so umgebaut, dass er sich auch die Quell-und-Zieldateien anschaut und danach entscheidet, welche neu gebaut werden müssen. Das macht den Generator zwar nur unwesentlich schneller (etwa Faktor 2, was in absoluten Werten nicht viel ist), verringert jedoch drastisch den Kompressionsaufwand, wenn nur wenig geändert wurde.

Auch hier gibt es einige Nachteile: Der Generator erkennt nicht, wenn er sich selber geändert hat und baut so nichts neu, auch wenn durch die Änderung alles anders aussehen würde. Da war ich zu faul, mir eine Lösung zu überlegen und muss jetzt semi-manuell die HTML-Dateien löschen. Ist meiner Meinung nach hier aber zu verkraften. Niemand außer mir benutzt den Generator, und ein Umbau um das Problem zu lösen würde noch einiges an Komplexität in den Generator bringen.

Ein weiteres Problem ist, dass make es nicht mag, wenn andere Programme während der Ausführung von make an den Quelldateien herumspielen. So wie ich make verstehe, wird ganz am Anfang entschieden, wwelche targets alle zu bauen sind. Das ist natürlich doof, wenn einige Quelldateien erst durch ein phony-Target (der Generator) erstellt werden. Ich habe ein paar Workarounds dafür drin, aber schön ist das nicht. Vielleicht ist make einfach nicht das richtige Werkzeug für diesen use case. Auf der anderen Seite ist es praktisch überall verfügbar, also bleibe ich erst einmal bei make.

### Parallelisiert

Make kann nämlich auch parallelisieren. Da Make Abhängigkeiten zwischen targets auflöst, kann make auch sehr gut entscheiden, was parallel laufen kann. Und einzelne Dateien unabhängig von einander zu komprimieren ist ziemlich gut parallelisierbar. Schauen wir uns zunächst mal die unparallelisierte Version an:

    $ time make

    […]

    real    4m6,925s
    user    4m4,581s
    sys     0m2,326s

Ok, vier Minuten. Das ist viel. Das geht doch besser, oder? Probieren wir es mal mit maximal vier parallelen Prozessen (vorher natürlich einmal ein clean):

    $ make -j4

    […]

    real    1m10,153s
    user    4m38,362s
    sys     0m2,114s

Ordentlich. Das ist fast eine vierfache Verbesserung. Probieren wir mal mehr.

    $ make -j8

    […]

    real    0m56,804s
    user    7m18,364s
    sys     0m2,854s

Immer noch besser, aber nicht mehr ganz so viel besser. Mag daran liegen, dass ich nur vier physische Kerne habe. Oder dass das I/O der Festplatte nicht mitkommt. Vermutlich ersteres, denn das Einlesen der Dateien ist hier nur ein winziger Teil der Zeit die so ein Kompressionsprogramm zum Arbeiten braucht.

### Wo kann man noch verbessern?

Im Großen und ganzen bin ich ziemlich zufrieden, wie der Krams läuft. Die Dateien kopiere mich mit rsync zum Server, wobei auch hier nur veränderte Dateien übertragen werden. Wenn man nicht viel geändert hat, geht das erstellen und Komprimieren des HTMLs auch ziemlich fix. Der Server liefert wann immer möglich komprimierte Dateien aus und beantwortet Anfragen wenn möglich mit einem 304 not modified.

Zwei Dinge möchte ich aber noch verbessern: Zum einen die oben erwähnte Brotli-Kompression (sobald ich einen Weg gefunden habe, sie mit nginx zu benutzen). Zum anderen sind da die assets.

Momentan ist die Ansage des Servers, dass alles fünf Minuten lang gecached werden darf. Für manche Dateien, zum Beispiel das css ist das aber viel zu wenig. Das kann man praktisch ewig cachen, wenn es sich nicht verändert. Allerdings will man auch, dass es tatsächlich neu geladen wird, wenn es sich verändert.

Und kluge Leute sind schon vor langem auf die Idee gekommen, wie man das löst: man hängt einfach einen hash des Dateiinhalts an den Dateinamen der asset-Datei (z.B. der css-Datei), verlinkt die Datei unter dem Namen mit Hash in der HTML-Datei und sagt dem Server, dass er den Clients sagen soll, dass sie diese Datei ewig cachen dürfen. Ändert sich der Dateiinhalt, dann ändert sich auch der Dateiname, und damit wird ab sofort eine neue Datei geladen, die alte im Cache wird ignoriert.

Die Sache hat nur zwei Haken: Zum einen müssen jedes Mal, wenn ich etwas am CSS ändere, _alle_ HTML-Dateien neu generiert und komprimiert werden. Das ist aber zu verkraften, wenn man selten was am Layout ändert.

Zum anderen aber wird es mit meinem momentanen Setup eine unschöne Angelegenheit, diese Dateien mit Cache-Hashes zu generieren und gleichzeitig den Hash in den HTML-Generator zu bringen. Nicht unmöglich, aber unschön. Gegeben der Größe der CSS-Datei (<3kB unkomprimiert, <900 Byte komprimiert) könnte ich mir das auch sparen und erst umsetzen, wenn es ein Performance-Problem gibt. Das wäre der sensible Ansatz.

Aber ich kenne mich selbst zu gut. Ich werde nicht den vernünftigen Ansatz nehmen, sondern den, bei dem ich so viel wie möglich optimieren kann, ohne dass es wirklich notwendig ist. Das ist so eine Art Berufskrankheit.
