---
title: GitLab Registrierungs-Dark-Patterns
filename: gitlab_dark_patterns
date: 2023-10-11T21:06:14+02:00
update-date:
tags: gitlab, dark patterns, webdesign, datenschutz, hugo, git
category: weiteweitewelt
summary: GitLab ist eine tolle freie Plattform für git-Repos. Allerdings gibt es bei der Registrierung ein paar Details, die ich durchaus als „Dark Pattern“ betrachte.
image:
language:
---

Als Github vor ein paar Jahren von Microsoft übernommen wurde, gab es eine große Abwanderung zu GitLab. Gut, soweit ich es erkennen kann, ist Github immer noch weitaus populärer, aber immerhin.

[GitLab](https://de.wikipedia.org/wiki/GitLab) ist einerseits eine Web-Application zum Hosten und Verwalten von git-Repos, andererseits gibt es auch eine zentrale Instanz dieser Software, wo man git-Repos hosten kann, sowohl öffentlich als auch in einem gewissen Rahmen private Repos. Das Ganze ist kostenlos.

Ich selbst hoste ein paar Sachen auf GitLab, unter anderem ein Repo, das ein mit [Hugo](https://gohugo.io/) erstelltes Lexikon enthält (über eine Fantasy-Welt einer Rollenspielrunde, an der ich teilnehme). [Hier](/blogposts/hugo) habe ich schon einmal davon berichtet.

### Kollaboration am Lexikon

Nun ist die Spielleiterin und Autorin der Welt mit an Bord bei diesem Projekt, hat bisher aber nur über mich Dinge eingepflegt. Ich wollte ihr jetzt zeigen, wie sie das selber machen kann, zumindest bis zu einem gewissen Grad, damit es für mich und für sie schneller geht.

Besagte Spielleiterin ist keine Informatikerin und hat weder viel Erfahrung it Markdown oder Hugo, und erst recht keine Erfahrung mit git. Besonders letzteres könnte ein Problem sein, denn obwohl git die ganze Arbeit _sehr_ vereinfacht, muss man doch erst ein paar Konzepte lernen, um damit umzugehen.

Ein paar Konzepte lernen, einen git-Client unter Windows installieren, den man idealerweise nicht nur per Kommandozeile benutzen kann, den ganzen Krams pushen… das alleine kann für jemanden, der nur kurz ein paar Tags zu Charakterseiten hinzufügen möchte, abschreckend sein. Besagte Freundin war aber willens, sich darauf einzulassen.

Glücklicherweise gibt es bei GitLab ja auch die Möglichkeit, kleinere Änderungen direkt auf der Website vorzunehmen, so dass vielleicht zumindest das Herumgehampel mit einem lokalen Klon des Repos entfällt. So oder so, die Spielleiterin brauchte erst einmal einen GitLab-Account.

### Account-Erstellung schwer gemacht

Ein Account bei GitLab ist grundsätzlich kostenlos. Man kann zwar Zusatzoptionen buchen, die einem eine CI-Pipeline oder ähnlichen Krams bringt und es erlaubt, mehr als eine kleine Anzahl an Personen an einem privaten Repo arbeiten zu lassen, aber das brauchten wir nicht. Die Spielleiterin brauchte nur einen kostenlosen, einfachen Account. Von dem ich ihr versicher habe, dass sie ihn anlegen kann.

Dementsprechend war sie ein bisschen überrascht, als sie auf der [GitLab-Seite](https://about.gitlab.com/) (also der Seite, auf die man gelangt, wenn man nicht eingelogged ist) folgendes sah:

![Screenshot. Ein Fett markierter „Get free trial“-Button links, ein unmarkierter „Sign in“-Link rechts](/file/gitlab_trial_login.webp)

„Get free trial“ impliziert ja, dass der Account grundsätzlich etwas kostet. Was besagte Freundin erst einmal sehr skeptisch gemacht hat. Auf den „Sign in“-Link zu klicken kam ihr nicht in den Sinn. Warum auch? Sie hatte ja noch keinen Account, um sich einzuloggen.

Glücklicherweise sah sie oben links im Menü einen „Pricing“-Link, der sie zu einer Seite führte, wo unter anderem das hier zu sehen war:

![Screenshot: „Free, essential features for individual users, 0$ per user/month“, darunter ein „Get started“-Link](/file/gitlab_free_account.webp "$ your soul")

Das sah richtig aus, also hat sich sich darüber einen Account erstellt. Die Überraschun kam nach der Registrierung, als der Account aktiviert werden sollte. Dort war nicht nur ein Feld zur Bestätigung der E-Mail-Adresse, sondern auch ein Pflichtfeld zur Eingabe einer Telefonnummer. Zitat der Spielleiterin:

> Jetzt wollen die auch noch meine Handynummer, sorry, nein

Durchaus eine gute Einstellung. Wieso zur Hölle wollen die ihre Telefonnummer? Habe ich dort meine Telefonnummer angegeben?

Ein kurzer Check stellte heraus: Nein, natürlich nicht. Es musste also einen Weg ohne Telefonnummer geben. Eine kurze Suche hat dann auch ergeben, wo: Wenn man, wie sie, über die „Pricing“-Seite geht, kommt man zu [dieser Registrierungsseite](https://gitlab.com/-/trial_registrations/new).

Wenn man aber zuerst konterintuitiv auf „Sign in“ klickt, und dann dort auf „Register now“, kommt man stattdessen auf [dieser Registrierungsseite](https://gitlab.com/users/sign_up). Der Unterschied ist erst einmal minimal. Bei dieser Seite muss man aber nach der Registrierung nur die Mailadresse bestätigen, keine Telefonnummer angeben.

Nun kann meine Spielleiterin jetzt erst einmal trotzdem keinen Account erstellen. Sie hat sich ja schon mit ihrer Mailadresse registriert. Da braucht sie aber eine Telefonnummer. Den Account kann sie auch nicht löschen, bis sie ihn aktiviert hat. Die einzige Möglichkeit, die bleibt, ist, drei Tage zu warten bis der nicht-aktivierte Account gelöscht wird.

### Fazit

Zitat meiner Spielleiterin:

> Organistionen, die solchen Kram veranstalten mit einem, sind eigentlich eh nicht so mein Fall. Es sieht für mich sehr so aus, als wenn die dauerhafte, kostenlose Option, wo sie einem nicht Infos aus der Nase ziehen, eigentlich nur noch so als Überbleibsel da ist, und das nicht das ist, was sie wollen, das man benutzt. So, wie das wirkt.

Sie hat recht. Das Ganze stinkt sehr nach einer Masche, Leute mit aller Kraft in Richtung der Zahloptionen zu schieben, selbst wenn sie das überhaupt nicht brauchen. Und um zusätzlich nocht weitere personenbezogene Daten (die Telefonnummer) einzusammeln. Ein klassisches [Dark Pattern](https://de.wikipedia.org/wiki/Dark_Pattern).

Nun habe ich persönlich ein gewisses Vertrauen in GitLab, aber ich war trotzdem ziemlich enttäuscht von dieser Benutzerführung. Und jetzt kann ich nur hoffen, dass meine Spielleiterin nach dieser Geschichte überhaupt noch GitLab benutzen will. Ihre Motivation hat vermutlich einen starken Dämpfer bekommen, noch _bevor_ ich überhaupt mit dem git-Workflow um die Ecke gekommen bin. So kann man Leute auch von cooler Technologie fernhalten: Indem man unnütze Hürden einbaut, die demotivieren, bevor man überhaupt richtig angefangen hat. 
