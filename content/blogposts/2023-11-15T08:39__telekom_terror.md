---
title: Telekomterror
filename: telekom_terror
date: 2023-11-15T08:39:33+01:00
update-date: 2023-11-15T17:55:00+01:00
tags: rant, telekom, deutsche telekom, email, usability, user experience, adressen
category: rant
summary: FUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUCK
image:
language:
---

Ich ziehe ja demnächst wieder nach Deutschland, und in meiner neuen Wohnung brauche ich natürlich auch einen Internetzugang. Die Tarife bei den verschiedenen Anbietern unterscheiden sich da nicht so sehr, von der Qualität habe ich in der Vergangenheit keine großen Unterschiede bemerkt (aka sie sind alle Murks), also gehe ich zur Telekom, da sind wenigstens der Anschluss am Haus und der Internetanbieter in derselben Hand.

Also gehe ich auf die Telekomseite, wähle einen Tarif (der jede Menge Kram hat, den ich nicht brauche, z.B. eine kleine Menge Cloud-Speicher bei der Telekom und eine E-Mail-Adresse bei der Telekom) und versuche mich durchzuangeln.

### Hindernis 1: Verfügbarkeitsabfrage

Bevor ich den Anschluss in den Warenkorb legen kann, möchte die Telekom prüfen, ob dieser Anschluss bei mir verfügbar ist. Soweit eigentlich verständlich.

Wenn ich in der Verfügbarkeitsabfrage den Straßennamen meiner neuen Wohnung angeben möchte, löscht die Seite den Straßennamen, bevor ich überhaupt die Hausnummer eingeben kann. Vorher blitzt ganz kurz auf, dass zu dieser Straße keine Treffer gefunden werden konnten.

Aber je nachdem auf welchem Weg ich mich durch die Seite durchklicke, komme ich auf eine Seite, wo ich den Anschluss ohne vorherige Prüfung in den Warenkorb legen kann. Also mache ich das.

Wenn ich dann zur Kasse gehe, kommt dort die Prüfung. Meine Straße (die es definitiv gibt, die Adresse ist korrekt, ich habe es mehrmals geprüft) findet er auch hier nicht. Allerdings kann ich hier einen Haken auswählen, dass die Prüfung übersprungen werden soll (ich denke mal, dass das bedeutet, dass die Telekomtechniker am anderen Ende dann das Problem mit der fehlenden Straße lösen dürfen).

Wenn ich aber auf diesen Haken klicke, komme ich trotzdem nicht weiter. Kein Hinweis, warum. Wenn ich dann aber nach oben im Formular schaue stelle ich fest, dass dadurch ein neues Pflichtfeld hinzugefügt wurde: Vorwahl des Ortes. Also die Vorwahl rausgesucht, eingegeben, und jetzt kann ich endlich weitermachen.

### Hindernis 2: Die E-Mail-Adresse

Nun muss ich alle möglichen Daten eingeben, unter anderem auch meine E-Mail-Adresse. Nun nutze ich für solche Sachen ungerne meine Posteo-Adresse (die ich hauptsächlich für private Kommunikation vorbehalte) und lieber meinen alten Googlemail-Account.

Der Googlemail-Account hat auch den Vorteil, dass ich im lokalen Teil der Mail (also vor dem @) etwas in der Form von `+irgendwas` hinzufügen kann, und das zum selben Postfach führt wie ohne dieses Suffix. Ich benutze das gerne, um zu kennzeichnen, wo ich diese Mail angegeben habe. So kann ich nachvollziehen, ob jemand meine Mailadresse geleaked oder verkauft hat. Die Adresse sieht dann in diesem Fall etwa so aus `festes.praefix+telekom@googlemail.com`.

Das ist eine gültige E-Mail-Adresse, aber das Telekom-Formular behauptet, sie sei nicht gültig.

### Hindernis 3: Kontaktformulare

