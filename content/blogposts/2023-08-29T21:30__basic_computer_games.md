---
title: Basic Computer Games
filename: basic_computer_games
date: 2023-08-29T21:30:48+02:00
update-date:
tags: basic, rust, coding horror
category: hoellenmaschinen
summary: Ende 2021 hat Jeff Atwood vom Coding Horror-Blog ein Projekt zur Reimplementierung alter Basic-Spiele in neuen Sprachen aufgestellt.
image:
language:
---

Ende Dezember 2021 hat Jeff Atwood auf seinem Blog „Coding Horror“ einen [Artikel](https://blog.codinghorror.com/updating-the-single-most-influential-book-of-the-basic-era/) zu einem mittlerweile 50 Jahre alten Buch veröffentlicht: [BASIC Computer Games](https://en.wikipedia.org/wiki/BASIC_Computer_Games). Der Artikel ist interessant, ich kann empfehlen ihn zu lesen.

Eine kurze Zusammenfassung: In den Zeiten, als private Computer noch rar waren, das das Internet noch in den Kinderschuhen steckte (vom WWW war noch nicht zu denken) und es noch schwierig war, Software zu verteilen, wurde ein Buch mit 101 Computerspielen veröffentlicht. In BASIC geschrieben, zum selberabtippen.

Ich kann das nicht selber bezeugen, aber anscheinend war dieses Buch sehr einflussreich auf die Entwicklung vieler angehender Programmierer zu der Zeit. Deswegen hat Jeff Atwood ein Projekt aufgesetzt, die Spiele aus diesem Buch in modernen Programmiersprachen zu reimplementieren.

Ursprünglich hielt ich das zwar für ein cooles Projekt, aber da rust nicht unter den offiziell gelisteten Sprachen war hatte ich nicht viel Lust, daran zu arbeiten.

Als ich aber vor Kurzem noch einmal in den Blogpost geschaut habe, musste ich feststellen, dass rust mittlerweile (seit spätestens März 2022) auf der Liste steht.

Also habe ich mir ein Spiel herausgepickt, das noch nicht portiert worden ist (Spiel 53, „King“) und habe es in rust nachgeschrieben.

Das Ergebnis ist nicht der schönste rust-Code, den ich je geschrieben habe, aber es war ein interessantes kleines Projekt für mich. Der Code ist [auf github gehosted](https://github.com/coding-horror/basic-computer-games), alle die wollen können mitmachen. Die originalen Spiele sind in Basic kann man [in diesem JS-basierten Interpreter spielen](https://troypress.com/wp-content/uploads/user/js-basic/index.html). Viel Spaß!
