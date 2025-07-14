---
title: Gleich ungleich Gleich
filename: gleich_ungleich_gleich
date: 2025-07-14T22:30:00+02:00
update-date:
tags: programmieren, C, C++, rust, c-sharp, golang, python, java, javascript
category: hoellenmaschinen
summary: Ich vergleiche Vergleichsoperationen in verschiedenen Programmiersprachen
image:
image-alt:
language:
---

Zwei Werte auf Gleichheit prüfen ist etwas, dass man beim Programmieren an jeder Ecke braucht. Die meisten Programmiersprachen verhalten sich hier zumindest bei einfachen Datentypen wie z.B. Ganzzahlen ganz so, wie man es erwarten würde (über Fließkommazahlen werde ich in diesem Artikel nicht viel reden, die sind in dieser Hinsicht kompliziert.

Was komplexe Datentypen angeht (Arrays, Strings oder zusammengesetzte Typen wie Structs oder Klassen) verhalten sich die verschiedenen Programmiersprachen allerdings sehr unterschiedlich. Es fängt schon damit an, dass verschiedene Sprachen verschiedene Typen als „einfache“ Typen (oder wie immer das im Jargon der jeweiligen Sprache heißt, das geht von „primitive types“ über „basic types“ bis zu „built-in types“) bezeichnen. Für manche sind Strings zum Beispiel primitive Typen die man einfach vergleichen kann. Für andere sind Strings Klassen, die man aber wie einfache Typen auch vergleichen kann. Für wieder andere kann man nicht mehr den Gleichheitsoperator benutzen sondern muss eine besondere Funktion aufrufen.

Letztere Sprachen sind häufig die, die recht willkürlich zwischen „reference types“ und „primitive types“ unterscheiden. Für die einen kann man den Gleichheitoperator (meist `==`) verwenden, für die anderen eine `equals`-Funktion in irgendeiner Form.

Sowohl der Gleichheitsoperator als auch die `equals`-Funktion können in verschiedenen Sprachen einen Default haben. Dann kann man die Operation zwar verwenden, oft vergleicht sie aber nur die Identität zweier Variablen (ist es _dieselbe_ Variable, nicht _der gleiche_ Wert). Manche Programmiersprachen haben auch keinen einheitlichen Standard für Gleichheitsoperationen. Man kann natürlich eine selbst definierte Funktion zum Vergleichen schreiben, die arbeitet dann aber nicht zwingend mit anderen Funktionen oder Datenstrukturen zusammen, die eine Gleichheitsoperation benötigen.

Ich möchte hier einmal eine Hand voll Programmiersprachen vergleichen (hah!). Ich stelle mir insbesondere folgende Fragen

- Für welche Datentypen ist ein echter Vergleich Teil der Sprache?
- Wird ein Unterschied zwischen Typen gemacht, bei denen man den Gleichheitsoperator verwenden kann und jenen die eine Equals-Funktion brauchen?
- Gibt es Defaultimplementierungen, die unerwarteter Weise auf Identität, nicht auf Gleichheit überprüfen?
- Ist es möglich, und wenn ja wie einfach ist es, eigene Gleichheitsoperationen zu definieren?

Eine Anmerkung noch: Üblicherweise erfordern Gleichheitsoperationen drei Eigenschaften: Sie müssen *symmetrisch* sein: `a = b ⇔  b = a`, *transitiv*: `a = b ∧ b = c ⇒ a = c` und *reflexiv*: `a = a`. Beim Programmieren ist das vor allem wichtig, wenn sich andere Codeteile auf diese Eigenschaften verlassen, weil es sonst zu Logikfehlern kommen kann. Insbesondere [Gleitkommazahlen](https://de.wikipedia.org/wiki/Gleitkommazahl), heutzutage üblicherweise nach dem Standard [IEEE 754](https://de.wikipedia.org/wiki/IEEE_754) implementiert, verletzen die letzte Eigenschaft: den `NaN` („Not a Number“) ist ungleich `NaN`. Aber Fließkommazahlen sollte man wegen Rundungsfehlern sowieso nie direkt vergleichen.

Außerdem gibt es noch einige weitere Ansprüche, die manche Sprachen stellen. Ungleichheit zum Beispiel muss konsistent mit Gleicheit sein: `a ≠ b ⇔  ¬(a = b)`. Oft wird auch gefordert, dass eine Hashfunktion für einen Wert konsistent mit der Gleichheitsoperation ist: `a = b ⇒ hash(a) = hash(b)`. Letzteres ist insbesondere in [Hashmaps](https://de.wikipedia.org/wiki/Hashtabelle) wichtig.

Aber genug Gerede, schauen wir uns mal ein paar Sprachen an. Fangen wir an mit C, denn C ist recht low-level und recht alt.

### C

Ich fange bei jeder Sprache mit derselben Baseline an: Vergleichen von Ganzzahlen:

```
int a = 1;
int b = 2;
int c = 1;
printf("a == b: %d, a == c: %d\n", a == b, a == c);
```

Der Output sieht so aus (in C ist `0` unwahr und alles andere wahr):

```
a == b: 0, a == c: 1
```

So weit keine Überraschungen. C hat auch Pointer, also Variablen, die eine Adresse auf einen Speicherbereich haben. Auch die kann man vergleichen, und es ist so etwas wie ein Identitätsvergleich (ich packe jetzt mal den output direkt im Kommentar unter den Codeblock, um es übersichtlicher zu halten):

```
int *ap = &a;
int *cp = &c;
printf("ap == ap: %d, ap == cp: %d\n", ap == ap, ap == cp);
printf("*ap == *cp: %d\n", *ap == *cp);
// ap == ap: 1, ap == cp: 0
```

`ap` zeigt auf `a`, `cp` zeigt auf `c`. Obwohl der Wert von `a` und `c` gleich ist, sind `ap` und `cp` verschieden. Auch hier keine Überraschungen. Aber schauen wir uns mal Arrays an:

```
int d[] = {1,2};
int e[] = {1,2};
printf("d == e: %d, memcmp(d, e) == 0: %d\n", d == e, memcmp(d, e, 2 * sizeof(int)) == 0);
// d == e: 0, memcmp(d, e) == 0: 1
```

Array-Variablen sind im Prinzip nur ein Pointer auf das erste (nullte) Element des Arrays. Deswegen funktioniert der direkte Vergleich nicht. Man muss eine Funktion benutzen, um sie zu vergleichen, wie hier `memcmp` aus der Standardbibliothek. `memcmp` liefert `0` zurück, wenn die Speicherbereiche gleich sind. Bei `memcmp` muss man aber vorsichtig sein, dass man die Größe des Arrays korrekt angibt.

Auch hier keine große Überraschung, C ist halt recht nah an der Maschine dran. Aber es fängt schnell an, aufwändig zu werden, mit diesen Datentypen zu hantieren. Wie sieht es mit Strings aus?

```
char f[] = "foobar";
char g[] = "foobar";
printf("f == g: %d, strcmp(f, g) == 0: %d\n", f == g, strcmp(f, g) == 0);
// f == g: 0, strcmp(f, g) == 0: 1
```

Strings in C sind nichts Anderes als `char`-Arrays, die ein `0`-Byte am Ende haben. `strcmp` vergleicht Strings, braucht aber keine Längenangabe, weil es sich auf das Nullbyte verlässt (aber wehe, wenn das nicht da ist).

Aber wie sieht es mit komplexeren Datentypen aus? C hat hier hauptsächlich `struct` zu bieten (und `union`, aber wir reden nicht über `union`):

```
typedef struct {
    int a;
} Foo;

// … irgendwo später:

Foo h = { .a = 42 };
Foo i = { .a = 42 };
printf("h == i: %d\n", h == i);
```

Nope, hier beschwert sich der Compiler: `error: invalid operands to binary == (have ‘Foo’ and ‘Foo’)`. Geht also nicht. Ich sehe das Positiv: anstatt hier einfach irgendwas zu machen sagt C hier: „nein, das geht nicht“. `memcmp` kann man hier aber auch verwenden:

```
printf("memcmp(h, i) == 0: %d\n", memcmp(&h, &i, sizeof(Foo)) == 0);
// memcmp(h, i) == 0: 1
```

Prima. Aber macht das auch einen tiefen Vergleich?

```
typedef struct  {
    int a;
    int *b;
} Foopar;

// irgendwo später:

Foopar l = {.a = 42, .b = &a};
Foopar m = {.a = 42, .b = &b};
Foopar n = {.a = 42, .b = &c};
printf("memcmp(l, m) == 0: %d, memcmp(l, n) == 0: %d\n", memcmp(&l, &m, sizeof(Foopar)) == 0, memcmp(&l, &n, sizeof(Foopar)) == 0);
// memcmp(l, m) == 0: 0, memcmp(l, n) == 0: 0
```

Nein, natürlich nicht. Aber auch das sollte keine Überraschung sein. `memcmp` vergleicht Speicherbereiche und folgt dabei natürlich keinen Pointern. Dafür müsste man sich selber eine Funktion schreiben.

#### Fazit

C ist simpel aufgestellt, bietet keine Komfortfunktionen, leitet hier aber auch nicht in die Irre. Alles ist umständlich, aber angesichts der Maschinennähe nicht verwunderlich. Positiv muss ich hervorheben, dass zumindest mein Compiler (gcc) einige Warnungen herausgehauen hat, als ich versucht habe, Variablen mit sich selbst zu vergleichen oder Arrays mit dem `==`-Operator. Ich nehme C jetzt mal als Baseline, um andere Sprachen zu bewerten. Zum Beispiel C++.

### C++

C++ ist eine auf C basierende, objektorientierte Sprache. C++ übernimmt viel von C, setzt eine Menge oben drauf, hat aber auch ein paar Unterschiede. Aber wir sind für eine Sache hier: Vergleichsoperatoren. Die einfachen Typen verhalten sich hier wir in C, deswegen fangen wir gleich mal mit Strings an:

```
string a = "foo";
string b = "bar";
string c = "foo";

cout << "a == a: " << (a == a) << ", a == b: " << (a == b) << ", a == c: " << (a == c) << endl;
// a == a: 1, a == b: 0, a == c: 1
```

Das funktioniert ja viel besser als in C. `string` ist hier übrigens kein eingebauter Typ, sondern eine Klasse (so etwas wie ein `struct` aus C, aber auf Steroiden) aus der Standardbibliothek. Man kann ja viel über objektorientierte Programmierung streiten, aber manche Features, die Klassen in C++ bieten, sind schon sehr hilfreich. Probieren wir es mal aus:

```
class Unfoo {
    private:
        const int a;
    public:
        Unfoo(int a): a(a) {}
};

// …weiter unten:
vector<Unfoo> o = {Unfoo(1)};
cout << "o == o: " << (o == o) << endl;
```

Ups… das geht nicht Das ist nicht einmal mit sich selbst vergleichbar. Das gibt einen Compilerfehler:

```
error: no match for ‘operator==’ (operand types are ‘const Unfoo’ and ‘const Unfoo’)
```

Dazu eine _lange_ Liste von Hinweisen, die wohl hilfreich sein sollen, aber erst einmal verwirren. Aber immerhin: Es wird nicht einfach irgendwie verglichen, wir müssen das selber definieren.

```
class Foo {
    private:
        const int a;
    public:
        Foo(int a) : a(a) {}
        bool operator==(const Foo& right) const {
            return this->a == right.a;
        }
        int get_a() const {
            return this->a;
        }
};

// … weiter unten

Foo d(42);
Foo e(9001);
Foo f(42);
cout << "d == d: " << (d == d) << ", d == e: " << (d == e) << ", d == f: " << (d == f) << endl;
// d == d: 1, d == e: 0, d == f: 1
```

Wunderbar! Es funktioniert. Man muss ein bisschen vorsichtig sein: Wenn man `Foo` später um weitere Membervariablen erweitert, dann muss man auch die `operator==`-Methode anpassen.

Man kann natürlich auch tiefe Vergleiche machen (wenn man sie entsprechend implementiert) oder unterschiedliche Typen vergleichen (sofern das Sinn ergibt):

```
class Foop {
    private:
        const int *const a;
        Foop& operator=(Foop& original) = delete;
    public:
        Foop(int a) : a(new int(a)) {}
        bool operator==(const Foop& right) const {
            return *this->a == *right.a;
        }
        Foop(const Foop& original) : a(new int(*a)) {}
        ~Foop() {
            delete this->a;
        }
        int get_a() const {
            return *this-> a;
        }
};

bool operator==(const Foop& left, const Foo& right) {
    return left.get_a() == right.get_a();
}

bool operator==(const Foo& left, const Foop& right) {
    return left.get_a() == right.get_a();
}

// …weiter unten

Foop g(42);
Foop h(9001);
Foop i(42);
cout << "g == g: " << (g == g) << ", g == h: " << (g == h) << ", g == i: " << (g == i) << endl;
cout <<"d == g: " << (d == g) << ", g == d: " << (g == d) << endl;
// g == g: 1, g == h: 0, g == i: 1
// d == g: 1, g == d: 1
```

Funktioniert, ist aber eine Menge Handarbeit. Ich musste den Vergleich zwischen `Foo` und `Foop` zwei Mal implementieren, damit der auch symmetrisch ist. Dafür hat man eine Gleichheitsoperation, die angenehm zu benutzen ist und überall gleich aussieht. Die Standardbibliothek nutzt das viel. Zum Beispiel mit Containertypen wie `vector`, einer Klasse für dynamisch wachsende Arrays. Wichtig ist nur, dass die Typen im Container den Operator implementiert haben:

```
vector<Foo> k = {Foo(1)};
vector<Foo> l = {Foo(1), Foo(2)};
vector<Foo> m = {Foo(1)};
cout << "k == l: " << (k == l) << ", k == m: " << (k == m) << endl;
// k == l: 0, k == m: 1
```

#### Fazit

C++ bietet die Möglichkeit, für selbstdefinierte Typen den Gleichheitsoperator zu implementieren. Das ist sehr hilfreich, aber einiges an Handarbeit. C++ bietet keine automatische Implementierung, aber auch keine Default-Implementierung, die einen vielleicht überrascht. Warum ich so auf dieser Default-Implementierung herumhacke? Schauen wir uns das mal am Beispiel von Python an.

### Python

Vergleichen wir erst einmal wieder Ganzzahlen, als Baseline:

```
a = 42
b = 11
c = 42

print(f"a is a: {a is a}, a is b: {a is b}, a is c: {a is c}")
print(f"a == a: {a == a}, a == b: {a == b}, a == c: {a == c}")

// a is a: True, a is b: False, a is c: True
// a == a: True, a == b: False, a == c: True
```

Python kennt _nur_ Referenztypen. Das `is`-Keyword vergleicht hier die _Identität_, der Gleichheitsoperator `==` den Wert. Aber warum ist `a is c` wahr? das sind doch zwei verschiedene Variablen? Nun, Python ist recht ineffizient, und um die Effizienz wenigstens ein bisschen zu steigern, versucht es an manchen Stellen, Platz zu sparen und legt, wenn möglich, nur ein Objekt anstelle von zweien an. Die Zahl selber ist unveränderlich, also ist das auch kein Problem, nur halt nervig für eine Demonstration. Für den Vergleich von Strings habe ich also ein Kommandozeilenparameter verwendet, dass ich beim Aufruf des Skripts mit `foo` gefüttert habe:

```
d = "foo"
e = "bar"
f = "foo"
g = sys.argv[1]

print(f"d is e: {d is e}, d is f: {d is f}, d is g: {d is g}");
print(f"d == e: {d == e}, d == f: {d == f}, d == g: {d == g}");

// d is e: False, d is f: True, d is g: False
// d == e: False, d == f: True, d == g: True
```

Strings werden also korrekt verglichen, egal ob sie dasselbe Objekt sind oder nicht. Das ist ja schön. Mit Arrays (oder „Listen“ im Python-Jargon) und Wörterbüchern sieht es genauso aus (dieser Post wird lang, also spare ich mir mal den Codeschnipsel). Aber Python hat auch Klassen. Wie sieht das da aus?

```
class Foo:
    def __init__(self, a):
        self.a = a

l = Foo(42)
m = Foo(11)
n = Foo(42)

print(f"l is l: {l is l}, l is m: {l is m}, l is n: {l is n}")
print(f"l == l: {l == l}, l == m: {l == m}, l == n: {l == n}")
// l is l: True, l is m: False, l is n: False
// l == l: True, l == m: False, l == n: False
```

Mist. Es _gibt_ hier eine Standardimplementierung von `==`, und es ist die Identität. Das kann zu fiesen Fehlern führen. Aber immerhin _können_ wir diese Implementierung überschreiben:

```
class Bar:
    def __init__(self, a):
        self.a = a

    def __eq__(self, other):
        return self.a == other.a

o = Bar(42)
p = Bar(11)
q = Bar(42)

print(f"o is p: {o is p}, o is q: {o is q}")
print(f"o == p: {o == p}, o == q: {o == q}")

print(f"o == l: {o == l}, l == o: {l == o}")

// o is p: False, o is q: False
// o == p: False, o == q: True
// o == l: True, l == o: True
```

Prima. Dann geht das. Wir können auch `Bar` mit `Foo` vergleichen, dank [Duck-Typing](https://de.wikipedia.org/wiki/Duck-Typing). Ich bin nicht sicher, ob das gut ist. Jedenfalls ist es symmetrisch, obwohl ich nicht speziell darauf geachtet habe.

#### Fazit

Python bietet für viele eingebaute Typen eine Vergleichsfunktion an und ermöglicht es auch, die Vergleichsoperation zu überschreiben. Gefährlich wird es, wenn man das nicht tut und die Standardimplementierung (Identitätsvergleich) verwendet wird.

Zu dem Identitätsvergleich aus Versehen komme ich später noch einmal. Aber da wir gerade bei Python waren, bleiben wir mal bei Skriptsprachen und nehmen eine Sprache, die deutlich schlechter designed ist, aber trotzdem in jedem gängigen Browser läuft: Javascript

### Javascript

Zunächst einmal: Javascript hat _zwei_ Vergleichsoperatoren: `==` und `===`. Ich weiß nicht, warum. Sicher ist: Ich will hier nicht viel über `==` sprechen, denn der Operator hat einige _ungewöhnliche_ Verhaltensweisen, wenn man zwischen verschiedenen Typen vergleicht:

```
console.log(`"0" == false: ${"0" == false}`);
console.log(`"" == false: ${"" == false}`);
console.log(`"" == "0": ${"" == "0"}`);
// "0" == false: true
// "" == false: true
// "" == "0": false
```

Ups. `"0"` ist `false. `""` ist `false`. Aber `""` ist offensichtlich nicht `"0"`. Nicht transitiv. Also gehen wir lieber über zu `===`, denn hier wird immer auch der Typ überprüft:

```
const a = 42;
const b = 11;
const c = 42;

console.log(`a === a: ${a === a}, a === b: ${a === b}, a === c: ${a === c}`);
// a === a: true, a === b: false, a === c: true

const d = "foo";
const e = "bar";
const f = "foo";

console.log(`d === d: ${d === d}, d === e: ${d === e}, d === f: ${d === f}`);
// d === d: true, d === e: false, d === f: true

const g = [1,2,3];
const h = [1,2];
const i = [1,2,3];

console.log(`g === g: ${g === g}, g === h: ${g === h}, g === i: ${g === i}`);
// g === g: true, g === h: false, g === i: false

const k = {foo: 42};
const l = {foo: 11};
const m = {foo: 42};

console.log(`k === k: ${k === k}, k === l: ${k === l}, k === m: ${k === m}`);
// k === k: true, k === l: false, k === m: false
```

Uff. Strings und Zahlen kann man noch gut vergleichen (Achtung: Technisch gesehen hat Javascript keine Ganzzahlen, nur Fließkommazahlen. Vergleiche zwischen effektiven Ganzzahlen funktionieren trotzdem gut). Schon bei Arrays hört es aber auf. Objekte (Wörtberbücher/Maps) machen auch nicht mit. Hier wird jeweils nur die Identität überprüft.

Eigene Gleichheitsoperatoren, die man dann mit `===` benutzen kann, kann man nicht definieren. Man kann natürlich Vergleichsfunktionen schreiben. Über Klassen reden wir nicht. Klassen in Javascript sind _weird_ und niemand verwendet sie.

#### Fazit

Javascript vergleicht bei Arrays und Objects nur die Identität. Dieses Verhalten kann man nicht überschreiben. Man kann überhaupt keinen eigenen Vergleichsoperator definieren. Wenn der Identitätsvergleich nicht wäre, dann wäre es immerhin simpel, wenn auch unbequem. So ist es aber auch noch eine Falle. Gehen wir also weiter zum Namensvetter von Javascript, der außer dem Namen nichts mit Javascript zu tun hat: Java

### Java

```
int a = 42; 
int b = 11; 
int c = 42; 

System.out.println(String.format("a == a: %b, a == b: %b, a == c: %b", a == a, a == b, a == c));
// a == a: true, a == b: false, a == c: true
```

Die Baseline funktioniert. Wie sieht es mit Arrays aus?

```
int[] d = {1,2,3};
int[] e = {1,2};
int[] f = {1,2,3};

System.out.println(String.format("d == d: %b, d == e: %b, d == f: %b", d == d, d == e, d == f));
System.out.println(String.format("compare(d, e) == 0: %b, compare(d, f) == 0: %b", Arrays.compare(d, e) == 0, Arrays.compare(d, f) == 0));
// d == d: true, d == e: false, d == f: false
// compare(d, e) == 0: false, compare(d, f) == 0: true
```

Na toll. Bei Arrays wird auch wieder nur die Identität verglichen. Warum? Keine Ahnung. Vermutlich, weil man sich so verhalten wollte wie C. Arrays sind hier immer nur Referenzen, aber keine Klassenobjekte. Also müssen wir eine andere Vergleichsfunktion nehmen als wir für Klassen nehmen würden. Maximal unpraktisch.

Aber Strings sind Klassen. Klassen haben doch sicher eine schöne Vergleichsoperation, oder?

```
String g = "foo";
String h = "bar";
String i = "foo";
String k = args[1];
System.out.println(String.format("g == h: %b, g == i: %b, g == k: %b", g == h, g == i, g == k));
System.out.println(String.format("g.equals(h): %b, g.equals(i): %b, g.equals(k): %b", g.equals(h), g.equals(i), g.equals(k)));
// g == h: false, g == i: true, g == k: false
// g.equals(h): false, g.equals(i): true, g.equals(k): true
```

Ich habe hier denselben Trick wie bei Python angewandt, um das Caching von statischen Strings zu umgehen: einen Kommandozeilenparameter, den ich mit `foo` gefüllt habe.

Also was sehen wir hier? Der `==`-Operator vergleicht hier immer nur die Identität. Die Entwickler von Java haben hier eine Dreiklassengesellschaft (no pun intendet) eingeführt. Klassenobjekte, primitive Typen und Arrays. Nur primitive Typen werden als Wert behandelt, alles andere als Referenz. Für Arrays gibt es keine Methoden, also muss man das die `Arrays.compare()`-Funktion nehmen. Klassenobjekte vergleicht man mit der `equals()`-Methode. Aber wer verwendet schon Arrays in Java? Es gibt doch die praktische `ArrayList`-Klasse!

```
List<Integer> l = Arrays.asList(Integer.valueOf(1), Integer.valueOf(2));
List<Integer> m = Arrays.asList(Integer.valueOf(1));
List<Integer> o = Arrays.asList(Integer.valueOf(1), Integer.valueOf(2));
System.out.println(String.format("l == l: %b, l == m: %b, l == o: %b", l == l, l == m, l == o));
System.out.println(String.format("l.equals(l): %b, l.equals(m): %b, l.equals(o): %b", l.equals(l), l.equals(m), l.equals(o)));
// l == l: true, l == m: false, l == o: false
// l.equals(l): true, l.equals(m): false, l.equals(o): true
```

Immerhin. Die kann Vergleiche. Hier ist übrigens auch ein guter Grund, Arrays zu verwenden: Primitive Typen muss man in Klassen Wrappen, um sie in generischen Containerklassen zu verwenden. Es gibt einen [Vorschlag für Generics über primitive Typen](https://openjdk.org/jeps/218), aber da hat sich meines Wissens seit 2017 nicht viel getan. Aber ich schweife ab. Wie sieht es denn mit selbst definierten Klassen aus?

```
// Foo.java
public class Foo {
    final int a;

    public Foo(int a) {
        this.a = a;
    }
}

// in der main-Funktion:
Foo p = new Foo(42);
Foo q = new Foo(42);
System.out.println(String.format("p.equals(p): %b, p.equals(q): %b", p.equals(p), p.equals(q)));
// p.equals(p): true, p.equals(q): false
```

Selbes Problem wie in Python: Wenn nicht selbst implementiert, wird die `equals`-Funktion von `Object` geerbt. _Jede_ Klasse in Java erbt von `Object`. Ich habe das immer für komisch gehalten, aber hier ist es regelrecht nervig. Also definieren wir die Funktion mal selbst:

```
// Foo2.java
public class Foo2 {
    final int a;

    public Foo2(int a) {
        this.a = a;
    }

    public boolean equals(Object other) {
        if (other == null) {
            return false;
        } else if (other.getClass() != this.getClass()) {
            return false;
        }
        Foo2 foo = (Foo2) other;

        return this.a == foo.a;
    }
}

// in der main-Funktion
Foo2 r = new Foo2(42);
Foo2 s = new Foo2(11);
Foo2 t = new Foo2(42);
System.out.println(String.format("r.equals(s): %b, r.equals(t): %b, r.equals(42): %b", r.equals(s), r.equals(t), r.equals(42)));
// r.equals(s): false, r.equals(t): true, r.equals(42): false
```

Das Positive zuerst: Es funktioniert. Das Negative: Weil wir `equals` von `Object` erben und überschreiben, muss die überschriebene Funktion natürlich auch `Object` als Parameter nehmen. Wir müssen also zur Laufzeit noch einen Typ-Check vornehmen, um sicherzustellen, dass wir das `Object` zu `Foo2` casten können. Immerhin können wir hier auch mit anderen Typen vergleichen. Mit aktivierten Warnungen warnt der Compiler auch, dass wir hier `hashCode` überschreiben sollten, damit beide konsistent sind. Immerhin.

#### Fazit

Java ist kompliziert was Gleichheit angeht, hat je nach Metatyp unterschiedliche Wege, wie man vergleicht, vergleicht bei Referenztypen standardmäßig nur die Identität und macht es fummelig, den Gleichheitsoperator zu überschreiben. Ich bin froh, dass ich nicht mehr in dieser Sprache entwickeln muss. Stattdessen muss ich aber in Javas Schwestersprache entwickeln: C#

### C#

Zunächst einmal: Ich programmiere erst seit knapp zwei Wochen in C#, deswegen würde ich alle Beispiele hier mit ein bisschen Vorsicht betrachten.

C# ist auf demselben Mist gewachsen wie Java und hat an vielen Stellen dieselben Fehler gemacht. An manchen Stellen aber auch andere Fehler. Gemein haben beide Sprachen eine Liebe für objektorientierte Programmierung und Boilerplate. Also: Fangen wir an.

```
int a = 42;
int b = 11;
int c = 42;
Console.WriteLine(string.Format("a == a: {0}, a == b: {1}, a == c: {2}", a == a, a == b, a == c));
// a == a: True, a == b: False, a == c: True
```

Keine Überraschungen. Strings?

```
string d = "foo";
string e = "bar";
string f = args[1];
Console.WriteLine(string.Format("d == d: {0}, d == e: {1}, d == f: {2}", d == d, d == e, d == f));
Console.WriteLine(string.Format("ReferenceEquals(d, d): {0}, ReferenceEquals(d, e): {1}, ReferenceEquals(d, f): {2}", Object.ReferenceEquals(d, d), Object.ReferenceEquals(d, e), Object.ReferenceEquals(d, f)));
// d == d: True, d == e: False, d == f: True
// ReferenceEquals(d, d): True, ReferenceEquals(d, e): False, ReferenceEquals(d, f): False
```

Ich habe hier wieder ein Kommandozeilenparameter (mit `foo` gefüllt) genommen. Aber immerhin: Strings kann man einfach mit dem Gleichheitsoperator vergleichen. Besser als Java. `ReferenceEquals` vergleicht explizit die Identität.

Arrays habe ich mal ausgelassen (bin nicht einmal sicher, in welcher Form die in dieser Sprache existieren) und bin direkt zu den Listenklassen gegangen:

```
List<int> g = new List<int>() { 1, 2, 3 };
List<int> h = new List<int>() { 1, 2 };
List<int> i = new List<int>() { 1, 2, 3 };
Console.WriteLine(string.Format("g == g: {0}, g == h {1}, g == i {2}", g == g, g == h, g == i));
Console.WriteLine(string.Format("g.Equals(g): {0}, g.Equals(h) {1}, g.Equals(i) {2}", g.Equals(g), g.Equals(h), g.Equals(i)));
// g == g: True, g == h False, g == i False
// g.Equals(g): True, g.Equals(h) False, g.Equals(i) False
```

…und da haben wir es wieder: Listen kann man _nicht_ mit `==` vergleichen, das gibt wieder nur die Identität. Im Gegensatz zu Java funktioniert hier aber auch die `Equals()`-Funktion nicht. Also definieren wir mal selber eine Klasse und schauen, wie man die implementiert. Nein halt, erst einmal keine Klasse. C# unterscheidet zwischen `class` und `struct`. `class` wird auf dem Heap allokiert, `struct` auf dem Stack, oder kriegt einen Speicherbereich direkt und ohne Referenz in anderen Klassen und Structs. Warum man das am Typ festmachen soll und nicht dort, wo man den Typ verwendet, ist mir ein Rätsel, aber so ist es halt.

```
public struct Foos
{
public Foos(int a)
{
    A = a;
}
public int A { get; }
}

// woanders

Foos j = new Foos(42);
Foos k = new Foos(11);
Foos l = new Foos(42);
Console.WriteLine(string.Format("j == k: {0}, j == l: {1}", j == k, j == l));
```

Halt, nein, das gibt einen Compilerfehler: `error CS0019: Operator '==' cannot be applied to operands of type 'Foos' and 'Foos'`. Immerhin. Liegt aber leider daran, dass das hier kein Referenztyp ist. Equals hingegen vergleicht ordentlich, auch ohne zu überschreiben:

```
Console.WriteLine(string.Format("j.Equals(k): {0}, j.Equals(l): {1}", j.Equals(k), j.Equals(l)));
// j.Equals(k): False, j.Equals(l): True
```

Bei Klassen sieht das leider schon wieder anders aus:

```
public class Fooc
{
    public Fooc(int a)
    {
        A = a;
    }
    public int A { get; }
}

// woanders

Fooc m = new Fooc(42);
Fooc n = new Fooc(11);
Fooc o = new Fooc(42);
Console.WriteLine(string.Format("m == m: {0}, m == n: {1}, m == o: {2}", m == m, m == n, m == o));
Console.WriteLine(string.Format("m.Equals(m): {0}, m.Equals(n): {1}, m.Equals(o): {2}", m.Equals(m), m.Equals(n), m.Equals(o)));
// m == m: True, m == n: False, m == o: False
// m.Equals(m): True, m.Equals(n): False, m.Equals(o): False
```

Keine Überraschung, sieht so aus wie bei `List`.

```
public class Fooc2
{
    public Fooc2(int a)
    {
        A = a;
    }
    public int A { get; }
    public override bool Equals(object? other)
    {
        if (other == null || !(other is Fooc2))
        {
            return false;
        }
        Fooc2 o = (Fooc2)other;
        return this.A == o.A;
    }
}

// später

Fooc2 p = new Fooc2(42);
Fooc2 q = new Fooc2(11);
Fooc2 r = new Fooc2(42);
Console.WriteLine(string.Format("p == q: {0}, p == r: {1}", p == q, p == r));
Console.WriteLine(string.Format("p.Equals(q): {0}, p.Equals(r): {1}", p.Equals(q), p.Equals(r)));
// p == q: False, p == r: False
// p.Equals(q): False, p.Equals(r): True
```

Auch hier dasselbe Spiel wie bei Java: nimmt nur `object?` entgegen, man muss also manuell den Typ überprüfen. Aber die Sprache entwickelt sich weiter. Es gibt ein Interface, mit dem man `Equals` typsicher implementieren kann. Und man kann auch den `==`-Operator überschreiben. Letzteres geht erst seit Kurzem Ich hatte hier zunächst den [Mono](https://de.wikipedia.org/wiki/Mono_(Software))-Compiler genommen, aber der kommt nur bis C#-6. Jetzt habe ich `dotnet` installiert und kann das Feature nutzen:

```
public class Fooc3 : IEquatable<Fooc3>
{
    public Fooc3(int a)
    {
        A = a;
    }
    public int A { get; }
    public bool Equals(Fooc3? other)
    {
        return other != null && this.A == other.A;
    }

    public static bool operator ==(Fooc3? left, Fooc3? right)
    {
        if (left is null)
        {
            return right is null;
        }
        return left.Equals(right);
    }

    public static bool operator !=(Fooc3? left, Fooc3? right)
    {
        if (left is null)
        {
            return right is not null;
        }
        return !left.Equals(right);
    }

    public override bool Equals(object obj)
    {
        if (ReferenceEquals(this, obj))
        {
            return true;
        }

        if (ReferenceEquals(obj, null))
        {
            return false;
        }

        throw new NotImplementedException();
    }
}

// woanders

Fooc3 s = new Fooc3(42);
Fooc3 t = new Fooc3(1);
Fooc3 u = new Fooc3(42);
Console.WriteLine(string.Format("s == t: {0}, s == u: {1}", s == t, s == u));
Console.WriteLine(string.Format("s.Equals(t): {0}, s.Equals(u): {1}", s.Equals(t), s.Equals(u)));
// s == t: False, s == u: True
// s.Equals(t): False, s.Equals(u): True
```

Erfolg! Es ist nur eine unglaubliche Menge Code. `operator!=` muss man implementieren, auch wenn der abgeleitet werden könnte. Ach ja, und as alte `Equals` wollte ich eigentlich überhaupt nicht implementieren. Das hat mir der Code-Formatter `dotnet format` mit eingeschleust. Vermutlich aus Gründen der Rückwärtskompatibilität. Aber nicht, was ich von einem Code-Formatter erwarten würde. An anderen Stellen hat er mir auch die `GetHashCode()`-Funktion eingeschmuggelt. Ohne Implementierung, und ohne mir Bescheid zu geben:

```
public override int GetHashCode()
{
    throw new NotImplementedException();
}
```

Immerhin weiß ich jetzt, warum `dotnet format` um eine paar Größenordnung langsamer ist als andere code formatter. Für dieses Mini-Projekt braucht es schon über 7 Sekunden (!). Zum Vergleich: `cargo fmt` braucht für meinen deutlich größeren Bloggenerator gerade mal etwa 470 _Millisekunden_. Um noch Bonus-Negativpunkte einzusammeln: Man muss `dotnet format` mehrmals laufen lassen, um alles hinzuzufügen. Beim ersten Durchlauf erkennt es, dass es eine `Equals`-Funktion hinzufügen muss, weil `IEquatable` implementiert ist (oder weil `==` implementiert ist? Keine Ahnung). Beim zweiten Durchlauf erkennt es dann, dass `GetHashCode` implementiert werden muss, weil `Equals` implementiert ist. Aber ich schweife schon wieder ab.

Compilerwarnung bekomme ich eine ganze Menge: Vergleich einer Variable mit sich selbst, fehlende Hashfunktion, fehlendes `Equals`. Immerhin ein Lichtblick.

#### Fazit

C# ist genau so ein Chaos wie Java, aber an anderen Stellen. Immerhin versucht es, besser zu werden schafft es aber nicht, sich dabei Altlasten zu entledigen. Wenn man die Altlasten wenigstens automatisch von den neueren Funktionen ableiten könnte… aber selbst `dotnet format` gibt nur eine mangelhafte Implementierung vor.

Jetzt hatten wir zwei alte, maschinennahe Sprachen, zwei Skriptsprachen, zwei von großen Konzernen gehypte und furchtbare Sprachen. Es folgen: Zwei moderne Sprachen, die ich beide lieber habe alle alle anderen Sprachen hier (mit Ausnahme vielleicht von Python, aber das kommt auf den Anwendungsfall an). Beginnen wir mit Go

### Go

```
var a uint32 = 42
var b uint32 = 11
var c uint32 = 42
fmt.Printf("a == a: %t, a == b: %t, a == c: %t\n", a == a, a == b, a == c)
// a == a: true, a == b: false, a == c: true
```

Bisher hat mich hier noch keine Sprache überrascht, und das ist gut.

```
d := "foo"
e := "bar"
f := "foo"
fmt.Printf("d == e: %t, d == f: %t\n", d == e, d == f)
// d == e: false, d == f: true
```

Strings lassen sich vergleichen, gut.

```
var g [2]uint32 = [2]uint32{1, 2}
// var h [3]uint32 = [3]uint32{1,2,3};
var i [2]uint32 = [2]uint32{1, 2}
// g == h: type mismatch (differently sized arrays)
fmt.Printf("/*g == h: ?*/, g == i: %t\n" /*g == h,*/, g == i)
// /*g == h: ?*/, g == i: true
```

Arrays lassen sich vergleichen, sofern sie dieselbe Länge haben. Ähnlich wie in rust erkennt Go Arrays unterschiedlicher Länge als unterschiedliche Typen an. Gleichtypige Arrays lassen sich vergleichen.

```
//var k []uint32 = []uint32{1, 2}
//var l []uint32 = []uint32{1, 2, 3}
//var m []uint32 = []uint32{1, 2}
// Compiler error: `invalid operation: k == l (slice can only be compared to nil)`
//fmt.Printf("k == l: %t, k == m: %t\n", k == l, k == m);
```

Slices lassen sich nicht direkt vergleichen, man muss das manuell machen. Genau so sieht es für `map` aus.

```
type Foo struct {
    a uint32
}

// woanders

n := Foo{a: 42}
o := Foo{a: 11}
p := Foo{a: 42}
fmt.Printf("n == o: %t, n == p: %t\n", n == o, n == p)
// n == o: false, n == p: true
```

Structs kann man vergleichen, wenn alle Typen darin vergleichbar sind. Vorsicht: Bei Pointer werden die Pointer verglichen, nicht die Werte, auf die sie zeigen:

```
type Foop struct {
    a *uint32
}

// woanders
q := Foop{a: &a}
s := Foop{a: &b}
t := Foop{a: &c}
fmt.Printf("q == q: %t, q == s: %t, q == t: %t\n", q == q, q == s, q == t)
// q == q: true, q == s: false, q == t: false
```

Selber überschreiben kann man Gleichheitsoperatoren nicht. Es gibt ein Pseudo-Interface `comparable`, das mehr oder weniger zeitgleich mit den lang ersehnten Generics eingeführt wurde (die hatte Go, ähnlich wie Java, lange Zeit nicht). [Was vergleichbar ist und was nicht](https://go.dev/ref/spec#Comparison_operators) ist relativ übersichtlich.

#### Fazit

Go hält es einfach. Einfach ist gut in der Programmierung. Eine Menge Probleme entstehen, weil Entwickler eine Komplexitätssucht entwickeln. Aber Go übertreibt es meiner Meinung nach mit der Einfachheit, so dass wiederum Probleme entstehen, die man mit ein bisschen mehr Komplexität nicht hätte. Dadurch wird es unbeabsichtlicht wieder komplizierter. Und ein paar Fallstricke gibt es trotzdem. Dennoch, obwohl man Gleichheitsoperationen nicht überschreiben kann, ist Go bisher mein Favorit. Aber mein persönlicher Goldstandard kommt erst noch:

### Rust

Ich bin natürlich voreingenommen. Rust ist halt meine Lieblingssprache. Aber ich habe gute Argumente, warum rust sich hier am besten schlägt. Meine Voreingenommenheit zeigt sich also nicht darin, dass ich rust übermäßig bevorteile, sondern daher, dass ich das Problem so gestellt habe, dass rust gut darin abschneidet.

```
let a: u32 = 42;
let b: u32 = 11;
let c: u32 = 42;
println!("a == a: {}, a == b: {}, a == c: {}", a == a, a == b, a == c);
```

Langweilig! Die nächsten Beispiele bitte im Schnelldurchlauf, ich will langsam ins Bett!

```
let d: &str = "foo";
let e: &str = "bar";
let f: &str = "foo";
let g: String = "foo".to_string();
println!("d == e: {}, d == f: {}, d == g: {}", d == e, d == f, d == g); 
// d == e: false, d == f: true, d == g: true

let h: &[u32] = &[1, 2, 3]; 
let i: &[u32] = &[1, 2]; 
let j: &[u32] = &[1, 2, 3]; 
println!("h == i: {}, h == j: {}", h == i, h == j); 
// h == i: false, h == j: true

let k: [u32; 3] = [1, 2, 3]; 
let l: Vec<u32> = vec![1, 2, 3]; 
println!("h == k: {}, h == l: {}", h == k, h == l);
// h == k: true, h == l: true
```

Strings kann man vergleichen. Egal ob string slice oder `String`. Auch untereinander. Man kann slices miteinander vergleichen (take _that_, Go!). Man kann Slices mit Arrays vergleichen. Man kann Slices mit dem `Vec`-struct vergleichen. Alles funktioniert, solange der Typ innerhalb des Containers vergleichbar ist.

```
struct Foo(u32);

// woanders:

let m = Foo(42);
let n = Foo(42);
println!("m == n: {}", m == n);
```

Nope, das gibt einen Compilerfehler: „binary operation `==` cannot be applied to type `Foo`“. Wie gesagt, ich halte das für gut. Nicht alles kann vergleichbar sein. Aber man _kann_ den das implementieren:

```
struct Fooman(u32);

impl PartialEq for Fooman {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// woanders

let m = Fooman(42);
let n = Fooman(11);
let o = Fooman(42);
println!("m == n: {}, m == o: {}", m == n, m == o);
// m == n: false, m == o: true
```

Schön. Aber das ist doch aufwändig. In den meisten Fällen sind zwei `struct`-Instanzen doch gleich, wenn alle Felder gleich sind. Dafür gibt es das `derive`-Makro:

```
#[derive(PartialEq, Eq)]
struct Fooder(u32);

// woanders
let p = Fooder(42);
let q = Fooder(11);
let r = Fooder(42);
println!("p == q: {}, p == r: {}", p == q, p == r);
// p == q: false, p == r: true
```

Erstellt die `PartialEq`-Implementierung zur Compilezeit. Funktioniert wunderbar. Und das Beste: Wenn man jetzt weitere Felder zum Struct hinzufügt, werden die automatisch mit in die Vergleichsfunktion aufgenommen. Weniger Spielraum für Fehler. Nur, wenn ein Feld in dem Struct nicht `PartialEq` implementiert, muss man manuell Hand anlegen.

`PartialEq` stellt dabei auch direkt noch den `!=`-Operator bereit. Den muss man also nicht manuell implementieren (könnte es aber, muss dann aber auf Konsistenz achten).

Eine Notiz noch am Rande: Es gibt hier zwei Traits (vergleichbar zu Interfaces in anderen Sprachen): `PartialEq` und `Eq`. `PartialEq` muss nicht reflexiv sein. `Eq` ist nur ein Marker ohne Funktionen, mit dem der Entwickler garantiert, dass die Operation auch reflexiv ist. Das heißt Gleitkommatypen (`f32`, `f64`) implementieren `PartialEq`, aber nicht `Eq`. Die meisten anderen Typen, die `PartialEq` implementieren, implementieren auch `Eq`.

#### Fazit

Rust kann alles, was die anderen Sprachen auch können, und vieles besser. Programmiererfehler werden abgefangen. Boilerplate wird reduziert. Trotzdem kann man überall Gleichheitsfunktionen haben.

### Zusammenfassung

Meine Güte, ich habe jetzt schon wieder drei Stunden an diesem Artikel geschrieben. Und das schließt nicht die Zeit ein, die ich beim Implementieren der Codebeispiele verwendet habe. Die vollständigen Codebeispiele lade ich bald auch irgendwo hoch, aber nicht mehr jetzt. Fassen wir also zusammen:

- C bietet kaum Hilfe, geht aber offen damit um
- C++ bietet Operatorüberladung und macht es ganz ordentlich, man muss aber viel selber machen
- Python hat den fiesen Identitäts-Default, man kann den Operator aber überladen und es ist einfach
- Javascript ist Javascript
- Java hat drei verschiedene Arten von Typen, die verglichen werden können, alle unterschiedlich. Objekte haben beim Vergleich den Identitäts-Default.
- C# ist so ähnlich wie Java, an einzelnen Stellen schlechter, aber mit ein paar neuen Features besser, allerdings ohne den alten Krams loszuwerden
- Go hält es simpel, aber vielleicht ein bisschen zu simpel. Man kann trotzdem gut damit arbeiten
- rust bietet Sicherheit vor Programmiererfehlern, Solide Auto-Implementierungen, keine Überraschungen, es kostet nur ein bisschen Komplexität im Verhältnis zu Go
