---
title: VW und die Security schon wieder
filename: vw_app_security
date: 2025-05-21T18:07:04+02:00
update-date:
tags: volkswagen, security, datensicherheit
category: hoellenmaschinen
summary: Noch mal ein fieses Sicherheitsproblem in den APIs von Volkswagen.
image:
image-alt:
language:
---

Ende letzten Jahres, während des 38C3, ist Volkswagen ja schon einmal durch die Presse gegangen, weil [Positions- und andere persönliche Daten von Autos und deren Nutzern offen im Netz standen](https://media.ccc.de/v/38c3-wir-wissen-wo-dein-auto-steht-volksdaten-von-volkswagen).

Stellt sich heraus: Im November letzten Jahres hat ein indischer Sicherheitsforscher unabhängig davon auch [ein paar Sicherheitslücken in der VW-API gefunden](https://loopsec.medium.com/hacking-my-car-and-probably-yours-security-flaws-in-volkswagens-app-24b34c47ba89).

Darunter waren:

- ein Sicherheitscode, um ein Auto in der App zu registrieren, war vierstellig und ohne rate limiting, er lies sich also brute-forcen
- interne Passwörter waren über eine API ohne Sicherung abrufbar
- Persönliche Informationen von Fahrzeugeigentümern waren abrufbar, man brauchte nur eine Fahrzeugnummer, die man wohl am Auto ablesen kann
- Reparatur- und Wartungsgeschichte des Autos war auch mit nur dieser Fahrzeugnummer abrufbar

Wann genau VW das behoben hat, kann ich aus dem Artikel nicht herauslesen, aber es sieht so aus, als hätten sie Monate dafür gebraucht. Zwei dicke Sicherheitslücken bei VW, praktisch gleichzeitig. Ich vermute mal, dass VW durch den ganzen Trubel einmal die ganzen APIs gut auf Lücken getestet hat. Wetten würde ich nicht darauf. So oder so vermute ich, dass da auch jetzt noch mehr zu finden ist.
