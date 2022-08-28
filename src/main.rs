mod args;
mod config;
mod string_utils;

use crate::string_utils::*;
use anyhow::{anyhow, Context, Result};
use config::CdpConfig;
use config::Credential;
use reqwest::{Client, RequestBuilder};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncWriteExt;

use args::Args;
use clap::Parser;

use async_recursion::async_recursion;

trait WindowString {
    fn window_string(&self) -> String;
}
impl WindowString for String {
    fn window_string(&self) -> String {
        self.replace("<", "")
            .replace(">", "")
            .replace("\"", "")
            .replace("/", "")
            .replace("\\", "")
            .replace("|", "")
            .replace("?", "")
            .replace("*", "")
            .replace(":", "")
            .replace(".", "")
    }
}
impl WindowString for &str {
    fn window_string(&self) -> String {
        self.replace("<", "")
            .replace(">", "")
            .replace("\"", "")
            .replace("/", "")
            .replace("\\", "")
            .replace("|", "")
            .replace("?", "")
            .replace("*", "")
            .replace(":", "")
            .replace(".", "")
    }
}

#[derive(Debug)]
struct CdpFile {
    fname: String,
    url: String,
    fpath: String,
}
impl CdpFile {
    /*
     * Fonction permettant d'initialiser un Fichier où:
     * fname: est le nom du fichier
     * url: url vers cdp du fichier
     * fpath: chemin d'accès relatif du fichier
     */
    fn new(fname: String, url: String, fpath: String) -> Self {
        Self { fname, url, fpath }
    }
    /*
     * Fonction permettant de sauvegarder le fichier en téléchargant le contenue où:
     * cookie: Le cookie de la session
     */
    async fn save(&self, base_url: String, parser: Arc<CdpParser>) -> Result<()> {
        fs::create_dir_all(Path::new(&base_url).join(self.fpath.to_owned())).await?;

        let content = parser
            .client
            .get(self.url.to_owned())
            .send()
            .await?
            .bytes()
            .await?;

        let mut dest = File::create(
            Path::new(&base_url)
                .join(self.fpath.to_owned())
                .join(self.fname.to_owned()),
        )
        .await?;

        let mut pos = 0;
        while pos < content.len() {
            let bytes_written = dest.write(&content[pos..]).await?;
            pos += bytes_written;
        }

        Ok(())
    }
}

pub struct CdpParser {
    cred: Credential,
    spacename: String,
    pub cookie: String,
    pub client: Client,
}

