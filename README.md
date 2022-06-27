# CDP Saver
Cdp Saver is a program to download the contents of a Cahier de Prepa workspace

## Installation & usage
First of all, you need the latest version of [Rust](https://www.rust-lang.org/fr/tools/install, "link to install the latest version of Rust"). Then you must complete the configuration file `config.toml`:
```toml
spacename = "yourspacename"
path_to_save = "your/folder" 

[credential]
username = "your@mail.com"
mdp = "mysuperpassword"
```
* Spacename is the name of the workspace you want to retrieve. For example for:
`https://cahier-de-prepa.fr/mp2i-pv/`, the namespace is
`mp2i-pv`. 
* path_to_save is the root folder where you want to download the workspace
* username & mdp is your credential for the workspace

and then run the command to start the program
`cargo run config.toml` 