Also nach Support gesucht. Der ist gut versteckt. Es gibt Kontaktformulare. Da muss man sich vorher durch ein dreiseitiges Formular durchkämpfen um zu bestimmen, welches Kontaktformular man haben möchte. Keine der Optionen passte wirklich auf mein Problem, also habe ich die nächstbesten Optionen genommen. Dann sagt mit das Teil, dass so ein Formular einige Tage braucht, um bearbeitet zu werden (und ich habe extra früh angefangen, mir den Anschluss zu besorgen, damit ich nicht _viel_ zu lange ohne Internetanschluss dastehe) und lässt mich endlich zum Formular.

Im Formular muss ich dann wieder alles Mögliche angeben: Name, Adresse, Anschlussnummer… Moment, Anschlussnummer? ABER MEIN FUCKING PROBLEM IST DOCH DASS ICH KEINEN ACCOUNT ERSTELLEN KANN GESCHWEIGE DENN EINEN ANSCHLUSS BUCHEN!

Dasselbe Problem hatte ich auch, als ich meinen Minecraft-Account von dem klassischen Mojang-Account zu einem Microsoft-Account umstellen musste. Microsoft wollte meine (gültige) Mailadresse nicht. Mojang konnte mir keinen Support geben, weil das Microsofts Sache war (bestand aber darauf, dass ich meinen Account umstelle). Der Microsoft-Support funktioniert nur, wenn man bereits ein Microsoft-Konto hat, selbst wenn mein Problem ist DASS ICH KEINS ERSTELLEN KANN.

### Hindernis 4: Chatbot

Aber es gibt noch andere Supportwege. Zum Beispiel einen Chat. Hier werde ich zunächst einem Chatbot vorgeworfen. Der hat nun bei Weitem nicht ChatGPT-Qualität, obwohl das am Ende vermutlich wenig Unterschied gemacht hätte. Nachdem das Teil meine Fragen zwei Mal nicht verstanden hatte, leitete es mich an eine menschliche Supportperson weiter.

### Hindernis 5: missing link

Die Supportperson hat sich dann über die Sache mit dem Pluszeichen gewundert und gemeint, dass das eigentlich gehen müsste. Sie hat dann für mich den Account angelegt. Ich habe eine Mail zur Bestätigung meiner E-Mail-Adresse erhalten.

In dieser Mail fehlte aber der Bestätigungslink. Ich zeige nur Plaintext-Mails an. Diese Mail hatte zwar einen Plaintextteil und einen HTML-Teil, aber der Plaintextteil enthielt den Link zur Aktivierung nicht. Andere Links enthielt er jedoch. Die Mail war also kaputt.

Ich habe dann mit dem Texteditor den Link aus dem HTML-Teil der Mail herausgefischt.

### Hindernis 6: Ausland

Nun sitze ich noch in den Niederlanden. Als ich den o.g. Link dann im Browser aufrief, sagte mir die Seite, dass ich das nur in Deutschland machen könnte. Ein VPN hatte ich gerade nicht zur Hand, also was tun?

Ich habe dann mein armes Mütterchen angerufen, damit die den Link für mich aufruft. Bei ihr hieß es aber dann „Die Seite sagt, dass diese E-Mail-Adresse schon bestätigt wurde.“

Mit anderen Worten: Die Seite hatte mich glatt angelogen, als sie behauptet hat, dass ich das nur aus Deutschland machen konnte

### Hindernis 7: Login

Aber der Account ist jetzt aktiviert, also versuche ich mich, einzuloggen:

> Benutzername ist nicht korrekt.

Na toll. Ich habe die Mailadresse gerade bestätigt. Ich habe _von euch_ eine Mail an diese Adresse bekommen! Ihr könnt nicht sagen, dass die nicht gültig ist.

Aber mal schauen, welche Optionen zum Login gibt es noch?

- Telekom-Mobilfunk-Nummer? Habe ich nicht
- E-Mail-Adresse? Funktioniert nicht
- Zugangsnummer? Dafür muss ich erstmal einen Anschluss gebucht haben
- Benutzername? Kenne ich nicht, oder ist auch die E-Mail-Adresse

Aber vielleicht braucht es ja einfach ein bisschen Zeit, bis die Aktivierung der Mail auch in den richtigen System angekommen ist. Also einfach regelmäßig probieren, mich einzuloggen

