use crate::config::{self, ModelManifest};
use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode; // Ajout de l'import
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
}

#[derive(Deserialize)]
pub struct GitTreeResponse {
    pub tree: Vec<GitTreeItem>,
}

#[derive(Deserialize)]
pub struct GitTreeItem {
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

const GITHUB_API_URL: &str = "https://api.github.com";

pub fn fetch_model_manifest(client: &Client) -> ModelManifest {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/main/models.json",
        config::REPO_OWNER,
        config::REPO_NAME
    );

    match client.get(&url).send() {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<ModelManifest>() {
                    Ok(manifest) => return manifest,
                    Err(e) => eprintln!("   ⚠️  Erreur parsing models.json : {e}"),
                }
            }
        }
        Err(_) => {}
    }

    println!("   ⚠️  Impossible de récupérer la liste à jour. Utilisation des modèles intégrés.");
    ModelManifest::fallback()
}

pub fn fetch_latest_tag(client: &Client) -> Result<String> {
    let url = format!(
        "{}/repos/{}/{}/releases/latest",
        GITHUB_API_URL,
        config::REPO_OWNER,
        config::REPO_NAME
    );
    let resp = client.get(&url).send()?;

    if !resp.status().is_success() {
        bail!(
            "Erreur GitHub API : {} (Vérifiez que le dépôt a une Release publique)",
            resp.status()
        );
    }

    let release: GithubRelease = resp.json()?;
    Ok(release.tag_name)
}

pub fn fetch_repo_tree(client: &Client, tag: &str) -> Result<Vec<GitTreeItem>> {
    let url = format!(
        "{}/repos/{}/{}/git/trees/{}?recursive=1",
        GITHUB_API_URL,
        config::REPO_OWNER,
        config::REPO_NAME,
        tag
    );
    let resp = client.get(&url).send()?;

    if !resp.status().is_success() {
        bail!(
            "Impossible de lire l'arborescence du dépôt : {}",
            resp.status()
        );
    }

    let tree_resp: GitTreeResponse = resp.json()?;
    Ok(tree_resp.tree)
}

pub fn download_file(
    client: &Client,
    tag: &str,
    remote_path: &str,
    local_path: &Path,
) -> Result<()> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        config::REPO_OWNER,
        config::REPO_NAME,
        tag,
        remote_path
    );
    let resp = client.get(&url).send()?;

    if resp.status() == StatusCode::NOT_FOUND {
        bail!("Fichier distant introuvable (404) : {}", remote_path);
    }

    if !resp.status().is_success() {
        bail!("Erreur téléchargement {} : {}", remote_path, resp.status());
    }

    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = resp.text().context("Erreur lecture content")?;
    fs::write(local_path, content).context("Erreur écriture disque")?;

    println!(
        "   📄 Installé : {:?}",
        local_path.file_name().unwrap_or_default()
    );
    Ok(())
}
