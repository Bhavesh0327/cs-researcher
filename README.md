# CS Researcher: Automated Paper Downloader

A Rust-based tool for automated discovery, resolution, and downloading of open-access research papers.

## Features

- **Layered Discovery**: Searches across Semantic Scholar, arXiv, and OpenAlex.
- **Fuzzy Resolution**: Matches search results to your target title using Levenshtein distance.
- **Legality Enforcement**: Downloads only Open Access (OA) papers to ensure compliance.
- **Automated Metadata**: Stores paper metadata (JSON) alongside the PDF in a structured directory.

## Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Compatible with 1.82.0+)

### Configuration
1. Clone the repository.
2. The tool will auto-create a `.env` file from `.env.example` on the first run.
3. Add your API keys/email to the `.env` file:
   - `OPENALEX_EMAIL`: Required for the OpenAlex "polite pool".
   - `SEMANTIC_SCHOLAR_API_KEY`: Highly recommended to avoid 429 Rate Limit errors.

## Usage

Run the project using Cargo:

```bash
cargo run
```

Currently, the search query is configured in `src/main.rs`. In the next version, this will be available via CLI arguments.

## Download Options

Papers are downloaded to the directory specified in your `.env` file (default: `downloads/`).
Each paper gets its own folder named after its DOI or ID, containing:
- `paper.pdf`: The full-text PDF.
- `metadata.json`: Comprehensive metadata including authors, abstract, and source IDs.

## Contributing

Interested in contributing? Please check out our [Contributing Guidelines](CONTRIBUTING.md) for more information.

## License
MIT