### Hindernis 8: Zu lange inaktiv

Wenn ich eine ganze Zeit lang regelmäßig versuche, mich einzuloggen, läuft irgendwann meine session ab (obwohl ich die ganze Zeit aktiv war). Ich kriege also eine Fehlermeldung. Wenn ich dann auf den Link in der Fehlermeldung klicke lande ich auf `https://www.telekom.de/hilfe/technical-error?samChecked=true`, was zumindest gestern und heute eine absolut leere Seite ist. Absolut leer im Sinne von: Das ist eine komplett weiße Seite.

### Hindernis 9: Nervenzusammenbruch

Ich war mittlerweile seit über zwei Stunden beschäftigt, mir einen Account zu erstellen. Es war spät in der Nacht, und piqaerugfböaosuedfbdösaidufbüweiougfhalsidhvdöad.kjbsadfüiou FUCK YOU

### Update

Ich habe heute noch einmal versucht, per Chat Support zu bekommen. War nur mäßig erfolgreich bis zu dem Punkt, wo ich zugestimmt habe, den Chatsupporttypen den Vertrag für mich abschließen zu lassen. Der sagt mit dann, dass ich nur 16MBit/s kriegen kann.

Nun habe ich aber inzwischen herausgefunden, warum die Verfügbarkeitsprüfung bei mir nicht ging: Wenn ich statt „Soundsostraße“ nur „Soundsostr.“ eingebe, funktioniert das prima. Und das Ergebnis ist, dass bei mir 100MBit/s verfügbar sein sollten. Und diese Auskunft habe ich dann dem Typen vom Chatsupport mitgeteilt.

Der meinte, das läge daran, dass ich da wohl wholebuy-Kunde werden würde, was heißt, dass sie die Leitung von einem anderen Anbieter buchen. Er hat auch noch gesagt, ich solle dafür eine Nummer anrufen, die er mir nannte. Bevor ich nachfragen konnte, was ich denen dort erzählen soll, hatte er den Chat schon beendet.

Nun war damit aber der Hauptgrund weggefallen, warum ich überhaupt die Telekom gewählt hätte: Dass Infrastruktur und Internetzugang in derselben Hand liegen.

Also habe ich mir einen anderen Anbieter herausgesucht. Namen nenne ich )noch?) nicht, ich schaue mir erst einmal an, wie das läuft.

Dort musste ich jedenfalls aus mir unerfindlichen Gründen auch meine bisherige Wohnadresse angeben. Dazu gehörte auch ein Feld, in dem ich die Postleitzahl eingeben musste. Das Feld ließ aber nur fünfstellige PLZ zu, die nur aus den dezimalen Ziffern bestanden (also das deutsche Format). In den Niederlanden ist das Format aber anders (vier Ziffern plus zwei Buchstaben). Konnte ich nicht eingeben. Also habe ich einen Dummy-Wert eingegeben und in einer separaten Supportanfrage nachgehakt, warum sie die Adresse überhaupt brauchen, dass die PLZ falsch ist und wenn sie die richtige PLZ haben wollen, sie sich noch einmal bei mir melden sollen.

Fürs Protokoll: Die altbekannten [falsehoods programmers believe about addresses](https://www.mjt.me.uk/posts/falsehoods-programmers-believe-about-addresses/), den Hinweis, dass das beste Adressformular einfach ein großes Freitextfeld ist, und ein Hinweis auf [die Fallstricke von E-Mail-Adress-Validierungen](https://haacked.com/archive/2007/08/21/i-knew-how-to-validate-an-email-address-until-i.aspx/):

> What I found out was surprising. Nearly 100% of regular expressions on the web purporting to validate an email address are too strict.

Ein Kollege von mir ist übrigens der Meinung, dass, wenn eine kleine Minderheit von Entwicklern unfähig ist, Pluszeichen in E-Mail-Adressformularen zuzulassen, der Standard geändert werden müsse, so dass Pluszeichen überhaupt nicht mehr erlaubt sind.

Ich bin da anderer Ansicht.
