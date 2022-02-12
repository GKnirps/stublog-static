---
title: Die Zeugnishashblockchain
filename: zeugnis_blockchain
date: 2022-02-12T14:12:49+01:00
update-date:
tags: blockchain, bundesdruckerei, sicherheitslücke, zerforschung, rant
category: hoellenmaschinen
summary: Ein staatliches Projekt zu digitalen Zeugnissen sollte keine Blockchain beinhalten.
image:
---

Zerforschung hat [einen Haufen Sicherheitslücken in einem Projekt zu digitalen Zeugnissen gefunden](https://www.heise.de/news/Schlechtes-Zeugnis-fuer-Zeugnisse-in-der-Blockchain-6370807.html). Auf die Sicherheitslücken möchte ich hier nicht eingehen. L. Wittmann wird hier zitiert mit: 

> Primär zeigt das alles mal wieder, dass die Bundesverwaltung oder in dem Fall das BMBF über keinerlei Digitalkompetenz verfügt und somit nicht fähig ist, solche Projekte zu beauftragen und zu steuern oder gar fachliche Entscheidung zu treffen.

Man hört eine gewisse Frustration heraus.

### Wtf blockchain?

Aber ich will hier hauptsächlich auf einen Aspekt eingehen: Eine Blockchain in dem Projekt. Vielleicht hätte man den Aufwand, den man in den Blockchain-Krams gesteckt hat, lieber in einen Sicherheitsaudit stecken sollen. Denn eine Blockchain ist hier meines erachtens völlig nutzlos. Warum?

Fangen wir erst einmal an, wie [die Bundesdruckerei die Sache beschreibt](https://www.bundesdruckerei.de/de/digitale-zeugnisse):

> Die PDF-Datei wird vom System mit einer eingebetteten maschinenlesbaren XML-Datei in einem standardisierten Format angereichert. […] Anschließend wird die Datei über einen Genehmigungsworkflow von den verantwortlichen Stellen (zum Beispiel eine Prüfungskommission oder eine Schulleitung) freigegeben und zentral durch das System der Bundesdruckerei digital signiert.

So weit, so gut. An dieser Stelle hat man schon ein Zeugnis, das von der Bundesdruckerei signiert wurde. Rein theoretisch könnte man hier aufhören. Die Bundesdruckerei kann ihren öffentlichen Schlüssel veröffentlichen, damit kann jeder überprüfen, ob eine gegebene Zeugnisdatei echt ist.

Und so ist das auch gedacht:

> Hiermit kann [ein Zeugnisempfänger] sich bei einer Institution bewerben, die die Gültigkeit bei Erhalt entweder offline durch Prüfung der PDF-Signatur im PDF-Reader oder online über die Prüfseite der Bundesdruckerei verifizieren kann.

Aber das ist nicht alles, davor kommt noch folgender Teil:

> Daraufhin wird die Prüfsumme der Zeugnisdatei errechnet und zusammen mit der eindeutigen Identität der ausstellenden Institution dauerhaft und unveränderbar in die Blockchain geschrieben.

Warum zur Hölle brauchen wir das? Gehen wir mal theoretisch mögliche Gründe durch.

#### Invalidierung

Was ist, wenn die Bundesdruckerei ein Zeugnis nachträglich ungültig machen will? Zum Beispiel in einem Betrugsfall? Da reicht dann die alleinige offline-Validierung nicht aus.

Man könnte höchstens den Schlüssel, mit dem das Zeugnis unterschrieben wurde, für ungültig erklären. Damit würde man aber einen Haufen anderer Zeugnisse auch ungültig machen. Wäre also doof.

Man bräuchte also eine Authorität, die sage, ob ein Zeugnis ungültig ist. Das wäre dann ja wohl die Bundesdruckerei. Und siehe da, die will ja auch eine API zum verifizieren bereitstellen.

Das Problem ist nur: Auch hier _braucht man keine Blockchain_. Es reicht, wenn die Bundesdruckerei eine Liste (aka „Datenbank“) mit Hashes von ungültig gemachten Zeugnishashes enthält. Dort kann dann einmal schnell nachgeschaut werden, ob das Zeugnis ungültig ist, wenn nicht, wird die Signatur überprüft und gut ist.

Die Idee von Blockchains ist, dass es eine große Gruppe von Akteuren gibt, die sich nicht gegenseitig trauen. In diesem Fall haben wir aber eine Authorität, die sagt, wann ein Zeugnis gültig ist. Wir brauchen keine verteilte Blockchain dafür. Und eine private Blockchain ist vollkommen nutzlos, weil man wie gesagt einfach eine Datenbank von Zeugnishashes braucht.

„Aber was ist, wenn wir nicht wollen, dass jemand rückwirkend alte Einträge ändert? Wir wollen überprüfen können, dass die Bundesdruckerei nicht schummelt!“.

Auch hier ist eine Blockchain nicht das richtige Werkzeug. Eine private Blockchain könnte genau so gut gefälscht werden, weil die Bundesdruckerei die alleinige Kontrolle über sie hat. Eine öffentliche Blockchain wäre ein riesiger Overhead im Verhältnis zu einer ganz einfachen, schon lange existierenden Technik: [ein ewiges Logfile](https://de.wikipedia.org/wiki/Ewige_Logdatei).

Technisch viel einfacher handhabbar, und die Bundesdruckerei müsste nur regelmäößig den letzten Hash veröffentlichen. So kann z.B. bei einem Audit nachgeprüft werden, dass niemand an den Daten herumgepfuscht hat.

#### Datenschutz

Wenigstens ist bei dem ganzen Projekt das Datenschutzkonzept erstzunehmen (soweit ich das beurteilen kann), was durchaus nicht selbstverständlich ist. Statement auf der Projektseite:

> [Digitale Schulzeugnisse] müssen fälschungssicher, datenschutzkonform und leicht zu überprüfen sein.

Solche Statements sind üblicherweise mit Vorsicht zu genießen, weil sie meist nicht mehr als heiße Luft sind.

Bei diesem Ansatz hier sehe ich aber tatsächlich kein Problem. Das Zeugnis ist weder von der Bundesdruckerei abrufbar, noch speichern sie es überhaupt dauerhaft ab. Zur Verifizierung wird nur ein Hash des Zeugnisses abgelegt, aus dem man das Zeugnis nicht rekonstruieren kann. Wenn es also bei der Validierungsplattform ein Datenleck gibt (was unweigerlich passieren würde, wenn man sich ansieht, das Zerforschung einige Anfängerfehler in der Plattform entdeckt hat), kommen wenigstens keine sensiblen Zeugnisdaten abhanden.
