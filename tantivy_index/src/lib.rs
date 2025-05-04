use serde::{Deserialize, Deserializer, Serialize};
use tantivy::{Document, TantivyDocument};

pub use tantivy::Index;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Entry {
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    pub simplified: String,
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    pub traditional: String,
    #[serde(default)]
    pub simplified_radicals: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub traditional_radicals: Option<Vec<Vec<String>>>,
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    pub pinyin: String,
    #[serde(default)]
    pub pinyin_taiwan: Option<String>,
    // different pinyin variants for search. this could be covered by
    // tokenization but that's simpler
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    pub zhuyin: String,
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    pub pinyin_pretty: String,
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    #[serde(default)]
    tocfl_level: Option<u32>,
    pub meanings: Vec<String>,
    #[serde(default)]
    pub meanings_de: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    pub commonness_boost: f64,
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    count_per_million_written: u64,
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    count_per_million_spoken: u64,
    #[serde(deserialize_with = "deserialize_val_from_vec")]
    count_per_million_in_others: u64,
}

pub fn deserialize_val_from_vec<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + 'static,
{
    let mut vec: Vec<T> = Deserialize::deserialize(deserializer)?;
    Ok(vec.remove(0))
}

pub fn search(query: &str, index: &Index) -> tantivy::Result<Vec<Entry>> {
    let schema = index.schema();
    let traditional = schema.get_field("traditional").unwrap();
    let pinyin_pretty = schema.get_field("pinyin_pretty").unwrap();

    let query_parser =
        tantivy::query::QueryParser::for_index(index, vec![traditional, pinyin_pretty]);
    let query = query_parser.parse_query(query)?;
    //let query = query_parser.parse_query("traditional:下午 AND pinyin_pretty:\"xià wǔ\"")?;

    let searcher = index.reader()?.searcher();
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10))?;
    let mut hits: Vec<Entry> = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved: TantivyDocument = searcher.doc(doc_address)?;
        //println!("{}", retrieved.to_json(&schema));
        hits.push(serde_json::from_str(&retrieved.to_json(&schema)).unwrap());
    }
    Ok(hits)
}

pub fn open_index(index_path: &str) -> tantivy::Result<Index> {
    let index = Index::open_in_dir(index_path)?;
    Ok(index)
}
