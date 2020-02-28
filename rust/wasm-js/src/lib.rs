use wasm_bindgen::prelude::*;
use lucid_suggest_core as core;


#[wasm_bindgen]
pub fn create_store() -> usize {
    core::create_store()
}


#[wasm_bindgen]
pub fn highlight_using(store_id: usize, left: &str, right: &str) {
    core::highlight_using(store_id, (left, right));
}


#[wasm_bindgen]
pub fn add_records(store_id: usize, ids: &[usize], texts: String) {
    let records = ids.iter()
        .cloned()
        .zip(texts.split('\0'));
    core::add_records(store_id, records)
}


#[wasm_bindgen]
pub fn search(store_id: usize, query: &str) {
    core::run_search(store_id, query)
}


#[wasm_bindgen]
pub fn get_result_ids(store_id: usize) -> Vec<usize> {
    core::using_results(store_id, |results| {
        results.iter().map(|r| r.id).collect()
    })
}


#[wasm_bindgen]
pub fn get_result_highlights(store_id: usize) -> String {
    core::using_results(store_id, |results| {
        let bytelen: usize = results.iter()
            .map(|result| result.highlighted.len())
            .sum();
        let mut concat = String::with_capacity(bytelen + results.len());
        for result in results {
            concat.push_str(&result.highlighted);
            concat.push('\0');
        }
        concat
    })
}
