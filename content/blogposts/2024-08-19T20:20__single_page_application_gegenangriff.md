---
title: Der Gegenangriff gegen Single Page Applications
filename: single_page_application_gegenangriff
date: 2024-08-19T20:20:26+02:00
update-date:
tags: webdesign, javascript, single page application, komplexität, progressive enhancement, react
category: hoellenmaschinen
summary: Ich war nie glücklich mit Websites, die ohne Javascript nicht laufen, insbesondere Single Page Applications. Schaffen wir es, davon wegzukommen?
image:
image-alt:
language:
---

Ein altes Thema in diesem Blog, häufig von Rants begleitet, ist der übermäßige Einsatz von Javascript auf Websites. Insbesondere wenn Grundfunktionen der Website nicht mehr gehen, wenn man kein Javascript aktiviert hat. So habe ich mich zum Beispiel 2010 [auf meinem alten Blog darüber aufgeregt](/blogposts/old_1614001), dass bei einer oder mehreren Websites Grundfunktionen wie die Seitennavigation, der Login oder das Herunterladen von Dateien nur mit Javascript. Oder [die Suchfunktion](/blogposts/old_1677555) Oder, im selben Jahr, [ein Mini-Rant, ohne in die Details zu gehen](/blogposts/old_1675493). 

Als ich dann 2012 [dieses Blog in Ruby on Rails umgesetzt habe](/blogposts/1), habe ich extra darauf geachtet, _überhaupt kein Javascript zu verwenden_. Damals waren alle Rails-Tutorials Javascript-lastig. Nicht extrem, man hätte die Seite noch benutzen können, aber es war da. Ich wollte das nicht. 2020, als ich die auf Rails basierende Software [durch einen static site generator ersetzt habe](/blogposts/neustart), habe ich das beibehalten.

Dazwischen habe ich eine ganze Menge über Webentwicklung gelernt. 2015 habe ich angefangen, in Hamburg zu arbeiten. Ich habe am Backend und am Frontend gearbeitet. Und musste auch selber Javascript schreiben. Zuerst für eine Website, die für das meiste an Javascript einen Fallback hatte. Später dann an React-Anwendungen, die keinen Fallback hatten. Mea culpa. Ich war Teil des Problems geworden. Aber es war mir schlicht nicht möglich, meine Kollegen zu überzeugen, dass das eigentlich Wahnsinn ist. Obwohl mir das durchaus bewusst war, wie [dieser Blogpost von 2017](/blogposts/212) zeigt.

### Warum so viel Javascript?

