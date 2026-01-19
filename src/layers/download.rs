use anyhow::{Result, anyhow};
use reqwest::Client;
use std::path::{PathBuf};
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use crate::layers::PaperMetadata;

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

        let paper_id = paper.doi.as_ref()
            .or(paper.arxiv_id.as_ref())
            .or(paper.semantic_scholar_id.as_ref())
            .map(|s| s.replace('/', "_"))
            .unwrap_or_else(|| "unknown_id".to_string());

        let target_dir = self.base_dir.join(&paper_id);
        create_dir_all(&target_dir).await?;

        // Download PDF
        let pdf_path = target_dir.join("paper.pdf");
        let mut response = self.client.get(pdf_url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to download PDF: {}", response.status()));
        }

        let mut file = File::create(&pdf_path).await?;
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }

        // Save Metadata
        let metadata_path = target_dir.join("metadata.json");
        let metadata_json = serde_json::to_string_pretty(paper)?;
        let mut meta_file = File::create(&metadata_path).await?;
        meta_file.write_all(metadata_json.as_bytes()).await?;

        Ok(target_dir)
    }
}
