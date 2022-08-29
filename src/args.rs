use crate::config::{CdpConfig, Credential};
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Louis Marchal (piirios)",
    version,
    about = "programme permettant d'archiver le contenue d'un espace de travail sur Cahier de Prépa"
)]
pub struct Args {
    #[clap(short, long)]
    /// espace de travail à archiver
    workspace: String,
    #[clap(short, long)]
    /// emplacement où sauvegarder les fichiers de l'espace de travail
    save_location: String,

    #[clap(short, long)]
    /// nom d'utilisateur
    username: String,

    #[clap(short, long)]
    /// mot de passe
    password: String,
}

impl Into<CdpConfig> for Args {
    fn into(self) -> CdpConfig {
        CdpConfig {
            spacename: self.workspace,
            path_to_save: self.save_location,
            credential: Credential {
                username: self.username,
                mdp: self.password,
            },
        }
    }
}
