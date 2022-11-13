### ⚠️ Il s'agit ici de mon apprentissage personnel, donc le contenu n'est pas garanti. Ne surtout pas hésiter pas à me faire remonter tout type d'erreur ou amélioration.

### Tester Rust en ligne sans rien installer
[play.rust-lang]([https://play.rust-lang.org])

### Installer Rust sur macos et linux

Exécutez la commande suivante dans votre terminal, puis suivez les instructions.<br>
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Si ce message est affiché c'est que tout c'est bien passé :)<br>
`Rust is installed now. Great!`

Ajouter un linter pour rust sur macos.<br>
`xcode-select --install`

Connaitre la version de rust<br>
`rustc --version`

Mettre à jour rust<br>
`rustup update`

Désintaller rust<br>
`rustup self uninstall`

Voir la version de cargo, il c'est installé avec rust<br>
`cargo --version`

Créer un nouveau projet avec cargo<br>
`cargo new le-nom-du-projet`

Lancer l'application<br>
`cargo run`

### Site officiel de Rust
* [rust-lang](https://www.rust-lang.org)

### Documentation de Rust
* [doc.rust-lang](https://doc.rust-lang.org/stable/std/)

### Code source de Rust sur github
* [code-source](https://github.com/rust-lang/rust)

### Cours officel en anglais
* [rust-book](https://doc.rust-lang.org/stable/book/)

### Reddit pour poser une question, faire une recherche ou de la veille
* [rust-reddit](https://www.reddit.com/r/rust/)

### Extensions Visual Studio Code que j'utilise:

* [rust-analyser](https://code.visualstudio.com/docs/languages/rust)

* [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)

* [better toml](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)

## RUST

- [1 - Commentaires](#commentaires)
- [2 - Types](#types)

## Commentaires
Ecrire un commentaire aide à comprendre notre code plus tard.

```rust
fn main() {
  // Les programmes Rust commencent par fn main() 
  // Le code est dans un bloc entre les {}
  let number = 42; // Ecrire ici est autorisé, le compilateur ne le prendra pas en compte
}
```

Il est possible d'écrire au milieu du code.

```rust
fn main() {
  let number/*: i16*/ = 10;
}
```

Le compilater, `let number/*: i16*/ = 10;` prend en compte `let number = 10;`.

`/* */` est utilisé pour des commentaires multilignes

```rust
fn main() {
  let number = 10; /* Un super
  beau
  commentaire */
  let number = 10; // Un super
  // beau
  // commentaire
}
```

## Types
Rust est un langage à `typage statique`, il doit connaître les types de toutes les variables au moment de la compilation. Le compilateur peut déduire le type que nous voulons utiliser en fonction de la valeur et de la manière dont nous l'utilisons.

### Types primitifs (Primitive types)
Rust a des types simples appelés types primitifs. Nous allons commencer par les entiers et char (caractères). Les entiers sont des nombres sans point décimal. Il existe deux types d'entiers :
- Signed integers (entier signé)
- Unsigned integers (entier non-signé)

Un `entier signé` contient des valeurs entières qui peuvent être positives ou négatives. Un entier `non signé` ne peut contenir que des valeurs positives.

Les entiers signés sont : `i8`, `i16`, `i32`, `i64`, `i128` et `isize`.
Les entiers non signés sont : `u8`, `u16`, `u32`, `u64`, `u128` et `usize`.

Le nombre après le `i` ou le `u` représente le nombre de bits, donc les nombres avec plus de bits peuvent être plus grands. 8 bits = un octet, donc i8 est un octet, i64 est 8 octets, et ainsi de suite. Les types de nombres avec des tailles plus grandes peuvent contenir des nombres plus grands. Par exemple, un u8 peut contenir jusqu'à 255, un u16 peut contenir jusqu'à 65535 et un u128 peut contenir jusqu'à 340282366920938463463374607431768211455.

`isize` et `usize` sont le nombre de bits que possède un type d'ordinateur.Ainsi, `isize` et `usize` sur un ordinateur `32 bits` sont comme `i32` et `u32`, et `isize` et `usize` sur un ordinateur `64 bits` sont comme `i64` et `u64`.

Les caractères en Rust sont appelés `char`. Chaque caractère a un numéro : la lettre A est le numéro 65, le caractère 友 ("ami" en chinois) est le numéro 21451. La liste des numéros s'appelle "Unicode". Unicode utilise les nombres les plus petits pour les caractères qui sont le plus utilisés, comme A à Z, ou les chiffres 0 à 9, ou l'espace.

```rust
fn main() {
  let first_letter = 'A';
  let space = ' '; // L'espace entre les ' ' est un caractère
  let cat_face = '😺'; // Les emojis sont aussi des caractères
}
```

Les caractères les plus utilisés ont des nombres inférieurs à 256, et ils peuvent tenir dans un `u8`.

Tous les caractères utilisent 4 octets de mémoire, puisque 4 octets suffisent pour contenir n'importe quel type de caractère.
Les lettres et symboles de base nécessitent généralement 1 octet sur 4 : a b 1 2 + - = $ @ D'autres lettres comme les trémas ou les accents nécessitent 2 octets sur 4 : ä ö ü ß è é à ñ Les caractères coréens, japonais ou chinois nécessitent 3 ou 4 octets : 国 안 녕

`.len()` donne la taille de la chaîne en octets :
```rust
fn main() {
  println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
  println!("Size of string containing 'a': {}", "a".len()); //  1 byte
  println!("Size of string containing 'ß': {}", "ß".len()); // 2 bytes
  println!("Size of string containing '国': {}", "国".len()); // 3 bytes
  println!("Size of string containing '𓅱': {}", "𓅱".len()); // 4 bytes
}
```

`.len()` donne la taille en octets

`.chars().count()` transforme en caractères et compte

```rust
fn main() {
  let slice = "Hello!";
  println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count()); // slice est de 6 octets et également de 6 caractères
  let slice2 = "안녕!";
  println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count()); // slice2 est de 7 octets mais seulement 3 caractères
}
```
