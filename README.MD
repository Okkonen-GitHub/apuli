# Apuli - sanuli auttaja

toteutettu pelkästään rust ohjelmointikielellä.


## Käyttöohje

Käyttäminen on toistaiseksi aika ikävää

Argumenttien tulee olla järjestyksessä toistaiseksi.

`-5` ja `-6` käytetään valitsemaan sanapituus.

* `-g` määrää harmaat kirjaimet
```shell
cargo run -- -g okemun
```
* `-b` määrää keltaiset / siniset kirjaimet. `:` merkkiä tulee käyttää niiden erottamiseen toisistaan ja sijainneistaan.
* Alla oleva haku vastaa sitä, jos sanassa olisi kirjaimet A ja I, mutta I ei ole toinen kirjain sanassa ja A ei ole sanan viimeinen kirjain.
```shell
cargo run -- -5 -g ... -b a:4:i:1
```
* `-o` määrää vihreät / oranssit kirjaimet. Sitä käytetään samalla tavalla kuin `-b` vaihtoehtoa
* Alla olevassa esimerkissä A on toinen kirjain ja I viimeinen.
```shell
cargo run -- -5 -g ... -b ... -o a:1:i:4
```