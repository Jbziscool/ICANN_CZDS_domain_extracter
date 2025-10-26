Tein tän CLI-työkalun, jolla voi kaivaa domainit niistä zone fileistä, joita voi ladata ICANNin CZDS-palvelusta (https://czds.icann.org/).

En tiiä onko tää kellekään hyödyllinen, mut jos on, niin siistii.

Itellä oli tarkoituksena ladata zone filejä, et voisin ettiä ja ilmotella phishing-domainta, ja samalla kehittää datakaavion siitä, mitkä TLD:t on yleisimmin käytössä haittatoiminnassa. Siks tein tän skriptin. Tässä projektissa mua kiinnosti vaan ne domain-nimet ite, ei nameserverit tai muu sellanen.

Kontekstiks ja esimerkiks, tämmöseltä zone file näyttää (esimerkkidata):

```
abcdefg.com.	3600	in	ns	nameserver.cloudflare.com.
domainname.com.	3600	in	ns	cheese.ns.cloudflare.com.
domainname2.com.	3600	in	ns	icantthink.nsprovider.com.
```

Sivuhuomautus: huomasin kans et tää skripti on ihan sairaan nopee. En tiiä johtuuks se vaan siitä et oon tottunu käyttään python enkä rust (joka on tietty paljon nopeempi), mut tää sai kaivettuu miljoona domainii jossain 0.9 ms tai jotain tosi lähellä, ja sit loput 2 miljoonaa domainii meni jossain kahessa sekunnissa.

![image](https://i.ibb.co/5Wrj66qm/x-F15bj2vw-U.png)

# Kuinka käyttää

#### Huomio koodin lataamisesta

Voit joko ladata exe-tiedoston repositorion releases osiosta tai kääntää koodin ite komennolla `cargo build --release`.

Ekaks sun pitää olla ladannu zone file, jonka saa täältä: https://czds.icann.org/

Tän jälkeen aja tää komento:

```
./extract_domains.exe tld filename.txt
```

- `tld` on se TLD mitä haluut kaivaa (esim. `xyz`).
- `filename.txt` on se zone file (yleensä nimeltään `tld.txt` ku sen lataa).

Eli jos haluut kaivaa .xyz-domainit tiedostosta `xyz.txt`, niin käytät komentoo:

```
./extract_domains.exe xyz xyz.txt
```

Tässä on se regex mitä skripti käyttää domainien kaivamiseen:
```rust
let pattern = format!(r"\b[^\s]+\.{}\b", regex::escape(tld));
```

##### Jos tulee virheitä, voit avata issuen tai laittaa viestii mulle osoitteeseen jbz@jbz.dev (muista mainita et kyse on tästä repositoriosta).
