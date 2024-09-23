---
title: Content-Security-Policy
filename: content_security_policy
date: 2024-09-23T19:52:35+02:00
update-date:
tags: content-security-policy, security, website, http, it-sicherheit, xss
category: hoellenmaschinen
summary: Den Content-Security-Header gibt es jetzt seit über einem Jahrzehnt. Er wird aber fast nie so angewendet, dass er gegen cross-site-scripting hilft.
image:
image-alt:
language:
---

Den [Content-Security-Policy-HTTP-Header](https://content-security-policy.com/) gibt es jetzt schon seit über einer Dekade. Was macht dieser Header? Er bietet eine Möglichkeit, für Betreiber von Webservern bestimmte Sicherheitsregeln einzuführen, die dann vom Browser (aka User-Agent) der Benutzer umgesetzt werden. Das ist eigentlich ganz schön, denn man muss sich als Webseitenbetreiber sowieso darauf verlassen, dass der Browser im Interesse des Benutzers arbeitet.

Die Kernfunktion der Content-Secuity-Policy (kurz „CSP“) ist hierbei zu kontrollieren, von welchen Quellen die Webseit Resourcen welcher Art beziehen darf. So kann man zum Beispiel angeben, dass CSS-Dateien nur von der Domain der Website selbst geladen werden dürfen, dass Bilder von der Website selbst oder von einem bestimmten CDN-Server kommen können und so weiter. Es gibt noch ein paar andere wichtige Funktionen, aber fokussieren wir uns erst einmal auf die Resourcen.

### Exkurs: XSS-Attacken

Die gefährlichste Resourcen sind Scripte. Javascript ist eine Turing-Vollständige Progammiersprache und wenn eine Angreiferin es schafft, ein eigenes Script auf einer fremden Seite auszuführen, kann sie ziemlich viel anrichten, zum Beispiel Aktionen im Namen des Benutzers ausführen. Auch hier gibt es Sicherheitsmaßnahmen in Browsern, um den Schaden zu begrenzen, gefährlich ist es aber trotzdem. Solche Attacken nennt man [Cross-Site-Scripting](https://owasp.org/www-community/attacks/xss/), oder XSS (fragt nicht, wo das X herkommt, ich habe keine Ahnung. Vermutlich ist es da, um Verwechslungen mit [CSS](https://de.wikipedia.org/wiki/Cascading_Style_Sheets) zu verhindern).

XSS-Lücken passieren, wenn es Benutzern ermöglicht wird, eigenen Inhalt ungefiltert oder schlecht gefiltert auf einer fremden Website unterzubringen. Und dass Benutzer eigenen Inhalt auf fremden Websites unterbringen ist seit Jahrzehnten im Internet gang und gäbe. Nehmen wir als Beispiel mal einen Kommentar, den ich in einem Blog über eine Kommentarfunktion hinterlasse: Wenn die Seite meinen Kommentartext 1:1 übernimmt, und in meinem Kommentartext ist HTML-Code, dann wird dieser HTML-Code für alle gerendert, die sich diesen Kommentar durchlesen (oder auch nur die Seite laden, auf der er lesbar ist).

Wenn der HTML-Code jetzt ein Script enthält, wird auch dieser Code ausgeführt. Schon das Anzeigen beliebigen HTML-COdes in Kommentarn will man verhindern, aber ein Script ist, wie oben erklärt, eine Katastrophe. Deswegen ist es Standard, und wird auch von allen gängigen Frameworks und HTML-Templates unterstützt, die Benutzereingaben eben _nicht_ 1:1 in die Seite zu schreiben, sondern spezielle HTML-Zeichen vorher zu *escapen*, also sie durch Platzhalter zu ersetzen, die dann nur für die Anzeige als die Zeichen dargestellt werden, die der Benutzer eigentlich eingegeben hat.

### Wie man mit einer CSP XSS-Attacken verhindert

Escapen mag Standard sein, aber manchmal gibt es Bugs oder man stellt sich blöd an und man hat doch eine XSS-Lücke. Dafür (unter anderem) wurde der CSP-Header erfunden. Dort kann man wie gesagt angeben, woher Scripte stammen dürfen. Es gibt viele Optionen, das zu konfigurieren, doch kurz gesagt: Man gibt eine Liste von Domains an. Wenn die Scripte nicht auf dieser Liste stehen, werden sie nicht ausgeführt. Darunter fällt auch, dass sie nicht im HTML-Code direkt stehen dürfen. Will man das erlauben, muss man eine spezielle Anweisung `'unsafe-inline'` hinzufügen. Will man auch erlauben, dass Scripte nach eigenem Gutdünken Text als Script ausführen sollen, muss man die Anweisung `'unsafe-eval'` hinzufügen.

Die Namen dieser Spezialanweisung sollten aufhorchen lassen: dort steht ganz klar drin: Das ist nicht sicher. Wenn man diese Anweisungen in die CSP macht, hebelt man damit ihre Schutzwirkung fast komplett auf.

### Meine Erfahrung mit CSPs

Ich habe vor etwa sieben Jahren in meiner damaligen Firma einen kleinen Vortrag über den CSP-Header gehalten und auch [einen kleinen Webserver zum Demonstrieren](https://github.com/GKnirps/cspdemo) geschrieben. Ich habe das damals gemacht, weil ich Aufmerksamkeit auf diesen damals noch nicht ganz alten Header lenken wollte, der eine zusätzliche Absicherung gegen verschiedene Attacken bringen kann.

Ich habe auch versucht, diesen Header in die Projekte einzubringen, in denen ich tätig war. Das war schwierig. Denn inline-Javascript wird _oft_ verwendet. Es gibt zwar Wege, auch spezifisches Inline-Javascript zuzulassen, das ist aber bei bestehendem Code fast so aufwändig, wie das Javascript einfach in separat geladene Dateien auszulagern. Immerhin habe ich es geschafft, in einem Projekt einen CSP-Header in den Checkout einzubauen, also dort, wo ein Kauf abgeschlossen und bezahlt wird. Das ist ein sehr sicherheitskritischer Teil.

So zumindest die Theorie. In der Praxis haben wir den Header dann nie wirklich scharf schalten können, weil wir *unbedingt* einen Haufen Scripte von Drittparteien einbinden mussten, von irgendwelchen Sicherheitsbadges bis hin zum Google Tag Manager, in dem selber noch weitere Scripte geladen wurden.

Das wäre _eigentlich_ auch kein Problem gewesen. Denn das sind Scripte von Drittparteien, die werden von Servern geladen die wir einfach in die Liste erlaubter Quellen angeben können. Richtig? Falsch. Denn diese Scripte wiederum fügen inline-Javascript in die Seite ein und erwarten dann, dass der ausgeführt wird. Oder wollen gleich direkt ein `eval` machen. Oder jemand aus der Marketingabteilung beschwert sich, dass das neue Script, das per Google-Tag-Manager eingebunden wurde, nicht läuft. Und schwupps – war die CSP wieder weg.

So ähnlich ist mir das auch in den beiden Projekten danach passiert. Zumindest das letzte davon hatte _keine Entschuldigung_, weil wir quasi auf der grünen Wiese angefangen haben. Wir hätten das von vornherein richtig machen können. Meine Rufe verhallten aber, weil es immer gerade etwas Wichtigeres zu tun gab.

Meine Hypothese war damals schon: [IT-Sicherheit ist uns, als Gesellschaft, nicht wichtig genug](/blogposts/boten-erschiessen).

### Rein subjektiv: Wie sieht die Lage heute aus?

Dieses Blog hat eine strenge CSP. Ist aber auch unwichtig, weil das Blog hier statisch gerendert wird, ich die einzige Person bin, die Inhalt einfügt. Es gibt in der Website keine Kommentarfunktion, keine Kommentare von Lesern, die automatisch dargestellt werden. Es gibt auch keine sensiblen Informationen, die man mit boshaften Scripten heraustragen könnte oder Aktionen, die man durchführen könnte. Die CSP hier ist streng, aber überflüssig.

Meine Nextcloud-Instanz hat eine komplizierte CSP. Es werden zwar inline-Scripte erlaubt, aber es wird mithilfe von nonce-Anweisungen kontrolliert, dass nur gewollte Scripte ausgeführt werden.

Aber schauen wir uns doch mal wichtigere Seiten an. Wikipedia zum Beispiel hat keine CSP. Hmm. Aber gut, da kann sowieso jeder drin herumschreiben, was brauchen die schon an Sicherheit? Außerdem ist die Software alt. Aber sicher sind Bankenwebsites da besser aufgestellt, oder? Wie sieht es denn zum Beispiel mit der [Sparkasse Essen](https://www.sparkasse-essen.de/de/home.html) aus?

```
script-src 'self' blob: https://morris-server.de:8801 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: https:; font-src 'self' data:; media-src 'self' data: blob: https://api.sparkassen-mediacenter.de https://sparkassen-mediacenter.de https://cdn.sparkassen-mediacenter.de
```

Oh-oh. Was sehen meine müden Augen da? `'unsafe-inline'` und `'unsafe-eval'` unter `script-src`? Kein nonce, kein hash, fast kein Schutz mehr. Aber das ist ja alles Subjektiv, oder? Was sagen denn ordentlich angelegte Studien dazu?

### Studien zur CSP-Nutzung

Da gibt es eine Studie aus 2020: *[Complex Security Policy? A Longitudinal Analysis of Deployed Content Security Policies](https://par.nsf.gov/servlets/purl/10173479)*. Da haben sie die Nutzung des CSP-Headers seit 2012 (also in etwa seit den Anfängen dieses Headers) auf 10000 populären Seiten verfolgt. Ergebnis: Die meisten Websites, wenn sie überhaupt eine CSP haben, setzen auf `'unsafe-inline'` und `'unsafe-eval'`. Ein paar andere Funktionen der CSP, die ich hier nicht erwähnt habe (hauptsächlich sicherzustellen, dass alle Resourcen per TLS geladen werden), werden immerhin mehr genutzt.

Die Autoren schreiben:

> The insights gathered from our survey indicate that CSP has earned a bad reputation due to its complexity in content restriction, resulting in developers shying away from any part of CSP.

Ich muss hier sagen: eine gute CSP muss nicht komplex sein. **Es sind die fucking-Überkomplexen Webseiten, die so viel Bullshit an Javascript laden, dass es unmöglich wird, eine effektive CSP einzusetzen.**

In der Studie sind auch noch ein paar andere Studien referenziert, die mit anderen Schwerpunkten zu einem ähnlichen Ergebnis gekommen sind (mindestens eine davon hat sich über eine Milliarde Webseiten angeschaut).

### Also was tun?

Glücklicherweise ist eine CSP nicht _notwendig_, um eine sichere Website zu haben. Es ist nicht so schwierig, keine XSS-Lücken einzubauen. Die Realität zeigt aber, dass es immer wieder passiert, und da wäre eine CSP wirklich hilfreich, um dir den Arsch zu retten.

Wo kann man Anfangen? Wie auch in der Studie erwähnt: In neuen Projekten ist das ganz einfach: Als _erstes_ die CSP festlegen. Dann die Website entwickeln. So merkt man sofort, wenn man mit dem was man tut in Konflikt mit der CSP gerät. Meist kann man die Probleme ausräumen, ohne die CSP zu ändern. Wenn nicht, ändert man die CSP, indem man zum Beispiel eine weitere Quelle hinzufügt, oder, wenn es wirklich schlimm wird, ein nonce oder einen Hashwert (das würde ich aber nicht empfehlen, das macht alles nur komplexer).

In bestehenden Projekten ist das schwieriger. Dort gibt es mitunter Inline-Javascript auf eine riesiege Codebase verteilt. Hier hilft eigentlich nur, bei jeder Änderung, die sowieso gemacht werden muss, ein bisschen weniger inline-Javascript als vorher zurückzulassen. Vielleicht auch die CSP zuerst nur für besonders sicherheitsrelevante Bereiche setzen und nach und nach ausweiten. Nonces verwenden, um schnell größere Codeteile CSP-kompatibel zu machen. Nextcloud zum Beispiel hat das geschafft (oder ist zumindest auf dem Weg dorthin), wie ich anhand einiger geschlossener Tickets und meiner aktuellen Installation sehen konnte.

In beiden Fällen gilt: man muss denen mit dem Geld klarmachen, dass es _wichtig_ ist, eine effektive CSP auf dem Server zu haben. Am besten mal bei denen vorbeischauen, wenn es mal wieder ein IT-Sicherheitsvorfall Schlagzeilen macht. Nach dem Motto: Das könnten wir sein.

Ebenfalls in beiden Fällen: Überprüft, ob ihr die Javascripte, die von externen Servern geladen werden, _wirklich_ braucht. Das ist nicht nur wichtig für die CSP, es ist auch so ein Sicherheitsproblem für eure Seite. Und wenn ihr sie einbindet: lasst nicht zu, dass sie euch ein `'unsafe-inline'` oder `'unsafe-eval'` aufzwingen. Denkt darüber nach: selbst wenn ihr denen vertraut: Wenn sie unfähig sind, ihren Code ohne inline-JS oder `eval` zu schreiben, dann kann es mit ihren Fähigkeiten, sicheren Code zu schreiben auch nicht so weit her sein. Wollt ihr denen dann _wirklich_ erlauben, Code auf eurer Seite auszuführen?
