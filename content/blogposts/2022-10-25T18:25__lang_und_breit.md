---
title: Lang und breit
filename: lang_und_breit
date: 2022-10-25T18:25:14+02:00
update-date:
tags: rust, optimierung, bilder, svg
category: meta
summary: Mal wieder eine kleine Optimierung an meinem Bloggenerator. Diese hat mich unverhältnismäßig viel Zeit gekostet.
image:
---

Ich habe endlich mal was umgesetzt, was ich schon seit knapp zwei Jahren vorhabe (auch [hier](/blogposts/optimize_rust_build_time) schon einmal erwähnt): `width` und `height`-Attribute an `img`-tags, die ich per Markdown in meine Blogposts einfüge.

Das ist jetzt kein Riesending, insbesondere, da ich ohnehin nur eher wenige Bilder einbinde. Die [zwei](/blogposts/fordite) [Posts](/blogposts/fordite_update) über Fordite und das [Kochprojekt](/categories/kochprojekt) von 2014 sind die Stellen, wo ich die meisten Bilder einbinde.

Warum will ich diese Attribute? Perfektionismus. Es ist besser, wenn man die tatsächlichen Bildgrößen direkt an den `img`-tags angibt (nicht zu verwechseln damit, Größenverhältnisse über CSS zu konfigurieren. Das sind zwei verschiedene Dinge mit zwei verschiedenen Zielen). Warum ist es besser? Weil der Browser so wissen kann, wie viel Platz für die Bilder reserviert werden muss und er nicht das Layout die ganze Zeit hin-und herschieben muss, wenn die Bilder endlich geladen sind.

Warum habe ich das mit den Attributen so lange aufgeschoben? Weil ich die Werte dafür nicht in Markdown einfügen kann, und sie woanders her zu laden ist eine Menge Arbeit (dazu unten mehr).

Aber hey, [sinnlose](/blogposts/blog-optimization) [Optimierungen](/blogposts/benchmark) sind ja so ein Steckenpferd von mir. Also ran an die Sache!

### Warum ist das so aufwändig?

Es gibt zwei Gründe, warum diese Erweiterung so aufwändig ist. Ich muss die Metadaten der Bilder an einer Stelle laden und sie bis zum Rendern der Blogposts durchschleifen. Das ist nervig und macht den Code nicht schöner.

#### Bildgrößen lesen

Und die Bilder müssen geparsed werden. Für die üblichen Rastergrafiken habe ich das [image](https://crates.io/crates/image)-crate genommen. Netterweise müssen dazu nicht die ganzen Bilder geladen werden, wenn ich nur die Größe haben möchte kann ich mit dem crate auch nur die Header lesen.

Für die SVG-Dateien wollte ich nicht noch eine Abhängigkeit einbinden. Da ich aber eh schon [quick-xml](https://crates.io/crates/quick-xml) einbinde, um die atom-feeds zu rendern, konnte ich das nutzen um an die nötigen SVG-Attribute zu kommen. Nervig nur, dass die `width` und `height`-Attribute von SVGs nicht verpflichtend sind, keine Integer sein müssen, verschiedene Einheiten haben können (z.B. `em` wie in CSS, aber auch `mm` oder `%`). Außerdem gibt es noch das `viewBox`-Attribut, das auch noch mal verändert, wie sich die Größe verhält. Ich habe hier erst einmal einen minimalen Support eingebaut.

#### Bildgrößen rendern

Am Ende müssen die Bilder gerendert werden. Zum Parsen und Rendern des Markdowns (streng genommen ist es [CommonMark](https://commonmark.org/), eine sauberere Spezifikation) benutze ich [pulldown-cmark](https://crates.io/crates/pulldown-cmark).

Das Crate beinhaltet einen Parser, der netterweise einen Iterator über die Elemente liefert. Außerdem gibt es eine Funktion, um aus so einem Iterator HTML zu bauen.

Das Problem ist, dass ich in diese Funktion zum HTML-bauen nicht direkt eingreifen kann. Ich kann also nichts machen wie „render alles wie gehabt, nur `img`-tags anders“.

Ich könnte jetzt den kompletten HTML-renderer neu bauen. Das ist _sehr_ viel Arbeit und ich mache vermutlich Fehler dabei.

Was ich alternativ gemacht habe war ein Trick, den ich auch schon benutzt habe, um zu verhindern, dass in Markdown eingebettetes HTML gerendert wird: Ich ersetze Parser-Events durch andere Events. Zum Beispiel: Um zu verhindern, dass das HTML gerendert wird, ersetzte ich Parser-Events vom Typ HTML als HTML gerendert werden, benutze ich stattdessen ein Text-Event mit demselben Inhalt. Der HTML-Renderer am Ende wird dann den Inhalt escapen.

Für Image-Events muss ich also nur das Image-Event durch ein HTML-Event ersetzen, dass als Inhalt das gerenderte image-Tag hat.

### Codequalität

Die Codequalität hat unter der ganzen Geschichte etwas gelitten. Das lässt sich aber zumindest teilweise aufräumen, wenn ich mal Lust dazu habe.

Insbesondere achtet der Bloggenerator noch nich darauf, dass sich Bildgrößen geändert haben. Wenn sich also Bildgrößen ändern, werden die Seiten, die die Bilder referenzieren, nicht neu gerendert.

Außerdem ist der Code, um die SVG-Größe herauszufinden nocht ziemlich simpel und wird in vielen Fällen nicht funktionieren. Also viel Verbesserungspotential.

### Bauzeit

Das image-crate, das ich eingebunden habe, zieht die Bauzeit schon stark runter (wie [hier](/blogposts/optimize_rust_build_time) schon einmal beschrieben). Wenigstens lässt sich der Effekt reduzieren, wenn ich nur die Formate unterstütze, die ich auch wirklich brauche.

### Fazit

Am Ende war es viel Aufwand für wenig Nutzen. Aber ich bin ja Perfektionist, ich muss das bestmögliche HTML erzeugen. Außerdem hat die jetzt aufgebaute Infrastruktur ein paar Optionen für mögliche andere Verbesserungen.
