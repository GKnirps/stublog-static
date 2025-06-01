---
title: Monopole im Internet
filename: internet_monopole
date: 2021-01-10T22:38:49+01:00
update-date:
tags: trump, twitter, facebook, parler, aws, google, apple, monopol, internet, rant
category: ithought
summary: Trump wurde von Twitter verbannt. Das ist gut. Das Problem ist nicht Twitters Verhalten, sondern Twitters quasi-Monopol.
---

Trump wurde nun also dauerhaft von Twitter verbannt (seine bisherige Immunität hätte sowieso nur noch bis zum 20.1.2021 gehalten). Und von Facebook. Und die Microblogging-App Parler wurde von Google und Apple aus dem App-Store genommen, außerdem hat AWS denen ihr Webhosting gekündigt. Und alles nur wegen dieses kleinen blutigen Aufstandes gegen den rechtmäßigen Wahlausgang.

Aber eins nach dem Anderen. Fangen wir bei Trump an. Nachdem er ja diese Woche (und die letzten zwei Monate, und… naja, er hat halt fünf Jahre lang gelogen und eine Parallelwelt errichtet, in der er die einzige Quelle der Wahrheit ist, egal wie sehr es den Fakten widerspricht) einen Mob dazu angehetzt hat, das Kapitol in Washington zu stürmen, haben ihm Twitter, Facebook und ein paar andere Größen jetzt dauerhaft die Account gesperrt.

Ok, hier ein kurzer Rant zu der Kapitol-Geschichte, der schon seit Mittwoch heraus will: Mir fehlen immer noch die Worte um zu beschreiben, wie sehr mich diese ganze Sache aufregt. Das ist einfach auf so vielen Ebenen falsch. Allein die Ausgangsthese, dass die Wahl gestohlen sei, ist schon so oft widerlegt worden. Dann diese Wut auf den Kongress (und auf Pence, für den ich kein Mitleid habe, weil er die ganze Chose unterstützt hat, bei der Auszählung der Wahlleutestimmen aber nichts anderes machen _kann_ als sie zu zählen wie sie reinkommen), dann die ganzen Leute, die vandalierend, Verschwörungsmythen verbreitend und mit VERDAMMTEN AUSCHWITZ-PULLIS durch die Gegend laufen. Und _denen_ sagt Trump, er liebe sie alle. Und so langsam bemerken ein paar Republikaner, welchen Geist sie da gerufen haben, den sie nun nicht los werden! Jahrelang hat Trump gegen die Presse gehetzt, und der Mob vor dem Kapitol greift Journalisten an, beschimpft sie, schubst sie herum, zerstört ihre Ausrüstung. Was haben die denn erwartet? Wenn man die Presse jahrelang als Feind des Volkes bezeichnet, ihnen Unehrlichkeit und das Vertuschen eines riesigen Wahlbetrigs vorwirft. AHRG! Das ganze iölaudbcaöskdfjbhaösdu

Wie gesagt, mir fehlen die Worte. Aber nun zurück zum eigentlichen Thema.

### Monopole von sozialen Netzwerken

