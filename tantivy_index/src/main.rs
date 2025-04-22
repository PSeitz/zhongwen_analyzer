use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use tantivy::schema::*;
use tantivy::{doc, Index};

fn main() -> tantivy::Result<()> {
    // --- 1. Build the schema -------------------------------------------------
    let mut schema_builder = Schema::builder();

    let simplified = schema_builder.add_text_field("simplified", TEXT | STORED);
    let traditional = schema_builder.add_text_field("traditional", TEXT | STORED);
    let pinyin = schema_builder.add_text_field("pinyin", TEXT | STORED);
    let pinyin_pretty = schema_builder.add_text_field("pinyin_pretty", TEXT | STORED);
    let zhuyin = schema_builder.add_text_field("zhuyin", TEXT | STORED);
    let meanings = schema_builder.add_text_field("meanings", TEXT | STORED);
    let meanings_de = schema_builder.add_text_field("meanings_de", TEXT | STORED);
    let tags_field = schema_builder.add_text_field("tags", TEXT | STORED);

    let commonness_boost = schema_builder.add_f64_field("commonness_boost", FAST | STORED);
    let cpm_written = schema_builder.add_u64_field("count_per_million_written", FAST | STORED);
    let cpm_spoken = schema_builder.add_u64_field("count_per_million_spoken", FAST | STORED);
    let cpm_others = schema_builder.add_u64_field("count_per_million_in_others", FAST | STORED);

    let schema = schema_builder.build();

    // --- 2. Create / open index directory -----------------------------------
    let index_path = Path::new("./dict_index");
    let index = Index::create_in_dir(index_path, schema.clone())?;

    // --- 3. Prepare writer ---------------------------------------------------
    let mut index_writer = index.writer(500_000_000)?; // 500 MB mem buffer

    // --- 4. Stream ND‑JSON and add documents --------------------------------
    let file = File::open("db.json")?; // your two JSON lines (or more)
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let document = TantivyDocument::parse_json(&schema, &line)?;

        index_writer.add_document(document)?;
    }

    index_writer.commit()?;

    // --- 4b. Merge *all* segments into a single one --------------------------
    let seg_ids = index.searchable_segment_ids()?; // collect flushed segment IDs
    index_writer.merge(&seg_ids); // schedule merge
    index_writer.wait_merging_threads()?; // block until merge finished

    // --- 5. Tiny demo query --------------------------------------------------
    let reader = index.reader()?;
    let searcher = reader.searcher();

    let query_parser =
        tantivy::query::QueryParser::for_index(&index, vec![traditional, pinyin_pretty]);
    let query = query_parser.parse_query("traditional:下午 AND pinyin_pretty:\"xià wǔ\"")?;

    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}", retrieved.to_json(&schema));
    }

    Ok(())
}
