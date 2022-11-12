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

- [Commentaires](#commentaires)

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