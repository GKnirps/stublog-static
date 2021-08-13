---
title: Quantenkryptographie auf tagesschau.de
filename: quantenschluesselaustausch
date: 2021-08-13T07:09:24+02:00
update-date:
tags: verschlüsselung, it-sicherheit, quantenkryptografie, quantencomputer, staatstrojaner, hintertüren
category: hoellenmaschinen
summary: Auf tagesschau.de wird Verschlüsselung mit Quantenschlüsselaustausch als Lösung unserer IT-Sicherheitsprobleme dargestellt. Ich bin da anderer Meinung
---

Vor ein paar Tagen habe ich [diesen Artikel auf tagesschau.de](https://www.tagesschau.de/wirtschaft/quantenverschluesselung-101.html) gelesen. Da geht es darum, dass die erste „quantengesicherte“ Videokonferenz vor Kurzem in Bonn getestet wurde.

Dabei wird ein [Quantenschlüsselaustausch-Verfahren](https://de.wikipedia.org/wiki/Quantenschl%C3%BCsselaustausch) verwendet. Nun versteht mich bitte nicht falsch, ich halte Quantenkryptografie für eine coole Sache, auch wenn der Artikel zugibt, dass die Technik noch nicht praxisreis ist. Aber einige Aussagen in diesem Artikel sind schlichtweg falsch.

Da ist zum einen die Aussage „Nicht einmal das Kanzlerinnen-Handy ist abhörsicher“. Das mag stimmen, aber wenn das so ist, dann liegt das nicht daran, dass wir keine sicheren Verschlüsselungsverfahren haben. Richtig eingesetzt kann z.B. der symmetrische [Advanced Encryption Standard (AES)](https://de.wikipedia.org/wiki/Advanced_Encryption_Standard) nicht geknackt werden. Nicht mit aller Rechenleistung, die die Menschheit jetzt oder in absehbarer Zukunft aufbringen kann.

Außerdem wird über bisherige Verschlüsselungssysteme gesagt, dass die „von großen Rechnern allerdings bald geknackt werden können“. Ich vermute mal, das ist eine Anspielung an Quantencomputer. Herkömmliche Computer sind sehr lange davon entfernt, heute gängige symmetrische oder asymmetrische Verschlüsselunsverfahren zu knacken. Ich bin kein Experte, was Quantencomputer angeht, aber so wie ich das verstanden habe, könnten gängige asymmetrische Verschlüsselungsverfahren geknackt werden, deswegen wird auch schon an [Verfahren gearbeitet, die von einem Quantencomputer nicht geknackt werden können](https://de.wikipedia.org/wiki/Post-Quanten-Kryptographie).

Oh, und der Quantenschlüsselaustausch braucht auch einen „authentifizierten Kanal“. Was bedeutet das in der Praxis? Klassische asymmetrische Kryptografie.

### Die falschen Probleme lösen

Wenn das Kanzlerinnen-Handy also nicht abhörsicher ist, wie behauptet, so liegt das nicht an den eingesetzten Verschlüsselungsverfahren. Richtig eingesetzt sind die nämlich sicher. Das Problem liegt darin, wie sie angewandt werden und was sonst noch auf dem System zu finden ist. Das Problem sind unsichere Implementierungen.

Warum wollen Geheimdienste, Polizei und Politiker den [Staatstrojaner](https://netzpolitik.org/2021/ausweitung-bei-staatstrojanern-kollateralschaeden-im-prozessrecht/)? Warum werden [Hintertüren zu verschlüsselter Kommunikation gefordert](https://netzpolitik.org/2020/it-sicherheit-von-jahrelangen-debatten-ueber-hintertueren-unbeeindruckt/)? Warum will der Staat [bisher unbekannte SIcherheitslücken kaufen, um sie statt sie zu beheben als Waffe auszubewahren](https://netzpolitik.org/2020/reaktionen-auf-die-hackback-plaene-des-innenministeriums/)?

Die Antwort ist einfach: Weil man die Verschlüsselung selbst nicht angreifen kann, die ist sicher. Und wenn wir den Kern der Verschlüsselung auf Quantenkryptografie umstellen, so ändert sich nichts daran, dass die Umgebung, in der die Quantenkryptografie läuft, unsicher ist, und somit angegriffen werden kann.

Ich hatte ja vor Kurzem einen [Rant geschrieben, warum wir keine gute IT-Sicherheit haben](/blogposts/boten-erschiessen). Meine Antwort war: Es ist uns nicht wichtig genug. Das ist ein gesellschaftliches Problem. Quantenkryptografie auf dieses Problem zu werfen ist der Versuch, ein gesellschaftliches Problem mit technischen Mitteln zu lösen. Das funktioniert erfahrungsgemäß nicht.

Ich will nicht sagen, dass dieser Quantenschlüsselaustausch keine coole Idee ist, ich kenne mich nicht gut genug damit aus (oder mit Quantencomputern, oder mit Verschlüsselung im Allgemeinen). Ich kann mir schon vorstellen, dass einige Probleme damit gelöst werden können. Aber das Kernproblem ist nicht die Verschlüsselung, sondern der Kram drumherum. Solange der nicht sicher ist, bringt uns auch Quantenschlüsselaustausch nichts.
