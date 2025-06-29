---
title: Jetzt muss ich C# lernen
filename: muss_csharp_lernen
date: 2025-06-29T15:38:19+02:00
update-date:
tags: programmieren, c-sharp, mono, lernen, unicode, golang, vorurteile, oop
category: hoellenmaschinen
summary: Habe ich neulich gesagt, dass ich bald beruflich wieder Go benutzen werde? Das war falsch, wie es aussieht, wird es C#.
image:
image-alt:
language:
---

In der Erwartung, dass ich bald beruflich wieder Go benutzen werde, habe ich Anfang der Woche [herumexperimentiert, wie gut Go einen davor schützt, falsches Encoding in `string`-Variablen zu haben](/blogposts/golang_string_encoding).

Zwei Tage später hatte ich dann ein Gespräch mit zukünftigen Kollegen und musste erfahren: Es wird nicht Go. Der Kunde besteht auf C# und lässt sich durch nichts davon abbringen. 75% des Teams kann zwar kein C#, aber das sind alles intelligente Leute, die werden das lernen.

Ich habe noch nie C# benutzt. Aber ich habe meine Vorurteile. Meinen Vorurteilen zufolge hat C# (zusammen mit Mono, bzw. .net) die gleichen Probleme, mit denen ich mich bei Java jahrelang herumplagen musste und froh war, davon wegzukommen. Insbesondere, dass es zwanghaft Klassen verwendet, auch für statische Funktionen, und dass es die Art von Entwickler anzieht, die es für eine gute Idee halten, auf Teufel komm raus alle Design-Pattern, die die objektorientierte Programmierung zu bieten hat, in den Code zu quetschen, egal, ob es nun sinnvoll ist oder nicht.

Ich erwarte [Dependency Injection](https://de.wikipedia.org/wiki/Dependency_Injection), [Inversion of Control](https://de.wikipedia.org/wiki/Inversion_of_Control), zu viele Interfaces, Dinge wie die [`AbstractSingletonProxyFactoryBean`](https://docs.spring.io/spring-framework/docs/current/javadoc-api/org/springframework/aop/framework/AbstractSingletonProxyFactoryBean.html) in Java Spring, [ORMs](https://de.wikipedia.org/wiki/Objektrelationale_Abbildung), das ganze Programm. Alles Dinge, die sich jemand ausgedacht hat um Sachen einfacher und flexibler zu machen, die am Ende aber alles komplexer machen, Flexibilität schaffen, die man nicht braucht und es dafür an anderer Stelle unmöglich machen, eigentlich triviale Dinge überhaupt zu machen.

Ich war in meiner Anfangszeit als Programmierer ein großer Fan von objektorientierter Programmierung, habe aber mit der Zeit ihre Schwächen gelernt und picke mir jetzt nur noch die Teile davon heraus, die ich für gut befinde. Das geht in Go und rust sehr gut, in Java eher nicht. Wie es bei C# aussieht bleibt abzusehen.

Jedenfalls kann ich meine Vorurteile demnächst prüfen. Vielleicht werden sie ja widerlegt. Ich erwarte das aber nicht wirklich.

Zwei positive Seiten hat es. Zum Einen lerne ich mal wieder eine neue Programmiersprache. Nur schade, dass ich nie gezwungen werde, Sprachen wie Lisp oder Haskell zu lernen (und alleine bringe ich nicht genug Motivation auf, diese Sprachen zu lernen, weil ich mit rust so bequem geworden bin).

Zum Anderen kann ich die Experimente zum String-encoding, die ich neulich mit Go gemacht habe, in C# wiederholen. Ich erwarte nicht wirklich viel, denn erste Experimente haben schon gezeigt, dass es sehr einfach ist, einen `string` in c# mitten im (UTF-16-codierten) codepoint durchzuschneiden:

```
const string snek = "🐍";

string snekstats = string.Format("length: {0}", snek.Length);
Console.WriteLine(snekstats); // ergibt "2"

char foo = snek[0]; // char ist 16 bit
Console.WriteLine(string.Format("{0}", foo)); // ungültiges encoding
```
