---
title: Name: Ungültig
filename: name_ungueltig
date: 2024-11-25T17:05:01+01:00
update-date:
tags: utf-8, unicode, rc3
category: weiteweitewelt
summary: Jan Stępień hat eine Website mit Screenshots von Websites die behaupten, sein Name sei nicht gültig.
image:
image-alt:
language:
---

Manche Dinge sind ja leider ein Dauerbrenner. So zum Beispiel das, was Websites als „gültige“ Namen akzeptieren. Da wird dann Menschen einfach gesagt, ihr Name sei ungültig. Jemand namens Jan Stępień hat jetzt [eine Website mit Screenshots angelegt, die das über seinen Namen behaupten](https://wtf-8.stępień.com/).

Weitere Resourcen zu diesem Thema:

- [Falsehoods programmers believe about names](https://www.kalzumeus.com/2010/06/17/falsehoods-programmers-believe-about-names/)
- [Your Name Is Invalid](https://media.ccc.de/v/rc3-channels-2020-77-your-name-is-invalid-), ein Talk auf der rc3 von Miroslav Šedivý, der mit demselben Problem zu kämpfen hat
- [dieser alte Blogpost über kaputte Software in Behörden](/blogposts/old_1757972)
- [I Can Text You A Pile of Poo, But I Can’t Write My Name ](https://modelviewculture.com/pieces/i-can-text-you-a-pile-of-poo-but-i-cant-write-my-name) ein Artikel über die Unvollständigkeit von Unicode
- [The Absolute Minimum Every Software Developer Absolutely, Positively Must Know About Unicode and Character Sets (No Excuses!)](https://www.joelonsoftware.com/2003/10/08/the-absolute-minimum-every-software-developer-absolutely-positively-must-know-about-unicode-and-character-sets-no-excuses/), ein Artikel von 2003 über Textencoding

Wie der letzte Eintrag auf der Liste schon zeigt: Vor zwanzig Jahren gab es schon keine Entschuldigung mehr, wenn dein Code nicht mit Unicode klarkommt. Vor dreißig Jahren vielleicht noch, vor zwanzig Jahren nicht.

Wie übrigens „Falsehoods programmers believe about names“ und „I can text you a pile of poo but I can't write my name“ zeigen, sind selbst mit perfekter Unicodeunterstützung nicht alle Fälle abgedeckt. Z.b. weil die Person keinen Namen hat, oder sich der Name in Unicode nicht korrekt schreiben lässt. Die Lösung ist einfach: Macht das Namensfeld optional. In den allermeisten Fällen braucht man nicht _unbedingt_ einen Namen. Es hilft oft, z.B. damit ein Paket richtig ankommt, aber meist gibt es auch andere Optionen.
