---
title: Golang
filename: golang_meinung
date: 2025-03-11T15:39:41+01:00
update-date:
tags: golang, programmieren
category: hoellenmaschinen
summary: Ein Vereinskamerad aus dem Kanuklub hat mich nach meiner Meinung zu Golang gefragt. Hier ist sie.
image:
image-alt:
language:
---

Neulich hat mich ein Vereinskamerad aus dem Kanuklub nach meiner Meinung zur [Programmiersprache Go](https://go.dev/) gefragt. Ich habe da nur kurz antworten können, weil ich weg musste, aber mir sind so viele Sachen eingefallen, dass ich hier noch etwas dazu schreiben möchte.

Zunächst erst einmal: Ich habe gemischte Gefühle zu Go, und die Gründe dafür werden sich hier durch den ganzen Blogpost ziehen. Ich habe mich Ende 2014 zum ersten Mal mit Go beschäftigt, und war damals begeistert. Kurze Zeit später wurde Go bei mir aber von Rust verdrängt.

Ich habe seitdem in verschiedenen Programmiersprachen gearbeitet, auch beruflich, und ich kann sagen, dass ich Go immer noch vielen Sprachen vorziehen würde, darunter Java, Javascript, je nach Anwendungsfall Python oder C oder C++ und in jedem Fall PHP (in den meisten Fällen würde ich allerdings dann eher in Rust schreiben als in Go). Aber schlüsseln wir das mal auf.

### Die Grundkonzepte

Go ist typsicher und kompiliert, viele Fehler fallen deshalb schon beim kompilieren auf. Go hat einen Garbage Collector und keine externe runtime, dafür sind Go-binaries schnell ein paar Megabyte groß. Go lässt sich gut in Container (z.B. Docker) verpacken, und zwar minimale Container, die nichts enthalten außer der Go-Binary.

Besonders an Go ist, wie selbstverständlich nebenläufige Programmierung in die Sprache eingebettet ist. Goroutinen sind leichtgewichtig zu starten (anders als Threads in den meisten anderen Sprachen), channels ermöglichen Kommunikation zwischen Goroutinen ohne mit Locks herumhantieren zu müssen (die gibt es auch, für Spezialfälle).

Go hat eine umfangreiche Standardbibliothek, so kann man zum Beispiel ohne externe Abhängigkeiten einen Webserver aufsetzen, inklusive HTML-Templates und JSON-Serialisierung/Deserialiserung von und in Go-structs.

### Tooling

Go hat schon standardmäßig einiges an Tools. Da ist zum Beispiel `gofmt`, um Quellcode zu formatieren. `gofmt` ist opinionated, hat also genaue Vorstellungen davon, wie gut formatierter Code auszusehen hat und lässt sich nur schwer davon abbringen. Ich zähle das als positiven Aspekt. Dadurch hat jedes Go-Projekt die gleiche Formatierung. Klar, die gefällt mir nicht an allen Stellen, aber ich muss keine Energie darauf verschwenden, mir Gedanken darüber zu machen.

Go hat keinen eigenen Paketmanager. Abhängigkeiten werden üblicherweise über Git-Repos geladen. Lange hatte Go auch keine Möglichkeit, explizit Abhänigkeiten anzugeben, man musste alle zum Projekt gehörigen Sachen in einem Pfad haben, den man durch die Umgebungsvariable `$GOPATH` definiert hatte. Das war umständlich und fehleranfällig. Seit einigen Jahren gibt es *Modules*, die den Umgang mit Projekten, Abhängigkeiten und deren Versionen vereinfachen.

Tools für automatische Tests (insbesondere unit tests) sind auch von Haus aus mitgeliefert und bieten ein paar Komfortfunktionen, es gibt also keine Ausrede, keine Tests zu schreiben.

### Error-Handling

Go hat kein System von Exceptions. Jede Funktion kann mehrere Rückgabewerte haben, Funktionen, die Fehler erzeugen können, geben einfach neben dem eigentlichen Rückgabewert noch einen Fehlertyp zurück. Wenn es keinen Fehler gab, ist der `nil`.

Dadurch vermeidet man eine ganze Reihe von Problemen, die man mit Exceptions hat. Insbesondere kann man direkt sehen, in welcher aufgerufenen Funktion ein Fehler auftreten kann und wie damit umgegangen wird.

Der Nachteil ist, dass Errorhandling in Go sehr verbose ist. Man hat praktisch immer ein

```
a, err := foo
if err != nil {
    return err
}
// [Erfolgsfall]
```

In Rust zum Beispiel gibt es für dieses Konstrukt ein Kürzel. Außerdem ist es in Go recht einfach, den Fehlerfall aus Versehen zu übergehen und mit dem anderen Rückgabewert weiterzuarbeiten. In dem steht aber im Fehlerfall üblicherweise nur Müll. Das kann zu den lustigsten Fehlern führen. Alles schon erlebt.

Eine weitere Schwäche hier ist, dass man üblicherweise nur den Typ `error` zurückgibt. Das ist alles was ein `error`-Interface implementiert. Man kann also nur sehr umständlich herausfinden, welcher Error jetzt speziell gerade zurückgegeben wurde. Wenn man je nach Fehlertyp unterschiedlich handeln will, muss man eine Menge Extracode schreiben.

Für schwerwiegende Fehler, mit denen das Programm nicht umgehen kann (z.B. Division durch 0) gibt es auch noch `panic`. Das funktioniert ähnlich wie in rust und ist prinzipiell erst einmal nicht recoverable. Man sollte also vermeiden, dass es ein panic gibt.

### Typsystem

Das Typsystem von Go ist erst einmal nicht schlecht. Man hat strikte Typisierung, einen Haufen grundlegender Typen und kann mithilfe von `struct` neue Typen definieren. Vererbung, virtuelle Funktionen und dergleichen gibt es nicht. Das verbuche ich auch positiv. Interfaces gibt es schon, also kann man wenn nötig eine gewisse, nützliche aber harmlose Art von Polymorphie haben.

Aber es gibt eine ganze Menge an Dingen, die ich an dem Typsystem auszusetzen habe.

#### The Billion Dollar Mistake

Es gibt in Go auch pointer (die immerhin memory-safe sind). Aber diese Pointer können auch `nil` sein (`null` oder `None` in anderen Sprachen). Es gibt keine nonnull-pointer. Mit anderen Worten: Überall, wo man Pointer verwendet (manchmal kommt man nicht darum herum) muss man überprüfen, ob der Pointer nil ist.

Außerdem werden hier zwei semantische Konzepte vermischt: Auf der einen Seite das Konzept der Referenz: Ein Wert, der auf einen anderen Wert verweist. Auf der anderen Seite das Konzept der Optionalität: Ein Wert, der entweder da ist oder eben nicht. Vielleicht möchte ich auch ein nicht-Referenz, die Optional ist. Oder eine Referenz, die nicht optional ist. Geht nicht.

Erschwerend kommt noch hinzu, dass nicht alle `nil`-Werte gleich sind und man beim Vergleichen zweier `nil`-Werte ein `false` bekommt.

Oh, aufgrund einiger komischer Zusammenhänge (s.u. unter „Nerviges“) ist `nil` eine leere slice und man kann Sachen dranhägen. Für Wörterbücher gibt das hingegen einen Fehler.

#### Default-Werte

Uninitialisiere Variablen sind gefährlich, also hat Go sich das Konzept der Defaultwerte ausgedacht. Das ist z.B. `0` für Integer-Typen, `false` für boolean, der leere String für Strings usw. Structs werden per default mit den Defaultwerten ihrer Member initialisiert.

Das führt dann zu Problemen, wenn man einem `struct` einen neuen Member hinzufügt. Wenn man dann nicht alle Stellen anpasst, an denen dieses Struct initialisiert wird, wird einen der Compiler nicht warnen und stattdessen dort einfach den Defaultwert reinschreiben. Versucht mal, _den_ Fehler zu finden.

Array-Slices werden per default mit `nil` initialisiert, aber das ist eigentlich kein Problem, weil man trotzdem noch Sachen dranhängen oder die Länge abfragen kann. Lustigerweise gilt das für `map` (ein Wörtberbuch) aber nicht: Das wird standardmäßig mit `nil` initialisiert und jede Interaktion damit führt zu einer `panic`.

Ein weiterer Punkt, wo die Default-Werte nervig sind, ist beim Deserialisieren von JSON: Wenn ein Feld in dem Struct, in das deserialisiert wird, nichtim JSON vorkommt, wird es einfach auf den Defaultwert gesetzt. Ich habe da schon die lustigsten Probleme mit gehabt:

##### Fall 1: Wert ist verpflichtend, Aufrufer vergisst ihn

In diesem Fall wird einfach angenommen, der Aufrufer hätte eine `0` oder so angegeben. Besser wäre hier, dem Aufrufer einen Fehler zurückzugeben.

##### Fall 2: Wert ist optional, ist aber im struct nicht als pointer definiert

In diesem Fall muss man hoffen, dass der Defaultwert kein gültiger Wert ist. Ich habe schon einmal Code gesehen, der immer eine leere Liste zurückgegeben hat, weil der optionale Wert so etwas wie `max_results` war, mit dem man angeben konnte, ob die Anzahl der Ergebnisse beschränkt sein soll.

Wenn der Defaultwert kein gültiger Wert ist, kann man natürlich einfach dagegen testen. Schön ist das aber nicht

##### Fall 3: Wert ist optional, ist im struct als pointer definiert

Für die Schnittstelle ist hier alles fein. Fehlt der Wert im JSON, ist der Wert im struct `nil` und man weiß, dass er nicht da ist.

Lustig wird es, wenn man in einem Struct einen Pointer auf einen primitiven Typ hat und dann versucht, das Struct zu initialisieren. Geht nicht ohne weiteres, weil man keinen Pointer auf z.B. ein `uint32`-Literal machen kann. Man muss zuerst eine gesonderte Variable anlegen und kann dann per Pointer darauf verweisen.

Meiner Meinung nach schaden Default-Werte also mehr als sie nützen und sind insbesondere in der Kombination mit `nil`-Werten bzw. mit den fehlenden Optional-Typen nervig.

#### Immutability

Es gibt keine gute Möglichkeit, Werte in Go immutable zu machen. Man kann immer alles überschreiben. Die einzige Lösung wäre, in einem Struct alle Member private zu machen und jede Menge Get-Funktionen zu schreiben. Macht aber keiner, weil es jede Menge Boilerplate erzeugt und wir ja gerade _von Java wegwollen_.

#### Generics

Lange Zeit gab es in Go keine Generics. Die Begründung war immer, dass man die ja nicht so dringend braucht. Auch mein ehemaliger Chef, ein großer Go-Fan, fragte immer: „Wann habt ihr das letzte mal etwas geschrieben, wo ihr Generics gebraucht habt?“.

Klingt nach einem fairen Punkt, verfehlt aber das Problem: Wenn man Anwendungen schreibt, braucht man meist keine Generics. Wenn man _Bibliotheken schreibt_ sind sie hingegen oft _sehr, sehr nützlich_. Ein anderes Team in meiner alten Firma hatte dann eine Bibliothek benutzt, die einen fucking _Code Generator_ dabei hatte, um typisierte Go-Dateien zu erzeugen. Lächerlich.

Und die Bibliotheken wird man früher oder später brauchen, wenn einem auffällt, dass man mit Arrays und Wörtberbüchern zwar sehr weit kommt, es aber immer Fälle geben wird, in denen man Containertypen braucht, die nicht in der Standardbibliothek drin sind (außer in Rust, an dieser Front ist Rust wesentlich besser aufgestellt als Go).

### Nerviges

Ein paar Dinge sind keine große Sache, nerven mich aber doch ein bisschen. Da wäre zunächst:

#### Privacy

Wie manche andere Sprache auch hat Go ein Konzept von privaten Werten und Funktionen. An sich finde ich die Implementierung ganz nett: Innerhalb eines Moduls kann man auf alles zugreifen, was dort definiert ist, außerhalb nur auf die öffentlichen Sachen.

Was mich ein bisschen nervt ist, _wie_ private Werte deklariert werden: Großbuchstabe am Anfang? Public. Kleinbuchstabe? Private.

#### Datums-/Zeitformatierung

Die meisten Sprachen bzw. Datumsbibliotheken von Sprachen haben eine Funktion, mit der man ein Datum in ein Textformat bringen kann. Die haben dann abstrakte Platzhalter, z.B. `%m` für den Monat. Golang hat einen anderen Weg genommen. In Golang gibt es ein Referenzdatum (in einem sehr ungewöhnlichen Format noch dazu): `01/02 03:04:05PM '06 -0700"`.

Ein Datumsformatstring muss dann die richtigen Zahlen aus diesem Datum zusammenpicken, ggf. noch auf eine 24-Stundenanzeige umrechenen und eingeben. Ein RFC3339-Datum sähe dann zum Beispiel so aus: `2006-01-02T15:04:05Z07:00`.

Ich kann mir denken, was der Gedanke dahinter war: Hey, dann kann man direkt sehen wie das Datum am Ende aussieht. Praktisch gesehen ist das aber nicht der Fall. Wie soll ich mir merken können, ob `01` jetzt der Monat oder der Tag ist? Wie soll ich mir die ganzen anderen Zahlen merken können. Ich muss _jedes Mal_ nachschauen, was denn jetzt das Referenzdatum ist.

#### arrays und slices

Go hat Arrays, die eine feste größe haben und slices, die auf einen Ausschnitt eines Arrays zeigen und Funktionen haben, mit denen man sie wachsen lassen kann. Für ein slice `s` kann ich also folgendes machen:

```
s = append(s, 4)
```

So wird der Wert 4 an das Slice angehängt. Leicht ungewöhnliche Syntax, aber daran könnte ich mich gewöhnen, wenn es wenigstens konsistent mit _irgendeinem anderen Kontrukt in der Sprache wäre_.

Wirklich fies wird es aber erst, wenn man _mehrere Referenzen auf die slice hat_. Sollte bei `append` das der Slice zugrundeliegende Array nicht groß genug sein, wird ein neues allokiert, die Daten aus dem alten rüberkopiert und der neue Wert hinzugefügt. Ansonsten wird nur der neue Wert hinzugefügt.

Preisfrage: Wenn man jetzt mehrere Referenzen hat, was passiert dann mit der alten Slice? Wenn ich in der jetzt etwas ändere (ein Feld ander zuweise), ändert sich dann die neue slice auch?

Die Antwort ist: Kann man nicht sagen. Das hängt davon ab, ob ein neues Array allokiert wurde. Wenn nein, dann ändert man auch die neue slice. Wenn doch, dann bleibt das neue slice unbehelligt.

Die einzige Lösung ist also: **auf keinen Fall mehrere Referenzen auf dieselbe slice**.

#### Kein map/filter/reduce

Was ich in Rust ja sehr schön finde ist die umfangreiche Sammlung von Funktionen, die man auf Iteratoren durchführen kann. Golang hat so etwas nicht. Keine `map()`- Funktion, kein `filter()`, kein `reduce()`. Aber hey, vielleicht kann man das ja per Bibliothek dazuladen?

Mittlerweile vielleicht. Lange Zeit ging das nicht oder nur mit erhöhtem Aufwand, denn _es gab ja keine Generics_.

#### Tiefe Kopien

Es gibt keine einfache Möglichkeit in Go, tiefe Kopien von etwas zu erstellen. Das habe ich schon on Java gehasst. In C++ geht es immerhin halbwegs einfach (mit ein bisschen Handarbeit in manchen Fällen), in Rust ist es trivial.

#### Unicode-Handling

Go-Quellcodedateien sind strikt UTF-8. Strings in Go… sind eigentlich nur unveränderbare `uint8`-Arrays. Man kann praktisch jeden Müll dort hereinschreiben. Klar, es gibt Funktionen um auf gültiges UTF-8 zu testen. Die meisten anderen Entwickler nutzen sie nur nicht und sind sich nicht einmal des Problems bewusst.

### The Ugly

An meinem ehemaligen Job gab es im Slack im `#Go`-Channel immer ein schönes Spiel. Irgendein Kollege hat ein Stück Go-Code geposted, das ganz harmlos aussah und gefragt: „Was gibt dieser Code hier aus?“.

Worauf ich hinaus will: Go hat einige _wirklich fiese Gotchas_. Keine Speicherkorruption, so weit sind wir hier nicht, aber einige Sachen, die in Rust definitiv nicht möglich sind, aber auch in vielen anderen Sprachen wie z.B. Java oder Python. Hier ein Beispiel (für Go-Versionen vor 1.22):

```
func main() {
	f := make([]func(), 5)
    for i := 0; i < 5; i++ {
        f[i] = func() {
            fmt.Println(i)
        }
    }
    for _, f := range f {
        f()
    }
}
```

Was gibt der Code aus? `55555`. Warum? Nun, die Variable `i` wird in der closure eingefangen. In der for-Schleife wird sie aber weiter hochgezählt. In den meisten Sprachen würde man davon ausgehen, dass hier der _Wert_ in die Closure kopiert wird. Wird er aber nicht. Es wird die Variable eingefangen. Wenn die Closures dann nach Ablauf der Schleife ausgeführt werden, ist `i` überall auf `5`.

Das Verhalten wurde glücklicherweise [in Version 1.22 geändert](https://go.dev/blog/loopvar-preview) (was ich bis vorhin nicht wusste). Bis dahin mussten wir uns immer auf zusätzliche Linter verlassen, die uns davor gewarnt haben.

Unglücklicherweise habe ich keinen Zugriff mehr auf die ganzen anderen Gotchas aus dem Slack-Channel. Ich vermute mal, dass einige andere auch durch diese Änderung behoben wurden. Trotzdem… dass es so lange gebraucht hat dafür, ist ein schlechtes Zeichen.

### Fazit

Ich bevorzuge Rust gegenüber Go in fast allen Fällen. Die Sicherheiten, die mir Rust bietet, vermisse ich in Go einfach zu sehr. Trotzdem würde ich weitaus lieber in Go schreiben als zurück zu Java zu gehen. Und ich kann nicht leugnen, dass es in Go _viel_ einfacher ist, einen kleinen Webserver aufzusetzen als in Rust. Für Rust muss man einen Haufen Abhängigkeiten installieren. Für Go… nichts.
