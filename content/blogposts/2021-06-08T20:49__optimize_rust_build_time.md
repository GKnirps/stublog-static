---
title: Buildzeit des Bloggenerators verkürzen
filename: optimize_rust_build_time
date: 2021-06-08T20:49:21+02:00
update-date: 2021-06-08T22:00:00+02:00
tags: rust, cargo, chrono, pulldown-cmark, clap, optimierung, meta
category: meta
summary: Ich wollte mal die Buildzeit meines Bloggenerators verbessern. Es hat funktioniert.
---

Ich bastle gerade an ein paar Kleinigkeiten, die ich vielleicht in dieses Blog einbaue. Zum Beispiel, dass ich den in Markdown referenzierten Bildern `width` und `height`-Attribute zu verpassen. Dazu würde ich gerne das [image-Crate](https://crates.io/crates/image) einbauen.

Dieses crate baut nur leider gefühlt relativ langsam. Und meine Buildzeit ist schon recht hoch. Ich habe das mal getestet:

```
$ hyperfine -p "cargo clean" "cargo build --release"
Benchmark #1: cargo build --release
  Time (mean ± σ):     19.100 s ±  0.753 s    [User: 119.325 s, System: 5.649 s]
  Range (min … max):   17.264 s … 19.749 s    10 runs
```

Über 19 Sekunden im Schnitt! Das muss doch besser gehen. Wo gehen denn diese Sekunden verloren?

Glücklicherweise kann man das mit `cargo build --release -Z timings` aufschlüsseln. Das gibt in html-Form ein Dokument aus, wo aufgeschlüsselt ist, welche crates wie viel Buildzeit verschlingen.

### clap
Und was stellt sich heraus? Das crate, das am meisten Zeit benötigt ist [clap](https://crates.io/crates/clap), ein Kommandozeilenparser. Über neun Sekunden wird daran gebaut. Für was? Für drei Parameter, die geparsed werden, plus automatische Erstellung einer Hilfenachricht.

Mit anderen Worten: das brauche ich nicht wirklich, da kann ich auch selber schnell einen primitiven Interpreter schreiben. Für meine Zwecke reicht das.

### pulldown-cmark

Zweiter Punkt: da ist [getopts](https://crates.io/crates/getopts). Auch mehrere Sekunden. Aber warum? Warum ist da noch ein Kommandozeilenparser?

Stellt sich heraus, dass [pulldown-cmark](https://crates.io/crates/pulldown-cmark) per default auch ein eigenes binary baut. Mit allem drum und dran (wenigstens baut getopts schneller als clap). Auf pulldown-cmark möchte ich nicht verzichten, das ist ein sehr schöner commonmark-parser. Aber dieses eigene binary können wir abschalten mit `pulldown-cmark = { version = "^0.8.0", default_features = false }`.

### chrono

Bleibt noch [chrono](https://crates.io/crates/chrono), zum Darstellen von Zeit und parsen von Zeitangaben. Da kann man auch noch ein bisschen herausholen, aber nicht wirklich viel. Auch hier kann man ein paar Features deaktivieren: `chrono = { version = "^0.4.19", default_features = false, features = ["std"] }`. Ist aber nicht wirklich der Rede wert.

### Ergebnis

Andere crates brauchten entweder nicht so lange, oder brauchen so lange, aber ich brauche sie (wie z.B. pulldown-cmark oder [maud](https://crates.io/crates/maud)).

Also, wie weit habe ich das Ergebnis verbessert?

```
$ hyperfine -p "cargo clean" "cargo build --release"
Benchmark #1: cargo build --release
  Time (mean ± σ):     11.467 s ±  0.525 s    [User: 63.517 s, System: 4.204 s]
  Range (min … max):   10.280 s … 11.959 s    10 runs
```

Das sind etwa siebeneinhalb Sekunden. Also etwa 40% weniger Zeit.

Da kann ich jetzt beruhigt eine Bildbibliothek einbauen. Oder auch nicht, je nachdem, was aus dieser Bastelei wird. So oder so sollte auch da vielleicht auch aufpassen, nur Features mitzunehmen, die ich brauche.

### Update: image-crate

Kurze Tests haben ergeben: das oben genannte image-Crate braucht, eingebunden in ein sehr kleines Programm, etwa dreißig Sekunden zum Bauen. Wenn ich nur die Formate einbinde, die ich momentan unterstützen möchte (jpeg, png, gif, webp), geht es auf etwa 15s runter. Immer noch eine ganze Menge. Was kann ich also tun?

- auf die Bastelei verzichten (es läuft ja auch so alles gut)
- Bildgrößen manuell in die Metadaten schreiben (ugh)
- höhere Compilezeit in Kauf nehmen (nervig)
- selber etwas schreiben, was die Bildgröße herausfinden (ich schwanke hier zwischen „cool, habe ich noch nie gemacht“ und „bäh, Dateiformat-Standards entziffern“)