Warum? Warum hat sich übermäßiges Javascript, insbesondere Single Page Apps (SPAs) so weit durchgesetzt? [Hier hat jemand letztes Jahr mal zusammengefasst, wie sich das aus seiner Perspektive entwickelt hat](https://infrequently.org/2023/02/the-market-for-lemons/). Ich kann den Artikel empfehlen, und auch ein paar andere Artikel auf der Seite, auf die ich später zurückkomme. In einer Sache bin ich aber anderer Meinung als der Autor. Der geht davon aus, dass der kontinuierliche Aufstieg von SPAs auf gezieltes Marketing wider besseren Wissens passiert ist.

Ich sehe das nicht ganz so von bösen Absichten durchdrungen. Ich bin einfach der Meinung, wir, als Entwickler, sind einer Art kollektivem Stockholm-Syndrom zum Opfer gefallen (ja, ich weiß, das Stockholm-Syndrom gibt es wahrscheinlich nicht wirklich. Aber es ist eine schöne Geschichte und eine passende Metapher). Wir stecken halt bis zum Hals im JS-Code mit allen Probleme, die das bringt, wir kommen nicht ohne weiteres mehr heraus, aber hey: schaut euch dieses Tool an, das alles viel einfacher macht!

Ich will Javascript nicht die Existenzberechtigung absprechen. Eine Landkartenseite wie OpenStreetMap? Ein Videokonferenztool? Ein Spiel? Klar, das braucht Javascript? Ein Webshop? Höchstens ein bisschen, zum Beispiel um im Checkout das Leben einfacher zu machen (und auch das sollte einen Fallback haben). Aber ein Blog? Eine Newsseite? Ein Webcomic? Eine Rezepte-Seite? All das braucht kein Javascript.

### A New Hope

Deswegen hat es mich sehr gefreut, dass ich in der letzten Monaten immer wieder Artikel gegen übermäßigen Einsatz von Javascript gefunden habe. Und in der letzten Woche [vier Artikel vom oben erwähnten Blog](https://infrequently.org/series/reckoning/), wo der Autor ordentlich mit SPAs aufräumt. Insbesondere geht es da auch um öffentliche Websites in den USA (Kalifornien, um genau zu sein), wo Leute SNAP benefits beantragen können oder so (so eine Art Sozialhilfe). Dummerweise ist die offizielle Seite dazu etwa **25 fucking Megabyte** groß, der größte Teil davon… natürlich Javascript. Auch in Kalifornien gibt es Gegenden mit nur langsamem Internetzugang, und 20 Megabyte JS sind für schwachbrüstige Geräte (Leute, die auf Sozialhilfe angewiesen sind haben i.d.R. keine high-end-Smartphones) auch ein ganzer Happen. Ergebnis: Die Seite braucht knapp 30 Sekunden um zu laden, eine vergleichbare Seite einer Drittpartei kommt auf vier Sekunden.

Mir machen diese Artikel Hoffnung, dass der Wind sich so langsam dreht. Nur ein bisschen Hoffnung, nicht viel. Vor knapp neun Jahren gab des den Talk [The Website Obesity Crisis](https://idlewords.com/talks/website_obesity.htm). Jetzt haben wir die oben genannte 25MiB-Seite und wie [dieser aktuelle Artikel über Javascript-Bloat](https://tonsky.me/blog/js-bloat/) zeigt, ist die Situation insgesamt auch nicht viel besser.

Ach ja: Bei dem Bookshop, an dem ich entwickelt habe, haben wir stets versucht, das Javascript klein zu halten. Es ist nicht wirklich gelungen, weil wir einige _sehr_ alte Browser unterstützen mussten, aber wir sind bei um die 200kiB geblieben. Es geht nicht in meinen Kopf herein, _was_ man machen muss, um auf 25MiB zu kommen. Wirklich nicht.

### Gründe gegen SPAs (oder überflüssiges Javascript auf nicht-SPAs)

Naja, jedenfalls ist das jetzt die Gelegenheit für mich, auch mal meinen Senf dazuzugeben und zu erläutern, welche Probleme man sich mit SPAs im Speziellen oder zu viel Javascript im Allgemeinen aufhalst.

Zuerst ist da natürlich die Größe, wie schon oben genannt. Trotz aller Beteuerungen, dass SPAs viel effizienter seien, weil sie für den Inhalt ja nur JSON nachladen müssen und nicht das ganze HTML immer und immer wieder, sind SPAs in der Regel so groß, dass man tausende Seiten anschauen muss, bevor sie sich lohnen.

Dann ist da die Archivierbarkeit. SPAs können zum Beispiel vom [internet archive](https://archive.org/) viel schlechter archiviert werden, weil sie dynamisch Daten nachladen. Das ist den meisten Betreibern natürlich egal. Was denen aber wichtig ist: sie wollen durch Suchmaschinen gefunden werden. Das funktioniert mittlerweile auch für SPAs, aber bei weitem nicht so gut wie für Websites, die ihren Inhalt direkt ausliefern.

#### Dinge, die der Browser schon kann und die in SPAs mühsam erneut implementiert werden müssen

Da wäre zunächst Routing: Normalerweise ist Routing einfach: Man schreibt ein `<a>`-Tag, verpasst ihm ein `href`-Attribut, fertig. Auf SPAs muss man aber zusätzlich noch:

- alle Klicks auf (interne, und nur interne) Links abfangen, damit der Browser die Seite nicht neu lädt
- die URL der Seite anpassen, ohne dass der Browser die Seite neu lädt
- den Inhalt der Seite ändern
- nach oben scrollen, damit die User den Anfang der Seite sehen und nicht die Position, wo man bisher war
- oder gegebenenfalls zum richtigen Anker auf der Seite springen, falls die URL ein Fragment (`#`) hat
- wenn die Seite neu geladen wird, in die URL schauen um den richtigen Inhalt anzuzeigen

Bevor jemand damit kommt, dass man diese Sachen sonst serverseitig machen muss: Erstens muss man die meisten davon nicht serverseitig machen, weil sie clientseitig laufen und das normalerweise der Browser erledigt. Zweitens muss man die Teile, die man doch serverseitig machen muss (unterschiedlichen Inhalt über unterschiedliche URL-Pfade ausliefern) trotzdem machen, weil man bei SPAs üblicherweise einen API-Server im Hintergrund hat, von dem die eigentlichen Inhalte kommen.

Das ist natürlich angenommen, man möchte es den Usern ermöglichen, eine normale Website-Erfahrung zu haben, wo u.a. auch die Navigation mit den zurück/vorwärts-Buttons des Browsers funktioniert, man zu Abschnitten innerhalb der Seite springen kann und Links auf einzelne Seiten hat, die man auch mit anderen teilen kann. Wenn man sich diese Mühe nicht macht, haben die User eine erheblich eingeschränkte Version einer normalen Website. In einem Fall haben mir Kollegen in einem anderen Team auch mitgeteilt, sie könnten keine direkten Links auf Seiten machen, weil sie dafür den Webserver so einstellen müssten, dass er für verschiedene URLs genau denselben Inhalt ausliefert. Nein, wirklich!

Besonders aufgefallen sind mir auch, wie schon oben verlinkt [Links, die nicht an `<a>`-Tags hängen](/blogposts/212).

Andere Dinge, die man nachimplementieren muss:

- Formulare (unterschätzt das nicht, da gibt es viele Fallstricke)
- Requests an den Server im Allgemeinen
- eventuell eine Ladeanzeige („spinner“)

Diese Liste ist nicht vollständig, es gibt noch mehr. Routing ist aber definitiv der wichtigste und umständlichste Punkt.

#### Komplexität

Komplexitätssucht ist eine Art Berufskrankheit unter Softwareentwicklern. Um [Rich Hickey](https://www.red-gate.com/simple-talk/opinion/geek-of-the-week/rich-hickey-geek-of-the-week/) zu zitieren:

> I think programmers have become inured to incidental complexity… when they encounter complexity, they consider it a challenge to overcome, rather than an obstacle to remove. Overcoming complexity isn’t work, it’s waste.

Manche Komplexität lässt sich nicht entfernen. Manche Komplexität würde sich entfernen lassen, ist es aber Wert, mit ihr umzugehen (ich, als Rust-Fan, sehe Rust in der zweiten Kategorie). Aber vieles an Komplexität, die wir uns antun, ist unnütz, nervig und schädlich.

SPAs sind ein schönes Beispiel dafür. Der ganze Krams, den ich oben über das Reimplementieren von Browserfunktionalitäten geschrieben habe? Unnütze Komplexität. Wo kommt in SPAs sonst noch vermeidbare Komplexität her? Hier nur eine kurze Auswahl:

- manuell geschriebene HTTP-Requests und das Parsen der Ergebnisse (und sanity-checks, ob die Antworten das enthalten was man erwartet. Kann man auch lassen, aber dann hat man halt später Spaß mit komischen Fehlern wenn mal etwas nicht passt)
- Behandeln von Sonderfällen für ältere Browser (polyfills)
- Abhängigkeiten auf Bibliotheken von Dritten, insbesondere Frameworks wie React (jede Abhängigkeit bedeutet einen erhöhten Wartungsaufwand. Und man sollte besser alle Updates zeitnah mitnehmen, oder man kommt irgendwann an den Punkt, wo man ein neues Update braucht, es aber nicht installieren kann, weil man vorher mehrere Jahre Update nachholen muss)
- Fehlerbehandlung

Dazu kommt Komplexität bei den Entwicklertools. Da wären zunächst die Entwicklertools selber. Man braucht

- NPM (für die oben genannten Abhängigkeiten)
- einen Linter (Javascript ist so furchtbar, dass das, im Gegensatz zu anderen Sprachen, mehr als nur ein nice-to-have ist)
- etwas wie Webpack, um das Ergebnis irgendwie zusammenzufügen
- einen Typescript-Compiler, wenn man Typsecript verwendet (was zu empfehlen ist, weil man damit Fehler vermeiden kann, die in so einer komplexen Umgebung schnell passieren)
- etwas wie Babel, um Polyfills für veraltete Browser einzufügen, die man unbedingt noch supporten muss

Es ist ein bisschen so, als ob man beim Hausbau beim Fundament gepfuscht hat, das Haus sich schon während des Baus zur Seite neigt, man aber einfach weiter baut und drumherum jede Menge Gerüste und Stützen hinstellt, damit das Haus nicht zusammenbricht, anstatt von vornherein für ein vernünftiges Fundament zu sorgen. So ein bisschen wie der Schiefe Turm von Pisa, wo man noch Jahrhunderte nach dem Bau mit den Folgen zu kämpfen hat. Nur dass eine Webapp nicht zu einer beliebten Touristenattraktion wird.

Was sind die Nachteile von Komplexität?

- Wartung: mehr Komplexität macht Wartung und Weiterentwicklung schwieriger und damit teurer
- Stabilität: mehr Komplexität macht Software anfälliger für Fehler, jeder Fehler kann größere Auswirkungen haben. Bonuspunkte: sobald auch nur eine Javascript-Datei nicht geladen werden kann, blockiert oft die ganze Seite
- Security: was für die Stabilität gilt, gilt auch für die Sicherheit: je unübersichtlicher, desto eher schleicht sich ein Fehler ein, der ernsthafte Konsequenzen hat
- Performance: Mehr Komplexität bedeutet mehr Code, d.h. es muss mehr heruntergeladen werden und auch die Ausführung ist langsamer

#### Vermeintliche Lösungen

Eine beliebte Lösung für ein paar der Probleme, insbesondere das mit der Archivierbarkeit und der Suchmaschinenfreundlichkeit: Wir rendern die Seite serverseitig _und_ lassen sie dann clientseitig als SPA laufen. Das geht schon fast in die richtige Richtung, wird aber häufig durch zwei Probleme heruntergezogen: Erstens kann man die Seite dann ohne Javascript trotzdem nicht nutzen (bzw. muss man trotzdem eine ordentliche Menge an Javascript laden, was den vermeintlichen Größenvorteil von SPAs endgültig zunichte macht), zweitens ist das für die Entwickler eine Menge Arbeit, weil sie die Seite gleich zwei Mal entwickeln müssen.

Das Ergebnis ist, dass so etwas meist eher halbherzig gemacht und für manche Seiten auf der Website komplett vergessen wird. Die Lösung dafür wiederum ist etwas, was die Marketingabteilung „isomorphic Javascript“ genannt hat. Da soll man dann denselben Code Server- und clientseitig laufen lassen können. Der Nachteil: _noch mehr_ Komplexität.

Mal ganz abgesehen davon, dass Javascript eine furchtbare Sprache ist, die ich nicht auch noch serverseitig sehen will, gibt es einen ganzen Haufen von Dingen, die auf dem Server grundsätzlich anders laufen als im Client. Ich habe einmal mit Nuxt gearbeitet. Das ist quasi eine isomorphic Javascript-Version von vue. Der ganze Code bestand aus Ausnahmen, wo man dann doch darauf achten musste, wo man serverseitig und wo man clientseitig arbeitete. Man musste überall höllisch aufpassen, dass man nicht aus Versehen irgendwelche serverseitigen Secrets leaked. Das ist eine Sicherheitslücke, die nur darauf wartet, implementiert zu werden.

### Alternativen

Denken wir mal konstruktiv: Was _sollten_ wir denn machen? Erst einmal: Keine SPAs, macht klassische Websites wo immer möglich. Dann gibt es da das Konzept des „unobtrusive Javascript“: Javascript, dass Funktionen ergänzt bzw. die Benutzbarkeit erhöht, wobei es aber trotzdem noch möglich sein muss, die Seite ohne Javascript zu verwenden (dann gehen halt schlimmstenfalls ein paar Komfortfunktionen verloren).

Verallgemeinert ist das im Konzept [Progressive Enhancement](https://de.wikipedia.org/wiki/Progressive_Verbesserung). Das läuft auf dasselbe hinaus, bezieht aber auch das Styling (CSS) mit ein. Die Idee: Eine Seite sollte komplett ohne Styling schon benutzbar sein (z.B. auch in einem textbasierten Browser wie [Links](https://de.wikipedia.org/wiki/Links_(Browser))). Das Styling kommt obendrauf, um die Seite schöner und vielleicht auch ein bisschen übersichtlicher bzw. lesbarer zu machen. Erst dann kommt das Javascript obendrauf, für Komfortfunktionen.

Dazu gibt es [einen schönen Artikel der britischen Regierung](https://www.gov.uk/service-manual/technology/using-progressive-enhancement). Als Website-Beispiel kann ich zum Beispiel dieses Blog hier nennen. Aber es gibt auch wichtige Websites, die das richtig machen: Wie ich [letzten Monat festgestellt habe](/blogposts/web_polychromie) funktioniert zumindest das Lesen von Wikipedia-Artikeln sehr gut auch ohne CSS. Als Negativbeispiel kann man leider fast jede größere Website nennen.

### Ausklang

So, das war wieder ein Artikel der deutlich länger geworden ist als ursprünglich geplant. Also spare ich mir den anderen, nur entfernt relevante Kram, den ich sonst noch im Kopf habe. Was können wir also mitnehmen?

- SPAs sind für die meisten Anwendungsfälle eine Scheißidee
- macht eure Websites nach dem Progressive Enhancement-Prinzip, nutzt Javascript nur, um Komfortfunktionen hinzuzufügen oder dann, wenn ihr etwas machen wollt, was _wirklich_ nicht ohne Javascript geht
- hoffen wir mal, dass die Tage der SPAs gezählt sind (aber ich glaube nicht wirklich daran)
