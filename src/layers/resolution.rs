use strsim::levenshtein;
use crate::layers::PaperMetadata;

pub struct Resolver;

impl Resolver {
    pub fn resolve(query_title: &str, candidates: Vec<PaperMetadata>, threshold: usize) -> Vec<(PaperMetadata, usize)> {
        candidates.into_iter()
            .map(|p| {
                let dist = levenshtein(query_title, &p.title);
                (p, dist)
            })
            .filter(|(_, dist)| *dist <= threshold)
            .collect()
    }

    pub fn sort_by_similarity(mut matches: Vec<(PaperMetadata, usize)>) -> Vec<(PaperMetadata, usize)> {
        matches.sort_by_key(|(_, dist)| *dist);
        matches
    }
}
