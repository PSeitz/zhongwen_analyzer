use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use tantivy::schema::*;
use tantivy::{doc, Index};
use tantivy_index::{open_index, search};

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
    //let reader = index.reader()?;
    //let searcher = reader.searcher();

    run_merge(index_path.to_path_buf())?;

    let index = open_index(index_path.to_str().unwrap())?;
    search("traditional:下午 AND pinyin_pretty:\"xià wǔ\"", &index)?;
    Ok(())
}

const HEAP_SIZE: usize = 300_000_000;
pub fn run_merge(path: PathBuf) -> tantivy::Result<()> {
    let index = Index::open_in_dir(&path)?;
    let segments = index.searchable_segment_ids()?;
    let segment_meta = index
        .writer::<TantivyDocument>(HEAP_SIZE)?
        .merge(&segments)
        .wait()?;
    println!("Merge finished with segment meta {:?}", segment_meta);
    println!("Garbage collect irrelevant segments.");
    Index::open_in_dir(&path)?
        .writer_with_num_threads::<TantivyDocument>(1, 40_000_000)?
        .garbage_collect_files()
        .wait()?;
    Ok(())
}
