# Apuli - sanuli auttaja

Auttaja sovellus [sanuli.fi](https://sanuli.fi) peliä varten.

Sisältää verkkopohjaisen käyttöliittymän ja komentorivisovelluksen (vanhentunut).

Demo saatavilla osoitteessa <http://129.151.218.65/index.html>.


## Lähteet ja lisenssit
Katso [NOTICE.MD](NOTICE.MD).


# TODO

- [x] Support for neluli
- [x] Ranking system for guesses based roughly on information revealed by guess
- [x] "Scouting" feature
- [ ] Fix blue and orange generation in bench, ui, and possibly checking them in lib (in progess)
- [ ] Apuli-dbg : a visual debugging app to ensure correctness of the algorithm
- [ ] Reduce information in Apuli-ui and other optimizations (in progess)
  - [x] Show first n words
  - [ ] Show last n words (and dots in between)
  - [ ] Let the use decide how many they want (1-100|all, 0-100)
    - If "all" show everything
    - Else show 1-100 best and 0-100 worst (remove duplicates)
    - Default: 25 and 3
- [ ] Make the algorithm smarter by including stuff from information theory (rank, scout?)
  - [x] Add rank_information to lib (and all subfunctionality)
  - [ ] Make it visible in the UI (a 3rd mode)
  - [ ] Benchmark information theory algorithm
- [ ] Refactoring code (in progess)
- [ ] Change colors with numbers (1-5)
## Apuli-ui

Tarvitset ainakin trunk:in ja sen vaatimat jutut, jotka saat seuraavasti:

```
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cargo install wasm-bindgen-cli
```


Käynnistä paikallinen Apuli(-ui) sovellus (<http://localhost:8080>):

```shell
trunk serve --release
```

Tai vaihtoehtoisesti käynnistä paikallinen kehitys palvelin:

```shell
trunk serve
```
Tai paketoi sovellus

```shell
trunk build --release
```


## Apuli-cli Käyttöohje

Käyttäminen on toistaiseksi aika ikävää, ja tulee todennäköisesti aina olemaan.
Suosituksena on käyttää selaimessa toimivaa käyttöliittymää (Apuli-ui).

Argumenttien tulee olla järjestyksessä (`-5|6 -g ... [-b ... -o ...]`) toistaiseksi.

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
