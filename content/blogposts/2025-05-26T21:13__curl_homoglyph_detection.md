---
title: Schutz vor Homograph-Attacken im Quellcode
filename: curl_homoglyph_detection
date: 2025-05-26T21:13:42+02:00
update-date:
tags: unicode, daniel stenberg, homoglyphen, homographen, curl, it-sicherheit, phishing
category: hoellenmaschinen
summary: Daniel Stenberg, curl-Entwickler, hat einen Blogpost geschrieben, wie man sich gegen Angriffe mit versteckten Homographen im Quellcode wehrt.
image:
image-alt:
language:
---

[Homographische Angriffe](https://de.wikipedia.org/wiki/Homographischer_Angriff) sind nichts Neues. Dabei wird ein Unicode-Codepoint durch einen anderen ersetzt, der (mit gängigen Schriftarten) genau so (oder sehr ähnlich) dargestellt wird (Homoglyphen). Das Ergebnis ist dann ein Wort oder eine andere Zeichenkette (Homograph), die für einen menschlichen Beobachter identisch mit der ursprünlichen Version ist, aber für den Computer völlig anders ist.

Das kann man auf verschiedene Art und Weise einsetzen. Ein Anwendungsfall war (oder ist?), [Plagiatserkennungssoftware mit Homoglyphen zu täuschen](https://eprints.whiterose.ac.uk/id/eprint/112665/1/paper_247v2.pdf). Der bekannteste Anwendungsfall ist aber, einen Domain-Namen einer bekannten Website zu nehmen, ein Zeichen auszutauschen, den so entstandenen Domainnamen registrieren und eine oberflächliche Kopie der urpsrünglichen Website als Phishing-Seite dort auszuliefern. Dann verteilt man überall links zu dieser Seite (z.B. durch Phishing-E-Mails). Besucher der Website sehen eine Website, die richtig aussieht, deren Domain richtig aussieht und die sogar ein gültiges Zertifikat hat. Die letzten beiden Punkte zusammen wären ohne diesen Angriff nicht zu schaffen gewesen.

Webbrowser als Schutz dagegen angefangen, bei Domainnamen mit Zeichen aus unterschiedlichen Schriftsystemen die nicht-ASCII-Codepoints in ihrer eigentlichen Codierung [Punycode](https://de.wikipedia.org/wiki/Punycode) darzustellen. So sieht für die Besucher der Seite die Domain nicht mehr identisch zu der Domain der echten Seite aus.

Das ist bei Browsern in der Adressleiste. Ein curl-Entwickler hat jetzt mal [demonstriert, dass das Problem aber auch in Open Source-Projekten auftreten kann](https://daniel.haxx.se/blog/2025/05/16/detecting-malicious-unicode/). Weder die CI-Pipeline noch menschliche Reviewer haben das bemerkt. Denn übliche Texteditoren stellen die Zeichen halt identisch dar, und übliche CI-Tools achten nicht darauf.

Daniel Stenbergs Lösung war, für curl einen CI-Job anzulegen, der den Code nach Codepoints absucht, die nicht auf einer whitelist stehen (diese Liste enthält ASCII und ein paar andere Zeichen). Bei curl funktioniert das, weil der Quellcode sowieso größtenteils nur ASCII enthält.

Wie man mit Quellcode umgeht, der mehr nicht-ASCII-Zeichen enthält, wird dort nicht geklärt. Ich hätte da spontan mehrere Ideen. Man könnte zum Beispiel Lokalisierungsdateien (das sind Dateien, in denen Übersetzungen der Texte eines Programms stehen) von dem Check ausklammern. Und wenn man wirklich im Quelltext selber Unicode braucht, kann man den in Strings vielleicht escapen, damit der Check nicht darauf anspringt (und ein Mensch sofort sieht, dass dort ein anderes Zeichen steht).

Schwieriger wird es, wenn Unicode auch in den Kommentaren genutzt wird (weil die Entwickler den Code in ihrer eigenen Sprache kommentieren). Dann bräuchte man einen Scanner, der die entsprechende Programmiersprache versteht und weiß, was ein Kommentar ist und was nicht. Das würde die ganze Sache wieder so komplex macht, dass es nur wenige Leute benutzen wollen und irgendwer vielleicht doch noch ein Schlupfloch findet, um etwas einzuschleusen.
