---
title: Import und Export
filename: import_export
date: 2020-10-26T21:16:47+01:00
update-date:
tags: html5, xhtml, parser
category: meta
summary: Ich habe nach acht Jahren endlich Einträge aus meinem alten Blog importiert.
---

Über vier Jahre (2008 bis Oktober 2011) hatte ich [mein erstes Blog](https://stu.blogger.de). Dann habe ich mir eine eigene Blogsoftware gebastelt und mir gedacht: „Bei Gelegenheit importiere ich mal die Einträge aus dem alten Blog in das neue Blog“.

Acht Jahre später habe ich die Blogsoftware [ersetzt](/blogposts/neustart). In den ganzen acht Jahren habe ich den Import immer vor mir hergeschoben. Ein einziges Mal habe ich angefangen, dann aber die Lust verloren, weil die HTML-Struktur von blogger.de das Heraussuchen der Einträge so unschön macht.

Jetzt habe ich endlich die alten Blogeinträge importiert. Das heißt natürlich auch, dass mit meinen Archivseiten genau das passiert, was ich vermeiden wollte: Die Einträge auf den Archivseiten werden verschoben. Was vorher auf Seite 1 war ist danach auf Seite 36. Das wird aber in Zukunft nicht mehr passieren, weil ich keine neuen Einträge in der Vergangenheit hinzufügen werde.

Jeder alte Blogpost hat auch einen Link zum Original. Dort kann man auch die Kommentare dazu lesen, die ich aus mehreren Gründen nicht kopiert habe. Zum einen wäre das bei der gegebenen HTML-Struktur umständlich gewesen, zum anderen weiß ich nicht, wie es da mit dem Urheberrecht steht.

### Technische Umsetzung

Die grundlegende technische Idee ist einfach: Ich schnappe mir die Daten vom alten Blog, transformiere sie in ein Format, das dem neuen Blog schmeckt und schiebe sie ins neue Blog.

#### Der Crawler

Praktisch gesehen fangen die Probleme natürlich schon ganz am Anfang an. Der Server des alten Blogs steht nicht unter meiner Kontrolle. Es gibt (soweit ich herausfinden konnte) keine Exportschnittstelle. Aber das ist ja eigentlich kein Problem. Ich schreibe seit Jahren immer mal wieder kleine Scripte, um irgendwelche Websites zu crawlen, da kann ich das hier ja auch machen.

So weit, so gut. Ich schreibe den crawler nicht in rust, sondern in Go, weil ich für meinen Job gerade sowieso ein bisschen Übung in Go gebrauchen könnte. Die alte Blogseite ist nicht utf-8 (oder überhaupt unicode)-codiert, sondern ISO-8859-1. Also erst einmal umcodieren. Dann parsen. Ist natürlich kein HTML5, sondern XHTML. Auch kein Problem, der XML-Parser von Go kommt damit klar.

Als nächstes muss man im HTML nach bestimmten Sachen suchen. Der „older stories“-Link ist recht einfach zu finden. Was weniger einfach zu finden sind, sind Titel uns insbesondere Text des Eintrags. Anstatt jeden Eintrag in einen eigenen Container zu stecken (in meinem neuen Blog ein schickes HTML5-`<article>`-Tag), wurde der Inhalt der Einträge einfach hintereinander gehängt. Die Überschriften sind formal nicht einmal Überschriften.

Das macht es natürlich kompliziert zu unterscheiden, welche Überschrift und welches Datum zu welchem Text gehört (umso mehr, da das `<div>` des eigentlichen Texts überhaupt keine Klasse hat). Glücklicherweise ist der Permalink zu einem Eintrag recht einfach zu finden. Also erst einmal alle Permalinks sammeln.

In der zweiten Phase geht der Crawler dann alle Permalinks durch. Jetzt sind Überschrift und Text ziemlich einfach zu finden, Datum und Zeit sind unglücklicherweise auf zwei Tags verteilt, die Kategorie ist überhaupt nicht vorhanden. Gerade als ich den Datumsparser fertig habe stelle ich auch noch fest, dass ich es einfacher hätte haben können, denn wie Datum und Zeit angezeigt werden sollen ist konfigurierbar.

Also habe ich jetzt: die Überschrift, Erstellungsdatum und den Inhalt. Fehlt noch die Kategorie. Die wird dummerweise nicht angezeigt. Außerdem ist der Inhalt nicht besonders schön. Keine `<p>`-Tags für Absätze, stattdessen harte Zeilenumbrüche wohin man blickt. Schöner wäre, wenn ich den Rohtext kriegen könnte, den ich mal dort eingetragen habe.

Glücklicherweise ist es ja mein Blog, also kann ich mich einloggen und die „bearbeiten“-Seite aufrufen. Dort findet mein Crawler jetzt die Kategorie und den Rohtext. Zumindest theoretisch.

Praktisch kommt mir da nämlich in die Quere, dass, sobald man eingelogged ist, zusätzliches HTML gerendert wird, das nicht XML-konform ist (ein `<` wird in einem Attribut verwendet). Die meisten Browser sehen über solche Fehler hinweg. Der Go-XML-Parser nicht, nicht mal, wenn ich ihn ganz lieb bitte.

Also erst einmal die komplette Seite abgespeichert. Sobald der Krams erst einmal heruntergeladen ist, kann ich offline damit herumbasteln.

#### Erstellen der Blogposts

Das Format meiner Blogpost-Dateien ist recht simpel: ein kurzer Header mit Name-Wert-Paaren und so Sachen wie Titel, Datum, Kategorie, Tags, usw., danach der Inhalt als Markdown.

Der meiste Header-Kram ist recht einfach zu transformieren. Datum habe ich. Tags gibt es keine. Titel war in einer kleinen Zahl der Fälle nicht vorhanden, da habe ich mir einen neuen Titel ausgedacht.

Bei den Kategorien musste ich tricksen: Ich habe im neuen Blog weniger Kategorien und wollte nicht alle Kategorien des alten Blogs übernehmen. Manche Kategorien, wie [Ganz normaler Wahnsinn](/categories/ganz_normaler_wahnsinn) können einfach 1:1 gemapped werden.

Manche Kategorien, wie „Saftladen“, also alles, was mit ÖPNV und deutscher Bahn zu tun hat, kann ich guten Gewissens komplett unter [Rant](/categories/rant) einsortieren. Da es im alten Blog keine Tags gab, kommt hier ein Tag „[öpnv](/tags/öpnv)“ dran, um die Sache runder zu machen.

Manche Kategorien, von denen ich ausgehe, dass ich sie noch brauchen könnte, z.B. [Bücher](/categories/buecher) habe ich neu hinzugefügt.

Eine Sonderstellung nimmt die Kategorie [TVTropes](/categories/tvtropes) ein. Die hatte ich 2012 mit dem neuen Blog angelegt, aber nie verwendet. Einige Posts aus dem alten Blog gehören aber in diese Kategorie, also wird sie endlich genutzt!

Der größte Block ist der Inhalt. Der enthält zwei Sachen, die ich transformieren musste: HTML-Tags und Macros der blogger.de-Software, die Dinge wie Bilder und Dateien, die man dort hochgeladen hat, einbinden. Das war schon ein bisschen Arbeit. Die Dateien vom alten Blog habe ich in diesem Zug auch übernommen.

Trotz all den automatischen Konvertierungen musste immer noch ein Feinschliff gemacht werden, weil man einige Sachen einfach nicht automatisch entscheiden konnte. Also bin ich noch einmal durch etwa 540 Einträge durchgegangen. Hatte den Vorteil, dass ich mir noch einmal durchlesen konnte, was ich damals eigentlich geschrieben habe. Das ist ja alles acht bis zwölf Jahre her.

Einen [Eintrag](https://stu.blogger.de/stories/2149890/) habe ich ausgelassen. Dieser ist nur ein Verweis auf das neue Blog (also dieses hier).

### Was habe ich früher so geschrieben?

Was als erstes auffällt ist nicht was, sondern wie viel ich geschrieben habe. In den knapp fünf Jahren vor meiner eigenen Blogsoftware waren es etwa 540 Einträge. In den acht Jahren danach nur 217, wovon alleine 56 auf mein [Kochprojekt 2014](/categories/kochprojekt) fallen. Mehr noch, seit ich berufstätig bin hat sich meine Blogaktivität stark gesenkt (83 Einträge in knapp sechs Jahren). Oder ich vegetiere hier in Hamburg einfach nur so vor mich hin und habe nichts interessantes zu berichten.

Also, *worüber* habe ich denn nun geschrieben? Häufig über Kleinkrams, den man ohne Kontext nicht versteht. Es ist aber schön, mal wieder Sachen aus der Uni- und der Abi-Zeit zu lesen. Besonders ganz am Anfang kam viel zusammenhangloses Zeug zu meinen gentoo-Linux-Erfahrungen. Ich habe aber auch einige durchaus interessante Dinge geschrieben, sowie einige, bei denen ich heute anderer Ansicht bin.

Vielleicht mal ein paar Beispiele:

- [hier](/blogposts/old_1343581) habe ich mich über, wie es mir damals schien, typographische Allüren eines Freundes beklagt. Heute bin ich anderer Ansicht und muss zugeben, dass er Recht hatte.
- der [erste Aprilscherz](/blogposts/old_1088679) des Blogs ist [hilarious in hindsight](https://tvtropes.org/pmwiki/pmwiki.php/Main/HilariousInHindsight) wenn man bedenkt, dass Microsofts [azure-cloud Linux unterstützt](https://azure.microsoft.com/de-de/overview/linux-on-azure/)
- In aktuellen Pandemiezeiten ist es ganz interessant zu wissen, dass Microsoft schon 2009 auf der Cebit eine [Flasche Desinfektionsmittel](/blogposts/old_1354466) in einem Gewinnspiel verschenkt hat. Rückblickend nicht einmal so abwegig, weil Bill Gates schon damals viel Geld in gemeinnützige Organisationen zur Weltgesundheit steckte.
- ich habe mich damals viel damit herumgeärgert, [64Bit-Java-Plugins](/blogposts/old_1386076) für den Firefox unter Linux zu kriegen. Die Technik war damals schon am aussterben, wurde aber noch hier und da verwendet. Schlimmer war es mit adobe flash, was man damals besonders noch für Youtube brauchte.
- Übersetzungsmaschinen sind seit damals besser geworden. Das Beispiel, dass ich im [Eintrag zu translation party](/blogposts/old_2049282) gebracht habe, kommt jetzt unverändert wieder heraus.
- ich habe mir damals meinen [ersten Raspberry Pi](/blogposts/old_2086596) bestellt, nachdem ein Kommilitone mir seinen gezeigt hatte. Zu der Zeit nahm das gerade richtig Fahrt auf und ich hatte Glück, meinen halbwegs schnell zu bekommen (in etwa drei Wochen), während andere Kommilitonen kurz später Monate warten mussten.
- [hier](/blogposts/old_1162270) habe ich mich über die Nutzlosigkeit des großen ẞ ausgelassen. Die Energie hätte ich mir sparen können. Wobei es [wirklich wichtigere Dinge gibt](https://modelviewculture.com/pieces/i-can-text-you-a-pile-of-poo-but-i-cant-write-my-name), die in den Unicode-Standard sollten. Emojis sind ja auch ganz nett, aber nicht alles ☺
- [Export von Überwachungstechnologier](/blogposts/old_1954013) an totalitäre Regimes ist immer noch [aktuell](https://www.tagesschau.de/investigativ/ndr/spaehsoftware-finfisher-101.html)

Was habe ich sonst noch so geschrieben? Rants über Netzpolitik, vieles davon leider auch heute noch ein Thema. Das gleiche gilt für freie Software und Gerätehoheit (auch wenn ich letzteren Begriff damals noch nicht kannte). Rants über den ÖPNV kommen in letzter Zeit nicht mehr so häufig, weil dieser zum einen in Hamburg besser funktioniert und ich ihn andererseits dank Fahrrad und kurzer Entfernungen nicht so häufig in Anspruch nehme.

Ansonsten habe ich das Blog oft als Ventil für Frust mit Hard- und Software benutzt.

Was mir auch aufgefallen ist: viele der Links in den alten Einträgen sind tot oder werden mittlerweile umgeleitet. Das ist schade. Ich selbst habe mir Mühe gegeben, alle wichtigen URLs meines Blogs zu erhalten (und habe das bei den Tags nicht geschafft). Viele andere Seiten sind aber verschwunden oder haben ihre Seite umstrukturiert, so dass man unter den alten URLs nichts mehr findet. Dazu kann ich mal den Artikel [Cool URIs don't change](https://www.w3.org/Provider/Style/URI.html) von Tim Berners-Lee empfehlen.

Und da ich jetzt schon wieder knapp zwei Stunden an einem Blogpost gesessen habe, muss ich jetzt ins Bett.
