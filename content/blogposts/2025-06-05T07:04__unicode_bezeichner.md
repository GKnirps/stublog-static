---
title: Unicode-Bezeichner in C und C++
filename: unicode_bezeichner
date: 2025-06-05T07:04:44+02:00
update-date:
tags: C, C++, programmieren, unicode
category: hoellenmaschinen
summary: Man darf in C und C++ auch bestimmte nicht-ASCII Code Points als Bezeichner verwenden.
image:
image-alt:
language:
---

Vor ein paar Tagen habe ich herausgefunden, dass man für Bezeichner in C und C++ auch nicht-ASCII Unicode Code Points verwenden darf. Mit Einschränkungen.

Es ist leider ein bisschen schwierig, definitive Informationen dafür zu finden. Für C++ sieht die Lage recht eindeutig aus: auf cppreference.com gibt es [eine Liste von Zeichen, die in in Bezeichnern](https://en.cppreference.com/w/cpp/language/identifiers.html) die am Anfang oder nicht am Anfang für Bezeichner zugelassen sind.

Für C habe ich keine so schöne Auflistung gefunden. Im [Wikipedia-Artikel über C](https://en.wikipedia.org/wiki/C_(programming_language)#C99) steht, dass ab C99 Escape-Sequenzen für Unicode in Bezeichnern zugelassen ist und dass auch vorgeschlagen wird, direkte Unicode-Code Points zuzulassen. Was genau erlaubt ist steht dort nicht.

Ich habe versucht, in den ISO-Standard zu schauen. Um darauf zuzugreifen müsste ich aber ordentlich blechen. Vielleicht kriege ich den auch irgendwo kostenlos zu sehen, aber die Mühe war mir diese Spielerei nicht wert. Auf gnu.org gibt es auch ein paar [Anmerkungen zu Unicode in Bezeichnern](https://www.gnu.org/software/c-intro-and-ref/manual/html_node/Identifiers.html), aber Details sind hier auch nicht gegeben.

Für's Erste nehme ich also an, dass die Regeln die gleichen sind wie bei C++.

### Beispiele

Ich habe das natürlich ausprobiert, sowohl für C als auch für C++. Zuerst der C-Code:

```
#include <stdio.h>

int main(int argc, char **argv) {
    int äöü = 42;
    float π = 3.14;
    char 茶[] = "tea";

    printf("äöü: %d\n", äöü);
    printf("π: %f\n", π);
    printf("茶: %s\n", 茶);
    printf("茶: %s\n", \u8336);

    return 0;
}
```

Ein vergleichbares C++-Programm:

```
#include <iostream>
#include <string>

int main(int argc, char** argv) {
    int äöü = 42;
    float π = 3.14;
    std::string 茶 = "tea";

    std::cout << "äöü: " << äöü << std::endl;
    std::cout << "π: " << π << std::endl;
    std::cout << "茶: " << 茶 << std::endl;
    std::cout << "茶: " << \u8336 << std::endl;

    return 0;
}
```

Ich kann deutsche Umlaute verwenden, griechische Buchstaben und auch chinesische Schriftzeichen. Ich kann auch Escape-Sequenzen anstelle der Code Points verwenden: `\u8336` ist das Gleiche wie `茶`.

Nicht erlaubt sind z.B. `☺` oder `🏴‍☠️` (die Piratenflagge sind auch nicht nur ein, sondern vier Code Points, aber das ist eine andere Geschichte).

Nun ist es leider so, dass historisch betrachtet die verschiedenen C- und C++-Compiler trotz eines einheitlichen Standards immer wieder leicht unterschiedliche Meinungen dazu hatten, was erlaubt ist und was nicht. Ich habe den code mit gcc 13.3.0 und clang 18.1.3 getestet, bei beiden lief er ohne Probleme.

### Sicherheitsimplikationen

Erst im Nachhinein ist mir aufgefallen, dass das auch Sicherheitsimplikationen hat. Ich hatte ja neulich schon über [Homograph-Attacken im Quellcode](/blogposts/curl_homoglyph_detection) geschrieben. Da ging es aber mehr um Stringkonstanten als um Bezeichner. Bei den Bezeichnern ist weiter eingeschränkt, was erlaubt ist, aber vielleicht gibt es ja trotzdem Homoglyphen.

Der Schutz vor solchen Attacken dürfte ähnlich sein wieder Schutz vor Homograph-Attacken. Vermutlich ein bisschen einfacher, weil es einfacher ist, nicht-ASCII in Bezeichnern zu verbieten als in Stringkonstanten.

### Fazit

Nicht-ASCII-Unicode in Bezeichnern in Programmiersprachen ist eine nette Sache. Ich würde aber in keinem Projekt, in dem ich etwas zu sagen hätte, welche zulassen. Weniger wegen der Sicherheitsbedenken (darauf muss man sowieso achten) sondern aus praktischen Gründen. Sehr oft arbeitet man mit Menschen aus unterschiedlichen Ländern zusammen. Was ist mit denen, die keine Umlaute auf der Tastatur haben? Müssen die die Variablennamen immer kopieren? Vielleicht die Compose-Key verwenden? Noch schwieriger wird es mit griechischen Buchstaben wie π oder chinesischen Zeichen wie 茶. Wie soll man die eintippen?

Umgekehrt: welchen Vorteil hat der Einsatz von nicht-ASCII-Unicode? Ok, man kann die Konstante für π als griechischen Buchstaben schreiben. Aber man löst damit kein wirklich bestehendes Problem. Also lasst es lieber bleiben und betrachtet das hier als Kuriosum. 
