# CDP-Parser
Cdp-Parser est un programme permettant d'archiver le contenue d'un espace de travail Cahier de Prépa

## Utilisation
télécharger dans `releases` la dernière version pour votre OS

pour lancer l'archivage de l'espace de travail executer:
```console
./votre_executable --workspace <WORKSPACE> --save-location <SAVE_LOCATION> --username <USERNAME> --password <PASSWORD>
```
où:
* WORKSPACE est l'acronyme de l'espace de travail. Par exemple pour un espace de travail à cette url
`https://cahier-de-prepa.fr/mp2i-pv/`, l'acronyme est
`mp2i-pv`. 
* SAVE_LOCATION est emplacement où sauvegarder les fichiers de l'espace de travail
* USERNAME & PASSWORD sont vaut identifiant de Cahier de Prépa

cela lancera le programme, aucune barre de chargement est prévu.

## Compilation
si vous ne trouvez une version dans `releases` faite pour votre OS, il va falloir compiler le projet. 
* munissez-vous de la dernière version disponible de [Rust](https://www.rust-lang.org/fr/tools/install, "link to install the latest version of Rust")
* Dans la racine du projet, executez: 
```console
cargo build --release
```
* Vous trouverez dans le dossier `target/release` la version pour votre OS.


