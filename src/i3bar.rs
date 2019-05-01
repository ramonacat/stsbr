use crate::data_source::DataSource;
use serde::Serialize;
use std::borrow::ToOwned;
use std::string::ToString;

#[derive(Serialize)]
struct Header {
    version: u64,
    click_events: bool,
}

#[derive(Serialize)]
struct Block {
    full_text: String,
    markup: String,
    name: String,
    instance: String,
}

pub fn get_header_json(allow_click_events: bool) -> String {
    let header = Header {
        version: 1,
        click_events: allow_click_events,
    };

    return serde_json::to_string(&header).unwrap();
}

fn convert_sources_to_blocks(sources: &Vec<Box<DataSource>>) -> Vec<Block> {
    sources
        .iter()
        .map(|block| {
            let state = block.current_state();

            return Block {
                full_text: state.text().to_string(),
                markup: "pango".to_string(),
                instance: "fixme".to_string(),
                name: "blahblahblahfixme".to_owned(),
            };
        })
        .collect::<Vec<Block>>()
}

pub fn sources_to_json(sources: &Vec<Box<DataSource>>) -> String {
    let blocks = convert_sources_to_blocks(sources);

    serde_json::to_string(&blocks).unwrap()
}
