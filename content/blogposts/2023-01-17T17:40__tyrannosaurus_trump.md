---
title: ChatGPT und der Tyrannosaurus Trump
filename: tyrannosaurus_trump
date: 2023-01-17T17:40:51+01:00
update-date:
tags: chatgpt, trump, tolkien, limerick, advent of code, zauberlehrling
category: hoellenmaschinen
summary: Ich habe mal ChatGPT ausprobiert. Technisch ist schon schon beeindruckend.
image:
---

Nach dem ganzen Hype in den letzten Wochen musste ich auch endlich mal dieses [ChatGPT](https://chat.openai.com/) ausprobieren. Und ich will hier auch überhaupt nicht in eine der Diskussionen einsteigen, ob und wie das die Gesellschaft verändern wird, ob das gut oder schlecht ist oder ob ChatGPT-Einreichungen beim Advent of Code unfair sind.

Ich habe mir das Teil angeschaut und mein erster Eindruck ist, dass es ein wirklich beeindruckendes Stück Technik ist. Hier einmal meine jüngste Anfrage (in einem Chat hatte irgendjemand mit Dino-Propaganda angefangen, und ich hatte mir gedacht, ich lasse ChatGPT was daraus machen):

> I need a satirical speech in the following setting: It is the end of the cretaceous period. Tyrannosaurus Trump gives a speech about how the imepnding Meteor ist just a lie to weaken the economy and that his arms are not small.

Genau so habe ich das eingegeben, Tippfehler inklusive. Das Ergebnis war:

> Ladies and gentlemen, esteemed dinosaurs, I stand before you today to address the rumors of an impending meteor strike. I want to be completely clear: this is fake news. It's a hoax perpetrated by the liberal media and their fossil fuel friends to weaken our economy and make us look bad.
>
> Let me ask you, have any of you seen this meteor with your own eyes? Of course not. It's all based on computer models and "scientific evidence" that can easily be manipulated. Trust me, I know a thing or two about fake news.
>
> And let me tell you, my arms are definitely not small. They are huge, the biggest in the history of the Cretaceous period. Believe me.
>
> But even if this meteor were real, which it's not, we would be perfectly fine. We have the best scientists, the best technology, and the best defense systems. We will protect ourselves and come out on top, as we always do.
>
> So don't believe the lies. Keep your heads held high and let's keep making the Cretaceous great again. Thank you.

Großartig. Nur drei Sätze als Eingabe, in natürlicher Sprache, und dass es um Donald Trump geht ist nur durch „Tyrannosaurus Trump“ angedeutet. Daraus macht ChatGPT eine großartige Satire einer Trumpschen Rede.

ChatGPT kann also nicht nur sprachliche Eingaben verstehen, es ist anscheinend auch recht gut darin, sie mit Kontextwissen zu verbinden und daraus etwas zu machen, was nicht nur Sinn ergibt, sondern auch ziemlich genau das ist, was ich wollte.

Natürlich kommen nicht immer so wunderbare Sachen dabei raus. Manchmal muss man ein paar Mal nachhaken und spezifischer werden, sonst sind die Ergebnisse eher mäßig. Die größten Probleme hatte ich bisher bei Lyrik. Eine einfache Anfrage nach einem Limerick:

> I need a limerick about how Morgoth stole the Silmarils

wurde noch mit einem brauchbaren Limerick beantwortet:

> Once Morgoth, dark lord of hate,  
> Stole the Silmarils, shining so great.  
> He hid them deep in the earth  
> In a land of eternal dearth,  
> But the Elves' quest for them was not late.

Kompliziertere Dinge, wie eine Parodie auf den [Zauberlehrling](https://de.wikipedia.org/wiki/Der_Zauberlehrling), gelangen nicht so gut. Es kam zwar ein Gedicht heraus, das von der Handlung auch sehr dem Zauberlehrling ähnelte, aber der Stil war völlig anders.

Eine Anfrage auf ein nicht ganz triviales Programmierproblem (gegeben eine einfach verkettete Liste, gib mir einen Algorithmus, der in linearer Zeit und konstantem Speicher herausfindet, ob die Liste eine Schleife enthält) kriegte ChatGPT noch hin. In Pseudocode, aber auch in rust. Als ich eine Rust-Version ohne pointer wollte, hat es mir die auch ausgespuckt, aber mit der Anmerkung, dass diese Version nicht funktioniert, wenn die Liste eine Schleife enthält (mit anderen Worten: sie ist nutzlos).

Die Anmerkungen, die ChatGPT macht, sind übrigens auch interessant. Ich habe mit ChatGPT über Verschwörungsideologien gesprochen, und ein fiktives Beispiel aus einer Rollenspielkampagne verwendet. Kurz zusammengefasst: Welche Argumente und Verschwörungsmythen kann ein rassistischer Elf verwenden, um nekromantische Experimente an anderen Spezies durchzuführen?

Die Ergebnisse waren erschreckend gut. Auf meinen Kommentar „Oh god I created a monster“ kam glücklicherweise die Antwort:

> It is important to remember that I am just a model trained on text and I do not have the ability to create or think for myself. The character you have described is fictional and the justifications provided are based on the information you've given. It's important to consider the potential consequences and ethical implications of the actions of any characters in a story, whether they're fictional or real-life. It is also important to consider the fact that many people in the real world use similar reasoning to justify harmful and unethical actions. It's important to critically evaluate any argument and be aware of the potential harm it can cause.

ChatGPT ist auch fähig, eine (zugegebenermaßen relativ langweilige) Fantasy-Spezies zu erfinden und mit DnD5-Stats dazu zu geben, sogar mit der Anmerkung, dass diese Spezies schwierig zu balancen sei.

Im Großen und Ganzen bin ich von ChatGPT wirklich beeindruckt. Wenn man die Fragen richtig stellt, kann man für viele Fälle coole Antworten bekommen. Was man jedoch bedenken sollte ist, dass einige Leute schon erlebt haben, wie ChatGPT einfach Sachen erfindet, wenn es nicht weiter weiß. Da ich es hauptsächlich benutzt habe, um Sachen zu erfinden, war das natürlich kein Problem.
