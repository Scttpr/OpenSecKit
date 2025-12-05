use crate::config;
use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::path::Path;
use std::fs;

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

pub fn fetch_latest_tag(client: &Client) -> Result<String> {
    let url = format!("{}/repos/{}/{}/releases/latest", GITHUB_API_URL, config::REPO_OWNER, config::REPO_NAME);
    
    let resp = client.get(&url).send()?;

    if !resp.status().is_success() {
        bail!("Erreur GitHub API : {} (Vérifiez que le dépôt a une Release publique)", resp.status());
    }

    let release: GithubRelease = resp.json()?;
    Ok(release.tag_name)
}

pub fn fetch_repo_tree(client: &Client, tag: &str) -> Result<Vec<GitTreeItem>> {
    let url = format!(
        "{}/repos/{}/{}/git/trees/{}?recursive=1",
        GITHUB_API_URL, config::REPO_OWNER, config::REPO_NAME, tag
    );

    let resp = client.get(&url).send()?;
    
    if !resp.status().is_success() {
        bail!("Impossible de lire l'arborescence du dépôt : {}", resp.status());
    }

    let tree_resp: GitTreeResponse = resp.json()?;
    Ok(tree_resp.tree)
}

pub fn download_file(client: &Client, tag: &str, remote_path: &str, local_path: &Path) -> Result<()> {
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        config::REPO_OWNER, config::REPO_NAME, tag, remote_path
    );

    let resp = client.get(&url).send()?;

    if resp.status().as_u16() == 404 {
        println!("   ⚠️  Non trouvé (404) : {}", remote_path);
        return Ok(());
    }

    if !resp.status().is_success() {
        bail!("Erreur téléchargement {} : {}", remote_path, resp.status());
    }

    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let content = resp.text().context("Erreur lecture content")?;
    fs::write(local_path, content).context("Erreur écriture disque")?;
    
    println!("   📄 Installé : {:?}", local_path);
    Ok(())
}