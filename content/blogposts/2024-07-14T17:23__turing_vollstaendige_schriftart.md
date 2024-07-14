---
title: Turing-vollständige Schriftarten
filename: turing_vollstaendige_schriftart
date: 2024-07-14T17:23:59+02:00
update-date:
tags: schriftart, wasm, harfbuzz, LLM, log4j
category: hoellenmaschinen
summary: HarfBuzz, ein verbreiteter text shaper, unterstütz Wasm-Code in Schriftarten
image:
image-alt:
language:
---

Vor ein paar Wochen bin ich über [llama.ttf](https://fuglede.github.io/llama.ttf/) gestoßen. Das ist eine Schriftart, die gleichzeitig ein Large Language Model ist und, ähnlich wie z.B. ChatGPT, Text generieren kann.

Da hatte ich natürlich einen gewissen WTF?-Moment. Wie geht das? Nun, es stellt sich heraus, dass [HarfBuzz](https://github.com/harfbuzz/harfbuzz/tree/main), ein verbreiteter text shaper, eine Funktion hat, die es einer Schriftart ermöglicht, beliebigen [WebAssembly](https://de.wikipedia.org/wiki/WebAssembly)-Code auszuführen, um die Ausgabe zu beeinflussen.

HarfBuzz wird nun in allen möglichen Projekten verwendet, unter anderem auch in Firefox und Chrome. Damit können dann auch Schriften korrekt dargestellt werden, die etwas komplizierter darzustellen sind als die auf der lateinischen Schrift basierenden Schriften. Und damit man auch wirklich alles machen kann, gibt es da jetzt auch die Option, [WebAssembly-Code direkt in die Schriftart einzubinden](https://github.com/harfbuzz/harfbuzz/blob/main/docs/wasm-shaper.md). Dieser Code wird dann von HarfBuzz ausgeführt.

Ich bin ja durchaus dafür, dass man jede Schrift der Welt auch im Computer korrekt darstellen können soll. Der Hauptentwickler von HarfBuzz war [laut Wikipedia](https://de.wikipedia.org/wiki/HarfBuzz) unglücklich mit mangeldem Support für die persische Schrift. Das war natürlich kein Zustand, und es ist gut, dass daran etwas getan wurde.

Aber bei Turing-vollständigem Code in der Schriftart hört für mich der SPaß auf. Erstens, weil natürlich genau das passieren wird, was auch mit JavaScript und Websites passiert ist: Webentwickler sind zu faul etwas richtig zu machen und fummeln dann lieber ein bisschen JS zusammen um eine Browserfunktionalität zu duplizieren und alles komplizierter zu machen. So erwarte ich, dass Designer von Schriftarten auch irgendwann nicht mehr die eigentlichen Features eines Schriftartenformats verwenden werden, sondern lieber etwas mit WebAssembly zusammenhacken, was dann das gesamte Textrendering langsamer macht und an vielen Stellen nicht funktioniert, weil dann doch nicht HarfBuzz verwendet wird.

Viel wichtiger aber: Das ist fucking _code execution_. Nun kann man argumentieren, dass das ja in einer Sandbox ist. Das kann nicht nach Hause telefonieren. Das kann nicht auf Dateien zugreifen oder sonst irgendwas machen. Aber wie wir wissen: Es gibt immer wieder Bugs in solchen Sandboxes, und irgendwie schafft dann jemand, daraus auszubrechen.

Bei Javascript ist das schon öfters passiert. Eine Type-confusion hier, ein Overflow dort, oder ein bisschen [Rowhammern](https://de.wikipedia.org/wiki/Rowhammer) und – bahm! sitzt man nicht mehr im Sandkasten. HarfBuzz ist auch nur in C geschrieben. Egal wie gut ein Entwickler ist, früher oder später passiert irgendwo ein Fehler auf.

Meine Prognose: Irgendwann innerhalb der nächsten 10 Jahre wird uns das auf die Füße fallen. Und dann, ähnlich wie bei [log4shell](/blogposts/log4j) damals werden sich alle wundern, warum es dieses Feature überhaupt gab und ob unsere Software nicht zu komplex geworden ist.
