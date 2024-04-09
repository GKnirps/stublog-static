---
title: Die SSH-Backdoor
filename: xz_ssh_backdoor
date: 2024-04-09T20:10:53+02:00
update-date:
tags: ssh, xz utils, ross anderson, zerforschung, bruce schneier, fefe
category: aus_zweiter_hand
summary: Kaum ist man mal für zwei Wochen im Urlaub, schon bricht die Hölle los. Ross Anderson ist gestorben, Zerforschung hat mal wieder zugeschlagen und vor allem: Jemand hat über eine Abhängigkeit von SSH eine Backdoor eingebaut und wäre damit fast durchgekommen.
image:
language:
---

So, nachdem ich vorgestern erst einmal [meinem Ärger Luft lassen musste](/blogposts/sncf_storno) habe ich jetzt mal ein bisschen geschaut, was während meines Urlaubs so alles in der IT-Sicherheit passiert ist.

### Ross Anderson

Zum Einen wäre da natürlich der Tod von [Ross Anderson](https://de.wikipedia.org/wiki/Ross_Anderson_(Informatiker)), Informatiker, Sicherheitsforscher, Kryptologe und Autor u.a. von [Security Engineering](https://www.cl.cam.ac.uk/~rja14/book.html). Sein Tod wurde auch von anderen Sicherheitsexperten wie [Bruce Schneier](https://www.schneier.com/blog/archives/2024/03/ross-anderson.html) und [Fefe](https://blog.fefe.de/?ts=98f83ef3) betrauert.

### Zerforschung und CanGuard

Dann hat Zerforschung mal wieder eine Bombe platzen lassen. Aufgrund irgendwelcher Regulierungen bezüglich der Cannabis-Legalisierung kann man „Cannabis Social Clubs“ gründen, für die es erforderlich ist, Daten von allen Mitgliedern zu erfassen.

Neue Bürokratie, neue Software, die man verkaufen kann. So hat ein Hersteller die Software „CanGuard“ beworben, mit der man Mitglieder, Satzung und Lagerbestände verwalten kann. Laut Werbung sicher und Datenschutzkonform. Sollte man ja wohl auch erwarten, immerhin geht es hier um persönliche Daten, Name, Anschrift und natürlich implizite Informationen über (legalen) Drogenkonsum.

Dann haben sich ein paar Leute gewundert, warum sie plötzlich viel Spam bekommen, und beim CanGuard-Anbieter gefragt, ob es bei denen ein Leck gab. Die Antwort: Nein, unsere Software ist total sicher.

[Zerforschung hat sich das dann angeschaut](https://zerforschung.org/posts/canguard/), und ich kann nur empfehlen, den Artikel zu lesen. Kurz zusammengefasst: Die Software ist _nicht_ sicher, die Fehler, die gemacht wurden, sind absolute Anfängerfehler, und die Daten von über 1000 Nutzer_innen standen _für jeden lesbar_ im Netz.

Wieder einmal kann ich nur bekräftigen: Zusicherungen von Firmen, dass ihre Software sicher seit ist _nicht zu trauen_! In diesem Fall waren die Sicherheitslücken wieder so eklatant, dass offensichtlich kein Gedanke an Sicherheit verschwendet wurde. Naja, immerhin waren die Passwörter mit bcrypt gehashed. Herausreichen sollte man sie trotzdem nicht.

### Die XZ Utils / SSH-Backdoor

Aber Platz 1 hat auf jeden Fall die [SSH-Backdoor](https://lwn.net/ml/oss-security/20240329155126.kjjfduxw2yrlxgzm@awork3.anarazel.de/) erhalten. Da hat jemand sich über drei Jahre hinweg in die Entwicklung von XZ Utils (einer Bibliothek, die von SSH verwendet wird) eingeschlichen, mithilfe mutmaßlich gefälschter Accounts sozialen Druck auf den Maintainer der Bibliothek ausgeübt, doch bitte als Maintainer abzutreten und dann ein paar subtile Änderungen gemacht, die _während des Buildprozesses_ eine Backdoor einfügten. Aufgefallen ist das nur durch Zufall (ein Entwickler hat sich über hohe Latenzen gewundert oder so), und zwar kurz bevor die größeren Linuxdistributionen die korrupte Version in ihre Repos übernommen haben.

Kurz: Da steckt eine ganze Menge kriminelle Energie und Zeit (und damit vermutlich auch Geld) hinter, und es wird viel spekuliert, wer dahinter stecken könnte und [welche Probleme dahinterstecken, dass das überhaupt möglich war](https://www.schneier.com/blog/archives/2024/04/xz-utils-backdoor.html). So oder so sicher das Security-Thema des Jahres, ungefähr auf dem Niveau der [log4shell-Lücke](/blogposts/log4j), nur dass das hier absichtlich war, dafür aber nicht weit verbreitet, als es entdeckt wurde. Natürlich gibt es dazu auch [einiges](https://blog.holz.nu/2024/03/29/0.html) an [Informationen](https://boehs.org/node/everything-i-know-about-the-xz-backdoor), die die Sache genauer erklären als ich hier.