Trump ist also von seinem Lieblings-Demagogiekanal abgeschnitten. Und von eigentlich allen Mainstream-Onlineplattformen. Das ist deren gutes Recht. Die haben da ja schließlich quasi Hausrecht. xkcd hat das mal [schön zusammengefasst](https://xkcd.com/1357/).

Das eigentliche Problem ist also nicht, dass Trump von Twitter verbannt worden ist. Wenn ich eine Website hätte, auf der jemand ständig Lügen, Verschwörungsmythen, Hass und Hetze verbreitet, würde ich die Person vermutlich auch rauswerfen.

Das Problem ist, dass Twitter so mächtig ist, dass ein Bann einen so großen Einfluss auf den fucking _Präsidenten der USA_ hat (der, möchte man anmerken, noch über mehr Kanäle verfügt, seiner Stimme Gehör zu verschaffen als die meisten anderen Leute). Das Problem ist, dass Twitter nicht _ein_ Microbloggingdienst ist, sondern _der_ Microbloggingdienst. Das Problem ist nicht, dass Facebook _ein_ soziales Netzwerk ist, sondern _das_ soziale Netzwerk. Das Problem ist, dass sich im Internet Monopole gebildet haben.

Vor fünfundzwanzig Jahren war die Hoffnung groß, dass im Internet eine Kultur dezentraler, untereinander kompatibler Plattformen zum Informationsaustausch bilden. Beim [Usenet](https://de.wikipedia.org/wiki/Usenet) zum Beispiel hatte das schon geklappt.

Vor fünfzehn Jahren war die Hoffnung auch noch da, obwohl sich da schon einige Monopole im Internet ausbildeten. Heute hängt das Internet größtenteils von Google, Facebook und Twitter ab.

Und die haben kein Interesse daran, offene Standards und offene Schnittstellen für ihre Produkte anzubieten. Warum auch? Sie haben ja die Macht, dass die Leute zu _ihnen_ kommen, warum sollte man anderen Anbietern die Chance geben, sich zu vernetzen? Das würden den Leuten ja nur Alternativen zur eigenen Plattform bieten.

Wenn jetzt zum Beispiel Facebook beschließt, die Facebook-Daten mit den Whatsapp-Daten zusammenzuführen, kann man da nichts machen. Wenn man seine Accounts weiter verwenden will, muss man dem zustimmen. Wenn sich deine sozialen Kontakte über Facebook und WhatsApp abspielen (gerade jetzt in der Pandemie), _hast du keine echte Wahl_. Das ist die Macht, die Facebook hat.

Nun gibt es die Idee (z.B. in der EU), dass man die großen Plattformen dazu zwingen soll, offene Schnittstellen zu anderen Plattformen anzubieten. Ob das was bringt? Ich weiß es nicht. Das Kind ist meiner Meinung nach schon in den Brunnen gefallen. Aber vielleicht bessert sich damit ja die Lage. Keine Ahnung. Früher hatte Google auch eine Schnittstelle des freien IM-Protokolls XMPP (Jabber).

### Vendor Lock-In

Außerdem wurde Parler aus dem Google-Play-Store und dem Apple App-Store geworfen. Für Android kein Problem – man kann ja trotzdem noch APKs installieren, und es gibt alternative Shops. Für Apple nicht. Da kriegst du die App über den App-Store oder kannst es vergessen. Nur zur Klarstellung: Ich vergieße keine einzige Träne für Parler, aber das kann auch jeder anderen App passieren. Das ist sogar schon anderen Apps [passiert](https://arstechnica.com/tech-policy/2020/08/apple-apologizes-to-wordpress-no-longer-requires-free-app-to-add-purchases/), wobei Apple in diesem einen Fall einen Rückzieher gemacht hat. Dein iPhone gehört nicht dir, es gehört Apple.

AWS hat Parler auch gekündigt. Amazon, Google und Microsoft dominieren momentan den „cloud“-Markt. Aus Entwicklersicht ist das a so praktisch. Keinen Ärger mit Hardware, man kann es überall auf der Welt deployen, man ist viel flexibler… und man unterstützt Monopole. Man schiebt einfach ein paar Docker-Images irgendwohin, startet Docker-Container und fertig ist die Sache. Kein herumkonfigurieren mit einem Server, der bei irgendeinem speziellen Hoster steht. Um die Server kümmert sich jemand anderes. Bis dieser Andere dich loswerden will.

Wenn du bei einem klassischen Hoster gehostet bist, ist das nervig, technisch aber kein riesiges Problem. Dann ziehst du den ganzen Krams halt um.

Wenn du in der Cloud bist… schon eher ein Problem. Schlimmster Fall: Du hast Technologien verwendet, die es nur bei deinem Cloud-Anbieter gibt (message queues des entsprechenden Anbieters zum Beispiel). Dann musst du eine Menge Aufwand betreiben, um deine Software auf andere Bibliotheken umzuschreiben. Wenn das überhaupt so „einfach“ ist.

Und nehmen wir mal an, du hast daran gedacht, nur freie Technologien verwendet, die es auch bei anderen Cloud-Anbietern gibt. Dann kann es immer noch sein, dass die alle großen Cloud-Anbieter plötzlich bannen. Weil die USA dein Land mit Sanktionen belegen. Dumm gelaufen. Momentan könntest du noch auf einen der klassischen Hoster wechseln. Aber die werden, wenn das so weitergeht, früher oder später vom Markt verdrängt. So funktionieren Monopole halt.

### Alternativen

Es _gibt_ Alternativen. Freie, standardisierte, dezentrale Alternativen. So zum Beispiel das [Fediverse](https://de.wikipedia.org/wiki/Fediverse), als Gegenstück zu Twitter. Da kann man auf dezentralen, untereinander vernetzten Servern microbloggen. Man kann als Server auch einzelne andere Server blocken, zum Beispiel wenn die zu viel Hetze und Hass verbreiten (das hat noch eigene Probleme, wie ein [Talk](https://media.ccc.de/v/rc3-857362-die_rosarote_brille_des_fediverse) von der rC3 erläutert).

Es gibt dezentrale soziale Netwerke als Alternativen zu Facebook. Es gibt [Jabber](https://de.wikipedia.org/wiki/Extensible_Messaging_and_Presence_Protocol) und [Matrix](https://de.wikipedia.org/wiki/Matrix_(Kommunikationsprotokoll)) als Alternativen zu WhatsApp. Man muss sie nur benutzen.

Und da ist das Problem: Warum soll ich eine Microblogging-Infrastruktur benutzen, die viel zu wenige Leute benutzen, wenn ich auf Twitter eine viel größere Reichweite habe? Warum soll ich ein dezentrales soziales Netzwerk verwenden, wenn alle meine Freunde auf Facebook sind? Warum soll ich Jabber oder Matrix verwenden, wenn dort niemand ist, dem ich schreiben möchte?

Vielleicht helfen hier verpflichtende Schnittstellen für die großen Anbieter wirklich. Vielleicht auch nicht. Probieren kann man es ja mal. Solange es diese nicht gibt, mache ich es weiter wie bisher: mühsame Überzeugungsarbeit bei Freunden und Verwandten, kein Facebook- oder Whatsapp-Account für mich.
