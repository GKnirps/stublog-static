---
title: Mit C von hinten durch die Brust ins Auge
filename: umstaendli_c_h
date: 2022-08-24T19:00:35+02:00
update-date:
tags: C, rust, programmieren, komplexität
category: hoellenmaschinen
summary: Über die Umstände, die man in C hat um Dinge zu tun, die in anderen Programmiersprachen ganz einfach sind.
image:
---

Die Programmiersprache C hat ja einen besonderen Platz in meinem Herzen. Sie war die erste Programmiersprache, die ich wirklich gelernt habe. Sie ist schnell, maschinennah, man kann eine Menge über die Funktionsweise von Computern lernen, wenn man Programme in C schreibt. Zum Beispiel, warum Felder mit dem Index `0` beginnen.

Ich will auch nicht sagen, dass man jede in C geschriebene Software in rust neu schreiben soll. Das ist in den meisten Fällen [nicht praktikabel](https://www.joelonsoftware.com/2000/04/06/things-you-should-never-do-part-i/), ob jetzt in rust oder in irgendeiner anderen Sprache. Außerdem macht man sich unbeliebt, wenn man uneingeladen irgendwo auftaucht und einen Haufen C-Entwickler dazu auffordert, all ihren Krams umzuschreiben.

Es gibt Fälle, wo das gut funktioniert hat [librsvg](https://mail.gnome.org/archives/desktop-devel-list/2017-January/msg00001.html) zum Beispiel.

Eine bessere Idee ist wohl eher, einzelne, neue Bestandteile, in rust zu schreiben. So wie [im Linux-Kernel](https://www.zdnet.com/article/linus-torvalds-on-where-rust-will-fit-into-linux/) zum Beispiel. Oder wie es [bei Firefox](https://wiki.mozilla.org/Quantum) gemacht wird.

Aber trotz allem sollte man sich es wohl verdammt noch einmal gut überlegen, ob man ein neues Projekt wirklich in C schreiben möchte.

### C-Code am Beispiel Open5GS

Ich bin neulich über [Open5GS](https://open5gs.org/) gestolpert. Eine freie Implementierung unter anderem des 5G Core-Networks. 

An sich eine schöne Sache. AGPL-Lizensiert. Soweit ich das anhand der wenigen Stellen, die ich mir angeschaut habe, beurteilen kann, recht ordentlich geschrieben.

Ich habe mir den Code angeschaut, weil ein Service (die Network Repository Function, NRF) abgeschmiert ist, als ich eine völlig normale Anfrage gestellt habe.

Dass die NRF abgeschmiert ist, liegt an einer fehlgeschlagenen assertion. Auch hier zumindest in der Hinsicht ordentlich programmiert, sonst hätte es einen Segfault durch eine null pointer dereference gegeben.

Hier geht es nicht um diesen Crash. Hier geht es darum, was ich gefunden habe, als ich herausfinden wollte, warum dieser pointer überhaupt `NULL` ist. Dabei bin ich ein wenig vom Weg abgekommen.

#### Generische doppelt-verkettete Listen in C

Dabei bin ich über eine Implementierung einer [doppelt verketteter Liste](https://de.wikipedia.org/wiki/Liste_(Datenstruktur)#Doppelt_verkettete_Liste) gestoßen. Die Implementierung sieht ganz ordentlich aus, aber… die Anzahl an merkwürdigen Konstrukten, die notwendig waren, um diese Liste stabil, sicher und benutzbar zu implementieren ist Wahnsinn.

Fangen wir mal am Anfang an. Die ganze Implementierung der Liste ist in der Datei `lib/core/ogs-list.h` in [diesem Repo](https://github.com/open5gs/open5gs) zu finden.

##### Generische Definition

Die Definition des Listen-structs sieht wie folgt aus:

```
struct ogs_list_s {
    struct ogs_list_s *prev, *next;
};
typedef struct ogs_list_s ogs_list_t;
typedef struct ogs_list_s ogs_lnode_t;
```

Erst mal alles, was eine doppelt verkettete Liste braucht. Einen Vorwärtspointer und ein Rückwärtspointer. Dazu noch ein paar `typedef`s um die Benutzung semantisch klarer zu machen.

Aber irgendwas fehlt. Ach ja. Eine Liste ist recht sinnlos, wenn es keinen Payload gibt. Aber wo ist der Payload?

Hier kommen wir zum ersten Problem: Wenn man in C eine Liste generisch implementieren möchte, hat man ein Problem: C hat keine generischen Typen. Also muss man das irgendwie umgehen.

Das könnte man mit einem `void*`-pointer lösen. Ist aber aus mehreren Gründen doof, auf die ich hier jetzt nicht eingehen werde.

Die Lösung die hier gewählt wurde, ist nicht ganz offensichtlich. Angenommen, ich würde jetzt einen Typ definieren, der in der Liste stecken soll. Dann würde ich etwa so etwas machen:

```
struct something {
    ogs_lnode_t lnode;

    int payload;
};
```

Aber… die `prev` und `next` pointer gehen doch wieder nur auf die `ogs_lnode_t`! Die hat doch wieder keine Payload! Wie geht man damit um?

Nun, schauen wir uns doch mal die Funktion an mit der man das nächste Glied in der Kette bekommt:

```
static ogs_inline void *ogs_list_next(void *lnode)
{
    ogs_list_t *node = lnode;
    return node->next;
}
```

Ok, hier kommt ein void-pointer rein, ein void-pointer raus, und innendrin wird der void-pointer zu einer `ogs_list_t` gecasted und der `next`-pointer zurückgegeben.

Um dann über die Liste zu iterieren, gibt es diese bequeme Makro:

```
#define ogs_list_for_each(list, node) \
    for (node = ogs_list_first(list); (node); \
        node = ogs_list_next(node))
```

Das kann man dann einfach so verwenden:

```
struct something* some_list;
// [… create list here]

struct something* entry;
ogs_list_for_each(some_list, entry) {
    // do something with an entry
}
```

Der Vorteil für den Aufrufer hier ist, dass es typsicher ist. Man bekommt nur Einträge vom Typ `*something`. Der Nachteil ist… what the fuck.

Ich meine, ist das überhaupt definiertes Verhalten? Wer garantiert mir, dass mir nicht gleich [Dämonen aus der Nase fliegen](http://catb.org/jargon/html/N/nasal-demons.html)? Wir greifen hier immerhin auf Elemente eines structs zu, das in einem anderen struct steckt, über einen pointer auf das äußere struct. Was ist, wenn hier irgendwelches komisches Padding alles durcheinanderbringt?

Die Lage scheint [nicht ganz einfach](https://stackoverflow.com/questions/66806198/c-is-accessing-initial-member-of-nested-struct-using-pointer-cast-to-outer-st), aber es sieht so aus, als ob es hier definiertes Verhalten ist.

Aber es ist doch schon sehr von hinten durch die Brust ins Auge. Man muss einiges an komischen Dingen anstellen, damit die Liste benutzbar ist. Und offensichtlich ist nicht, wie es funktioniert.

##### Jede Menge Makros

Doch damit hört es nicht auf. Auffällig ist, dass über die gesamte Datei verteilt sehr viele Makros verwendet werden.

Nun sind C-Makros so eine Sache. Der C-Präprozessor ist unglaublich dumm, deswegen ist es möglich mit ihm so typunabhägige Sachen zu machen, weil die Typen erst später vom Compiler überprüft werden.

Deswegen sollte man Makros immer doppelt und dreifach absichern. Beispiel: Initialisierung einer Liste:

```
#define ogs_list_init(list) do { \
    (list)->prev = (NULL); \
    (list)->next = (NULL); \
} while (0)
```

Wozu zur Hölle ist diese do-while-Schleife da? Sie wird immer nur ein Mal durchlaufen und vermutlich vom Compiler wegoptimiert.

Grund dafür ist der dumme Präprozessor. Angenommen, unser Makro sähe nur so aus:

```
#define ogs_list_init(list) \
    (list)->prev = (NULL); \
    (list)->next = (NULL);
```

Dann würde folgender code

```
if (foo) ogs_list_init(list);
```

wie folgt expandiert:

```
if (foo)
    (list)->prev = (NULL);
    (list)->next = (NULL);;
```

Das ist gleichbedeutend mit

```
if (foo) {
    (list)->prev = (NULL);
}
(list)->next = (NULL);
```

Ein Teil wird also _immer_ ausgeführt, unabhängig von der Bedingung.

### Komplexität

Versteht mich bitte nicht falsch. Die Entscheidungen, den Code so zu schreiben ist vernünftig. Vernünftig _unter den Bedingungen, mit denen man arbeiten muss, wenn man C schreibt_.

Es geht hier um Komplexität, die überwunden werden muss, um das zu tun, was man eigentlich machen will. An Stellen wie diesen wünsche ich mir eine gute deutsche Version des englischen Ausdrucks „jumping through hoops“.

Es gibt beim Programmieren Komplexität, die man nie los wird, weil das Problem, das man lösen will, einfach entsprechend komplex ist. Wenn man eine verkettete Liste haben möchte, braucht man z.B. Vorwärts- und Rückwärtsverweise.

Dann gibt es Komplexität, die die Sprache vorgibt, für die man aber etwas zurück bekommt. Das ganze Ownership-Konzept in rust zum Beispiel. Man muss sich an halbwegs komplexe Regeln halten, kriegt dafür aber vom Compiler einige Garantien, die das Leben sehr viel einfacher machen.

Als drittes gibt es Komplexität, die einfach unnötig ist. Zum Beispiel, dass man Code in Makros in do-while-Schleifen hängen muss, um sicherzustellen, dass nichts Dummes passiert.

C mag eine einfach aufgebaute Sprache sein, aber aus dieser scheinbaren Einfachheit entsteht solche komplexen Konstrukte wie oben. Für Bequemlichkeiten und Sicherheiten, die die meisten modernen Sprachen von Haus aus mitliefern, in der einen oder anderen Form.

Die ganze Energie, die investiert wird, um diese unnötige Komplexität zu überwinden könnte man viel besser woanders investieren.

Ich mag C, aber eine Sprache, die mich zu solchen Konstrukten zwingt, um ein bisschen Sicherheit und Bequemlichkeit zu bekommen, ist keine Sprache, in der ich entwickeln möchte.
