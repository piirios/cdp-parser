# CDP Saver
Cdp Saver is a program to download the contents of a Cahier de Prepa workspace

## Installation & usage
First of all, you need the latest version of [Rust](https://www.rust-lang.org/fr/tools/install, "link to install the latest version of Rust"). Then you must complete the configuration file `config.toml`:
```toml
spacename = "yourspacename"
path_to_save = "votre/dossier" 

[credential]
username = "votre@mail.com"
mdp = "monsupermotdepasse"
```