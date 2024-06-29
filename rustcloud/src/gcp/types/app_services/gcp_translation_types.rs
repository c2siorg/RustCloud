use serde::{Deserialize, Serialize};

// Structs for request payloads

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateRequest {
    // Define fields as per Google Cloud Translation API
    pub contents: Vec<String>,
    pub target_language_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchTranslateRequest {
    // Define fields as per Google Cloud Translation API
    pub source_language_code: String,
    pub target_language_codes: Vec<String>,
    pub contents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlossaryName {
    // Define fields as per Google Cloud Translation API
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGlossaryRequest {
    // Define fields as per Google Cloud Translation API
    pub parent: String,
    pub glossary: Glossary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Glossary {
    // Define fields as per Google Cloud Translation API
    pub language_pair: LanguagePair,
    pub input_config: InputConfig,
    pub entries: Vec<GlossaryEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguagePair {
    // Define fields as per Google Cloud Translation API
    pub source_language_code: String,
    pub target_language_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputConfig {
    // Define fields as per Google Cloud Translation API
    pub gcs_source: GcsSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GcsSource {
    // Define fields as per Google Cloud Translation API
    pub input_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlossaryEntry {
    // Define fields as per Google Cloud Translation API
    pub input: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectLanguageRequest {
    // Define fields as per Google Cloud Translation API
    pub content: String,
}



