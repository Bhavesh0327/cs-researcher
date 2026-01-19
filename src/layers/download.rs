use anyhow::{Result, anyhow};
use reqwest::Client;
use std::path::{PathBuf};
use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::layers::PaperMetadata;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ManifestEntry {
    title: String,
    first_author: String,
    year: Option<u32>,
    id: String,
    relative_path: String,
    downloaded_at: String,
}

pub struct Downloader {
    client: Client,
    base_dir: PathBuf,
}

impl Downloader {
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            client: Client::new(),
            base_dir: base_dir.into(),
        }
    }

    pub async fn download_paper(&self, paper: &PaperMetadata) -> Result<PathBuf> {
        if !paper.is_oa {
            return Err(anyhow!("Paper is not Open Access, skipping download."));
        }

        let pdf_url = paper.pdf_url.as_ref()
            .ok_or_else(|| anyhow!("No PDF URL found for paper despite OA status."))?;

        let raw_id = paper.doi.as_ref()
            .or(paper.arxiv_id.as_ref())
            .or(paper.semantic_scholar_id.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("unknown_id");

        // Sanitize ID: remove scheme, replace non-alphanumeric chars
        let paper_id = raw_id.replace("http://", "")
                             .replace("https://", "")
                             .replace(|c: char| !c.is_alphanumeric() && c != '.' && c != '-', "_");

        // Download PDF
        tracing::info!("Downloading PDF from: {}", pdf_url);
        let mut response = self.client.get(pdf_url).send().await?;
        
        if !response.status().is_success() {
            let err = format!("Failed to download PDF: {}", response.status());
            tracing::error!("{}", err);
            return Err(anyhow!(err));
        }

        // Only create directory if request was successful
        let target_dir = self.base_dir.join(&paper_id);
        create_dir_all(&target_dir).await?;
        
        let pdf_path = target_dir.join("paper.pdf");
        let mut file = File::create(&pdf_path).await?;
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }

        // Save Metadata
        let metadata_path = target_dir.join("metadata.json");
        tracing::info!("Saving metadata to: {:?}", metadata_path);
        let metadata_json = serde_json::to_string_pretty(paper)?;
        let mut meta_file = File::create(&metadata_path).await?;
        meta_file.write_all(metadata_json.as_bytes()).await?;

        // Update Manifest
        self.update_manifest(paper, &paper_id, &pdf_path).await?;

        Ok(target_dir)
    }

    async fn update_manifest(&self, paper: &PaperMetadata, id: &str, pdf_path: &std::path::Path) -> Result<()> {
        let manifest_path = self.base_dir.join("manifest.json");
        let mut entries: Vec<ManifestEntry> = if manifest_path.exists() {
            let mut file = File::open(&manifest_path).await?;
            let mut content = String::new();
            file.read_to_string(&mut content).await?;
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        let first_author = paper.authors.first().map(|s| s.as_str()).unwrap_or("Unknown").to_string();
        let relative_path = pdf_path.strip_prefix(&self.base_dir)
            .unwrap_or(pdf_path)
            .to_string_lossy()
            .into_owned();

        let new_entry = ManifestEntry {
            title: paper.title.clone(),
            first_author,
            year: paper.year,
            id: id.to_string(),
            relative_path,
            downloaded_at: Utc::now().to_rfc3339(),
        };

        // Remove existing entry with same ID if exists (update)
        entries.retain(|e| e.id != id);
        entries.push(new_entry);

        let json = serde_json::to_string_pretty(&entries)?;
        let mut file = File::create(&manifest_path).await?;
        file.write_all(json.as_bytes()).await?;
        tracing::info!("Updated manifest at: {:?}", manifest_path);

        Ok(())
    }
}
