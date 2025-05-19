---
title: Furchtbare Passwortrichtlinien bei der Bundesagentur für Arbeit
filename: ba_passwortrichtlinie
date: 2025-05-19T20:25:28+02:00
update-date:
tags: bundesagentur für arbeit, passwort, rant, passwordgenerator, pwgen
category: hoellenmaschinen
summary: Im Jahr 2025 hat die Bundesagentur für Arbeit immer noch Passwortrichtlinien, die besser ins Jahr 2000 passen würden.
image:
image-alt:
language:
---

Ich musste vor Kurzem einen Account bei der Bundesagentur für Arbeit anlegen. Ein Account braucht irgendeine Authentifizierung, traditionell ein Passwort. Zunächst einmal die positiven Seiten: Man muss zwar ein Passwort nutzen, muss aber nach Anlegen des Accounts entweder einen zweiten Faktor ([TOTP](https://de.wikipedia.org/wiki/Time-based_one-time_password)) hinzufügen oder kann einen [Passkey](https://de.wikipedia.org/wiki/FIDO2#Passkey) anlegen. Die Mindestlänge des Passworts ist mit 12 Zeichen angemessen, aber nicht übertrieben hoch.

Aber ich würde diesen Blogpost nicht schreiben, wenn ich nicht etwas zu meckern hätte. Ich habe mich ja [früher schon](/blogposts/umzug_nijmegen) [über doofe Passwortregeln ausgelassen](/blogposts/sparkasse-passwort). Andere Leute auch, z.B. [Jeff Atwood](https://blog.codinghorror.com/passphrase-evangelism/) vor 20 Jahren, [Bruce Schneier](https://www.schneier.com/blog/archives/2021/11/why-i-hate-password-rules.html) und andere Experten. Deswegen sollte es keine Überraschung sein, dass ich hier mal wieder über furchtbare Passwortregeln herziehen muss. Und zwar die schlimmsten, die mir bisher untergekommen sind (von Spielen wie [Password Purgatory](https://passwordpurgatory.com/) mal abgesehen):

![Eine Liste von Passwortkriterien. Darunter die üblichen Verdächtigen (Groß-/Kleinschreibung, Ziffern, Sonderzeichen) aber auch „Keine unerlaubten Zeichen“](/file/ba_passwortrichtlinien_2025.webp)

Als Erstes ist das natürlich ein Rundumschlag über alles, was man als Pflicht ins Passwort nehmen muss. Großschreibung, Kleinschreibung, Ziffern _und_ Sonderzeichen. Für mich hat aber „Keine unerlaubten Zeichen“ den Vogel abgeschossen. Denn erst einmal ist das eine dumme Regel, wenn man sichere Passwörter ermöglichen möchte. Und zum Anderen **steht nicht dabei, welche Zeichen nicht erlaubt sind**.

Ich wäre das ja einfach übergangen, hätte ich nicht die Komfortfunktion von Firefox benutzt und mir ein sicheres, zufälliges Passwort generieren lassen. Nehmen wir mal Beispielsweise an, das wäre `6=kjW.s%Ka3N$>N` gewesen (war es nicht, das hier stammt aber auch von Firefox). 15 Zeichen, alles druckbare ASCII-Zeichen, bunte gemischte Groß- und Kleinschreibung, Sonderzeichen, unmöglich zu merken (aber wozu hat man Passwortmanager), das ganze Programm. Was sagt die BA-Seite?

Die sagt: „Geht nicht, da sind unerlaubte Zeichen drin“. Welche Zeichen sind nicht erlaubt? Muss ich das jetzt echt manuell nachprüfen? Ein anderes, von Firefox generiertes Passwort, `V"YP6YdxNV;"CTj`, ist auch nicht erlaubt (enthält keine Ziffern). `pwgen 12` liefert mir so twas wie `Igah9aeli1ba`. Da fehlt dann das Sonderzeichen. Nun könnte ich mir natürlich die Manpage von `pwgen` durchlesen und mit der Option `-y` noch Sonderzeichen hinzufügen. Auf die Gefahr hin, dass ich dann wieder ein unerlaubtes Zeichen habe. Also habe ich getan, was jeder frustrierte Nutzer machen würde: Einfach an das von pwgen generierte Passwort ein `!` (oder ein anderes der Sonderzeichen aus der Liste) drangehängt. Schwupp, Passwort wird akzeptiert.

Das ist natürlich Murks. Da benutzt man schon einen Passwortgenerator, der auch sichere Passwörter ausspuckt, und dann werden die auch nicht erlaubt. So wird das nichts.

Man hätte an dem Passwort auch _ganz_ vorbeikommen können, wenn man direkt bei der Accounterstellung die Passkey-Option angeboten hätte. Bei dieser Seite hat sich offensichtlich jemand Gedanken über die Loginsicherheit gemacht, mit der Option für Passkeys und 2FA. Warum zur Hölle werden dann noch Passwortregeln verwendet, die vom Anfang des Jahrtausends stammen und schon seit 20 Jahren in der Kritik stehen?