impl CdpParser {
    /*
     * Fonction permettant d'initialiser le Parser où:
     * cred: Structure stockant les identifiants fournis par le config.toml
     * spacename: chaine de caractère identifiant la prépa où l'on souhaite récupérer les fichiers
     *
     * => Cette Structure
     */
    fn new(cred: Credential, spacename: String) -> Self {
        Self {
            cred,
            spacename,
            cookie: String::default(),
            client: Client::builder().cookie_store(true).build().unwrap(),
        }
    }
    /*
     * Fonction permettant de générer une url où:
     * url: url relative
     *
     * => l'Url
     */
    #[inline]
    fn build_url(&self, url: &str) -> String {
        if url.len() == 0 {
            format!("https://cahier-de-prepa.fr/{}", self.spacename)
        } else {
            format!("https://cahier-de-prepa.fr/{}/{}", self.spacename, url)
        }
    }
    /*
     * Fonction permettant de récupérer le cookie de session en s'identifiant
     *
     * => Résultat de l'authentification
     */
    async fn auth(&mut self) -> Result<()> {
        let mut form = HashMap::from([
            ("login", self.cred.username.as_str()),
            ("motdepasse", self.cred.mdp.as_str()),
            ("connexion", "1"),
            ("csrf-token", "undefined"),
        ]);
        let req_url = self.build_url("ajax.php");
        let req = self.client.post(&req_url).form(&form).send().await?;

        let temp = String::from_utf8(req.headers().get("set-cookie").unwrap().as_bytes().to_vec())?;
        self.client.get(self.build_url("")).send().await?;
        let cookie = temp.before(";").unwrap().to_owned();
        self.cookie = cookie;

        Ok(())
    }
    /*
     * Fonction permettant de générer une requête comme il le faut dès lors que l'on dispose du cookie de session
     *
     * => La Structure de la Requête
     */
    #[inline]
    pub fn build_request(&self, url: String) -> RequestBuilder {
        self.client.get(url)
        //.header("Cookie", self.cookie.to_owned())
    }
    /*
     * Fonction permettant de récupérer l'url du flux RSS
     *
     * => Résultat avec l'url du flux RSS
     */
    async fn get_rss_url(&self) -> Result<String> {
        //dbg!(self.build_request(self.build_url("recent")));

        let res_temp = self.build_request(self.build_url("recent")).send().await?;

        //dbg!(&res_temp);
        let res = res_temp.text().await?;

        //dbg!(&res);

        let temp = res
            .after(r#"RSS" href=""#)
            .context("failed to split in order to get Rss url")?;
        println!("\n\n");
        let url_temp = temp
            .before(r#"</head>"#)
            .context("failed to split in order to get Rss url")?;

        let url = url_temp
            .before("\">")
            .context("failed to split in order to get Rss url")?;

        Ok(url.to_owned())
    }
    /*
     * Fonction permettant de parser le flux RSS où:
     * url: Url vers le flux RSS
     *
     * => Résultat avec le vecteur des Fichiers
     */
    async fn get_files(&self) -> Result<Vec<CdpFile>> {
        let url = self.get_rss_url().await?;
        dbg!(&url);
        let xml = self
            .build_request(self.build_url(&url))
            .send()
            .await?
            .text()
            .await?;

        //dbg!(&xml);

        Ok(xml
            .split("<item>")
            .skip(1)
            .into_iter()
            .flat_map(|file| {
                let content = file.after("<![CDATA[");
                let fname = content.before("]]>").unwrap();

                let link = content.after("<link>");
                let url = link.before("</link>").unwrap();

                let path_temp = content.after("<a href");
                if let Some(path_temp_sm) = path_temp {
                    let path_temp2 = path_temp.after("\">");

                    let fpath = path_temp2.before("</a>]]>").unwrap();

                    if !fpath.starts_with("semaine") {
                        Some(CdpFile::new(
                            fname.to_string(),
                            url.to_string(),
                            fpath.to_string(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>())
    }

    /*
     * Fonction permettant de parser un dossier de fichier où:
     *   enter_url: url d'entrée du dossier de fichier
     *   path: chemin d'accès de base des fichiers
     *
     *   => Vec de fichier
     */
    #[async_recursion]
    async fn parse_folder(&self, enter_url: String, path: &String) -> Result<Vec<CdpFile>> {
        let content = self
            .build_request(self.build_url(&enter_url))
            .send()
            .await?
            .text()
            .await?;
        //dbg!(&content);
        let mut res = Vec::new();

        for el in content.split("<p class=\"").skip(1) {
            if el.contains("icon-minilock") {
                continue;
            }
            let inner = el.before("</a></p>");
            let type_element = inner.before(r#""><span"#).context("failed to parse type")?;
            //dbg!(type_element);
            if type_element == "rep" {
                let link_after = inner.after(r#"<a href=""#);

                let link = link_after
                    .before(r#"">"#)
                    .context("failed to parse link")?
                    .to_owned();

                let folder_name_after = inner.after(r#"class="nom">"#);
                let folder_name = folder_name_after
                    .before(r#"<"#)
                    .context("failed to parse name of a folder")?;

                let mut new_path = path.clone();
                new_path.push_str("/");
                new_path.push_str(&folder_name.window_string());

                //dbg!(self.parse_folder(format!("docs{}",link), &new_path).await?).append(&mut res);
                res.append(
                    &mut self
                        .parse_folder(format!("docs{}", link), &new_path)
                        .await?,
                );
            } else if type_element == "doc" {
                let link_after = inner.after(r#"<a href=""#);

                let link = link_after
                    .before(r#"""#)
                    .context("failed to parse link")?
                    .to_owned();

                let ftype_after = inner.after(r#"class="docdonnees">("#);
                let ftype = ftype_after
                    .before(",")
                    .context("failed to parse file type")?;

                let file_name_after = inner.after(r#"class="nom">"#);
                //dbg!(file_name_after);
                let file_name = file_name_after
                    .before("<")
                    .context("failed to parse name of a folder")?;

                res.push(CdpFile::new(
                    format!("{}.{}", file_name.window_string(), ftype),
                    self.build_url(&link),
                    path.to_owned(),
                ))
            }
        }
        Ok(res)
    }

    /*
     * Fonction permettant de parser le menu de cahier de prépa
     *
     *   => Vec d'un tuple où le premier élément est le nom du dossier et le second l'url d'entrée
     */
    async fn parse_menu(&self) -> Result<Vec<(String, String)>> {
        let mut res = Vec::new();
        let content = self
            .client
            .get(self.build_url(""))
            .send()
            .await?
            .text()
            .await?;
        for section in content.split("<h3>").skip(1) {
            if section.contains(r#"<a href="docs"#) {
                let title = section
                    .before("<")
                    .context("failed to parse menu section title")?
                    .to_string();
                let url = section
                    .split("<a")
                    .skip(1)
                    .map(|balise| {
                        let link_after = balise.after(r#"href=""#);
                        link_after.before(r#"">"#).unwrap().to_string()
                    })
                    .find(|link| link.contains("docs"))
                    .context("failed to find the link to the file page")?;

                res.push((title, url))
            }
        }
        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    //Parse la configuration
    /* let conf_path = env::args()
    .nth(1)
    .context("failed to parse configuration filepath")?;
    let content = fs::read_to_string(conf_path).await?;
    let config: CdpConfig = toml::from_str(&content)?; */
    let config: CdpConfig = Args::parse().into();

    let mut parser = CdpParser::new(config.credential, config.spacename);
    parser.auth().await?;

    let menu = parser.parse_menu().await?;

    let parser_atomic = Arc::new(parser);

    for (base_path, base_url) in menu.iter() {
        let files = parser_atomic
            .parse_folder(base_url.to_string(), base_path)
            .await?;

        for file in files {
            //let cookie = parser.cookie.clone();
            let base_path = config.path_to_save.clone();
            //dbg!(file.save(cookie, base_path).await);
            let cloned = parser_atomic.clone();

            tokio::spawn(async move {
                file.save(base_path, cloned)
                    .await
                    .context("failed to parse file")
            })
            .await?
            .unwrap();
        }
    }
    Ok(())
}
