---
title: Sparkasse pushTAN und das Passwort
filename: sparkasse-passwort
date: 2023-10-23T22:26:36+02:00
update-date:
tags: passwort, sparkasse, pushtan, user interface, user experience
category: hoellenmaschinen
summary: Die Sparkasse hält sich mit ihrer pushTAN-App nicht an moderne Passwortrichtlinien. Außerdem ist die Benutzbarkeit verbesserungswürdig.
image:
language:
---

Ach ja, Passwortregeln. Darüber habe ich mich ja schon vor einem Jahr [aufgeregt](/blogposts/umzug_nijmegen). Kurz: Viele Passwortregeln in freier Wildbahn sehen etwa so aus ([Quelle](http://www.ibash.de/zitat_56773.html)):

> Es tut uns leid, Ihr Passwort muss mindestens einen Großbuchstaben, zwei Ziffern, ein Symbol, eine inspirierende Nachricht, einen Zauberspruch, ein Gang-Logo, eine Hieroglyphe und das Blut einer Jungfrau enthalten.

Das Problem ist so verbreitet, dass es schon [Spiele dazu gibt](https://passwordgame.io/). Dabei ist der heutige Stand der Wissenschaft: Wenn man überhaupt ein Passwort benutzen muss (anstatt einer anderen Möglichkeit zur Authentifizierung), dann muss es nur zwei Regeln einhalten:

1. Es muss eine Mindestlänge haben (häufig wird 10 genannt)
2. Es darf nicht in einem Wörterbuch stehen (Bonus: Es darf nicht in einer Datenbank von geleakten Passwörtern vorkommen)

In der Praxis sieht das leider immer noch anders aus. Das geht so weit, dass sich der Typ, der sich die typischen Passwortregeln ausgedacht hat, [öffentlich entschuldigt hat](https://gizmodo.com/the-guy-who-invented-those-annoying-password-rules-now-1797643987).

### Sparkasse pushTAN

Die Sparkasse (hier speziell die Sparkasse Essen, aber die scheinen größtenteils alle dieselbe App zu nutzen) scheint davon noch nichts mitbekommen zu haben. Eine Verwandte von mir wollte heute die pushTAN-App einrichten und wurde mit einem Formular konfrontiert, dass in etwa so aussah:

![Screenshot: Ein Passworteingabeformular. Unter dem Formular eine Liste mit grün abgehakten Punkten: „Mindestens eine Zahl“, „Mindestens ein Buchstabe“, „8 oder mehr Zeichen“, „Mindestens ein Sonderzeichen“, darunter in rot ein Button „App-Passwort festlegen“](/file/sparkasse_pushtan_pw.webp)

Wie man sieht einmal grob alle dummen Regeln aufgelistet. Im konkreten Fall sah das Formular aber etwas anders aus: Der Button zum Bestätigen war ausgegraut.

Komisch. Es sind doch alle Passwort-Regeln erfüllt, oder? Jede der Regeln ist grün markiert und mit einem Häkchen versehen. Das heißt, dass die erfüllt sind, oder?

Nein, natürlich nicht. Es stellte sich heraus, dass meine Verwandte übersehen hat, dass auch ein Sonderzeichen im Passwort enthalten sein muss. Der Punkt war trotzdem grün und abgehakt. Alle Punkte waren grün und abgehakt, selbst wenn das Passwortfeld leer war.

Das ist schon komplett konterintuitive Bedienung. Wenn die Liste wenigstens schwarzweiß gewesen wäre, dann hätte sie einfach nur wie eine Auflistung der Regeln ausgesehen. Grün und mit Häkchen sah sie aus wie eine Checkliste, in der alles abgehakt ist.

Und nicht nur das, es gab sonst _überhaupt keinen Hinweis_, warum der „App-Passwort festlegen“-Button ausgegraut war. Nichts. Man hat keinen Hinweis, dass die Eingabe auch nur in irgendeiner Form ungültig ist. Nur der Absenden-Button ist halt ausgegraut.

Das ist so typisch Sirius Cybernetics Corporation: Die offensichlichen Designschwächen täuschen nur zu gut über die absolut grundlegenden Designprobleme hinweg.
