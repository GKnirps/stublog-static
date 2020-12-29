---
title: Wahlcomputer
filename: wahlcomputer
date: 2020-12-27T20:47:54+01:00
update-date: 2020-12-29T20:44:00+01:00
tags: wahlcomputer, rc3, wahlen, rant
category: hoellenmaschinen
summary: Wahlcomputer und warum man sie nicht verwenden sollte
---

Seit Anfang November sind Wahlcomputer ja wieder groß im Gespräch, zumindest in den USA. Dort verbreiten Trump und seine Minions ja Verschwörungstheorien über gewaltigen Wahlbetrug, [unter](https://www.politifact.com/factchecks/2020/nov/17/youtube-videos/no-us-military-did-not-raid-election-software-comp/) [anderem](https://www.politifact.com/factchecks/2020/nov/13/facebook-posts/no-evidence-dominion-voting-systems-caused-widespr/) [einen](https://www.politifact.com/factchecks/2020/nov/12/donald-trump/trumps-tweet-about-27-million-deleted-votes-basele/) [Haufen](https://www.politifact.com/factchecks/2020/nov/09/blog-posting/trump-supporters-falsely-tie-nancy-pelosi-broader-/) [widerlegte](https://www.politifact.com/factchecks/2020/nov/18/ted-nugent/inaccurate-early-vote-count-onmichigan-county-was-/) [Vorwürfe](https://www.politifact.com/factchecks/2020/nov/10/pamela-geller/debunking-hammer-and-scorecard-election-fraud-cons/) [über](https://www.politifact.com/factchecks/2020/nov/06/facebook-posts/did-president-trump-issue-secret-watermarks-ballot/) [Wahlcomputer](https://www.politifact.com/factchecks/2020/nov/19/sidney-powell/trump-lawyer-falsely-claims-voting-technology-comp/).

Während diese Vorwürfe widerlegt sind und zu Trumps Kampagne gehören, die amerikanischen Präsidentenwahlen zu delegitimieren, so ändert das nichts daran, dass es grundsätzliche Probleme beim Einsatz von Wahlcomputern gibt. Mehr noch, Vorwürfe dieser Art sind genau eins der Probleme, die beim Einsatz von Wahlcomputern entstehen.

[Dieses Paper hier](https://people.csail.mit.edu/rivest/pubs/PSNR20.pdf) beschreibt, warum Blockchain-Technologie die Probleme mit Wahlcomputern [nur noch schlimmer macht](https://xkcd.com/2030/). Darum geht es mir hier aber nicht. Dort werden nämlich auch noch einmal die wichtigsten Ansprüche an eine demokratische Wahl aufgelistet:

1. Wahlen müssen nachprüfbar sein und nachgeprüft werden. Es muss nachweisbar sein, dass die Anzahl der gezählten Stimmen tatsächlich den abgegeben Stimmen entsprechen.
2. Wahlen müssen geheim sein.
3. Wahlen müssen softwareunabhängig sein. Ein Fehler oder eine Manipulation in einer Wahlsoftware darf nicht zu einer unbemerkten Veränderung des Ergebnisses führen.
4. Wähler müssen verifizieren können, dass tatsächlich die Stimme für das gezählt wird, was sie wählen.

Da sind noch ein paar mehr, aber mir reichen erst einmal diese. Die [Begründung](https://de.wikipedia.org/wiki/Wahlger%C3%A4t#Das_Bundesverfassungsgericht_in_Sachen_%E2%80%9EWahlcomputer%E2%80%9C_(2009)), warum in Deutschland keine Wahlcomputer zugelassen sind, ist insbesondere der vierte Punkt:

> Der Zweite Senat hat entschieden, dass der Einsatz elektronischer Wahlgeräte voraussetzt, dass die wesentlichen Schritte der Wahlhandlung und der Ergebnisermittlung vom Bürger zuverlässig und ohne besondere Sachkenntnis überprüft werden können. Dies ergibt sich aus dem Grundsatz der Öffentlichkeit der Wahl (Art. 38 in Verbindung mit Art. 20 Abs. 1 und Abs. 2 GG), der gebietet, dass alle wesentlichen Schritte der Wahl öffentlicher Überprüfbarkeit unterliegen, soweit nicht andere verfassungsrechtliche Belange eine Ausnahme rechtfertigen.

Das Ganze ist recht kompliziert, läuft aber am Ende darauf hinaus, dass man vielleicht einzelne dieser Punkte mit einem Wahlcomputer umsetzen kann, das verhindert dann aber die Umsetzung der anderen Punkte. So kann man zum Beispiel digitale Signaturen verwenden, um zu verhindern, dass die Ergebnisse manipuliert werden. Das führt aber zu einer Menge komplizierter Kryptographie, die ein Laie nicht verstehen kann. Dann kann man natürlich jede Person ihre Stimme signieren lassen, so kann jede Stimme nur ein Mal abgegeben werden und die Person kann hinterher sogar nachschauen, ob die Stimme auch angekommen ist. Dann war die Stimmabgabe aber wieder nicht geheim. Kurz: Es ist eine Zwickmühle.

Ein anderer wichtiger Grund, warum keine Wahlcomputer benutzt werden sollten ist die Manipulierbarkeit. Wenn man Papierwahlen manipulieren will, muss man eine Menge Aufwand treiben, um in auch nur einem Wahllokal eine nennenswerte Anzahl von Stimmen zu manipulieren. Und wenn das Ergebnis sehr komisch ausfällt, wird vielleicht genauer hingeschaut und man [riskiert mehrjährige Haftstrafen](https://de.wikipedia.org/wiki/Wahlf%C3%A4lschung#Rechtslage_in_Deutschland).

Um das Ergebnis der Wahl wirklich zu beeinflussen muss man aber in vielen Wahllokalen manipulieren. Das ist noch aufwändiger. Man muss vermutlich pro Wahllokal nicht unglaublich viele Stimmen manipulieren (was auch den Vorteil hat, dass das Ergebnis nicht so sehr aus dem Rahmen fällt, dass es sofort auffällt), aber durch die Anzahl der Manipulationen ist die Wahrscheinlichkeit hoch, dass es _irgendwo_ auffällt.

Mit Wahlcomputern ist das einfacher: Wenn man einen Wahlcomputer manipulieren kann, kann man auch viele Wahlcomputer manipulieren. Mal werden sie nicht sicher gelagert, mal sind sie so unsicher, dass man in einer Minute alleine in der Wahlkabine Malware auf die Maschine spielen kann (u.a. dazu gibt es auch eine schöne [Folge Last Week Tonight](https://www.youtube.com/watch?v=svEuG_ekNT0).

Natürlich behaupten die Hersteller solcher Maschinen, dass sie sicher sind. Die offiziellen Stellen behaupten dann sowieso, dass niemand an die gelagerten Wahlcomputer drankommt, et cetera, et cetera. In [diesem Artikel](https://www.nytimes.com/2020/11/16/business/election-security-letter-trump.html), in dem es hauptsächlich darum geht, dem Wahlbetrugsverschwörungsmythos von Donald Trump zu entkräften, wird weiter unten auch folgendes erwähnt:

> The Senate majority leader, Mitch McConnell, killed three election security measures last year, arguing that they were unnecessary and “partisan.”

Mitch McConnel, der Typ (oder einer der Typen), der sehr lange gezögert hat um zuzugeben, dass Biden die Wahl gewonnen hat, um es sich nicht mit Trump zu verscherzen.

Es ist ja nicht so, als seien Sicherheitsprobleme mit Wahlcomputern neu sind. Im Prinzip wird damit umgegangen wie mit allen IT-Sicherheitslücken: Erst leugnet man, dass es sie gibt (und das Sicherheit sehr ernst genommen wird), dann leugnet man, dass sie ausnutzbar sind, dann schiebt man alle Schuld von sich, wenn sie gefunden und ausgenutzt werden. Denn dann ist man ja Opfer eines bösen Cybercyberkriminellen. Sätze wie „Wir nehmen Sicherheit sehr ernst“ (oder, off-topic, „Wir nehmen Datenschutz sehr ernst“) sind in der Regel nicht das Papier wert, auf dem sie gedruckt sind (oder den Srom den es kostet, sie auf dem Bildschirm anzuzeigen.

Hier in Deutschland ist ja der Einsatz von Wahlcomputern für staatliche Wahlen nicht zugelassen. Der Einsatz von Software zum Verarbeiten oder Zählen der Stimmen teilweise aber schon. So gab es zum Beispiel 2017 eine [Untersuchung der Software „PC-Wahl“](https://media.ccc.de/v/34c3-9247-der_pc-wahl-hack) durch den Chaos Computer Club.

Heute gab es wieder einen schönen Vortrag mit dem Titel „Hacking german elections“. Ich konnte den auf media.ccc.de noch nicht finden, aber wenn ich dran denke, füge ich den Link hier später noch ein. Kurz zusammengefasst: Da geht es um Software, die in Bayern hilft, Wahlzettel zu zählen (was sehr aufwändig sein kann). Der Hersteller sagt, dass sie Sicherheit sehr ernst nehmen. Die Sicherheit der Software wurde bis dato nie überprüft. Es gibt keine gesetzlichen Vorgaben zur Sicherheit. Die Software wird Infrastrukturbedingt auf _irgendwelchen_, nicht notwendigerweise sicheren Computern ausgeführt (zum Beispiel auf den Computern der Schule, in der das Wahllokal ist). Im Fall der Vortragenden war der einzige Grund, warum da keine Windows XP-Maschinen verwendet wurden, dass die Software eine 64-Bit-Architektur braucht.

Die erfassten Stimmdaten sind nicht mit z.B. Signaturen gegen nachträgliche Manipulation geschützt.

Die Software selber hat an allen Ecken und Enden Sicherheitslücken, angefangen dabei, dass man keine Admin-Rechte braucht, um Admin-Operationen durchzuführen (Kenntnis der richtigen URL reicht), über mögliche CSRF-Attacken bis hin zu Remote Code Execution, für die man nur im selben Netzwerk wie der Computer sein muss.

Der Hersteller hat sich laut den Vortragenden immerhin vorbildlich verhalten, als die Lücken gemeldet wurden. Trotzdem hätte diese Software imho nie eingesetzt werden dürfen, bei so vielen Anfängerfehlern, die damit gemacht werden.

Und das ist nur ein Beispiel warum man einem Softwarehersteller nie trauen darf, wenn er beteuert, Sicherheit sehr ernst zu nehmen.

### Update

Der Link für den [Vortrag „Hacking German Elections“](https://media.ccc.de/v/rc3-11440-hacking_german_elections) ist da.
