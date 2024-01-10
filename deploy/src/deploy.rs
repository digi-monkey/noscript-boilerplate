mod conf;
mod types;

use types::{FilterOptMode, NoscriptPayload, NOSCRIPT_KIND};

use base64::{engine::general_purpose, Engine};
use nostr_sdk::prelude::*;
use std::{fs::File, io::Read};

#[tokio::main]
async fn main() -> Result<()> {
    let conf = conf::get_config();
    let my_keys = Keys::from_sk_str(&conf.privkey.as_str())?;
    let pubkey: String = my_keys.public_key().to_string();
    println!("PubKey: {}", pubkey);
    let relays = conf.relays;

    let client = Client::new(&my_keys);
    for relay in relays {
        println!("add relay: {}", relay);
        client.add_relay(relay).await?;
    }
    client.connect().await;

    // Send custom event
    let content = read_wasm();
    let filter: Filter = Filter::new().kind(Kind::TextNote);

    let id: &str = "Mandarin-Lang";

    let noscript_payload = NoscriptPayload {
        title: Some("中文广场".to_string()),
        description: Some("a noscript that filter Mandarin text only".to_string()),
        version: Some("0.1.0".to_string()),
        ..Default::default()
    };

    let d_tags = create_d_tag(Some(id.to_string()));
    let filter_tags = create_filter_tag(filter, FilterOptMode::global);
    let noscript_tags = create_noscript_payload_tag(noscript_payload);
    
    let event: Event = EventBuilder::new(
        Kind::Custom(NOSCRIPT_KIND.try_into().unwrap()),
        content,
        vec![filter_tags, noscript_tags, d_tags].concat(),
    )
    .to_event(&my_keys)?;
    println!("{:#?}", event.id);
    client.send_event(event).await?;

    Ok(())
}

pub fn read_wasm() -> String {
    let wasm_file_path = "../script/pkg/script_bg.wasm";
    let mut wasm_file = File::open(wasm_file_path).expect("Failed to open .wasm file");
    let mut wasm_bytes = Vec::new();
    wasm_file
        .read_to_end(&mut wasm_bytes)
        .expect("Failed to read .wasm file");

    let wasm_base64 = general_purpose::STANDARD.encode(&wasm_bytes);

    //println!("Base64-encoded .wasm file:\n{}", wasm_base64);

    return wasm_base64;
}

pub fn create_d_tag(id: Option<String>)-> Vec<Tag>{
    let mut tags: Vec<Tag> = vec![];

    if id.is_some() {
        let d = id.unwrap();
        let d2 = d.clone();
        let tag = Tag::Generic(TagKind::D, vec![d]);
        println!("noscript #d: {:#?}", d2);
        tags.push(tag);
    }

    return tags;
}

pub fn create_noscript_payload_tag(payload: NoscriptPayload) -> Vec<Tag> {
    let mut tags: Vec<Tag> = vec![];

    if payload.title.is_some() {
        let tag = Tag::Generic(TagKind::from("title"), vec![payload.title.unwrap()]);
        tags.push(tag);
    }

    if payload.description.is_some() {
        let tag = Tag::Generic(
            TagKind::from("description"),
            vec![payload.description.unwrap()],
        );
        tags.push(tag);
    }

    if payload.picture.is_some() {
        let tag = Tag::Generic(TagKind::from("picture"), vec![payload.picture.unwrap()]);
        tags.push(tag);
    }

    if payload.version.is_some() {
        let tag = Tag::Generic(TagKind::from("version"), vec![payload.version.unwrap()]);
        tags.push(tag);
    }

    if payload.source_code.is_some() {
        let tag = Tag::Generic(
            TagKind::from("source_code"),
            vec![payload.source_code.unwrap()],
        );
        tags.push(tag);
    }

    if payload.published_at.is_some() {
        let tag = Tag::Generic(
            TagKind::from("published_at"),
            vec![payload.published_at.unwrap().to_string()],
        );
        tags.push(tag);
    }

    return tags;
}

pub fn create_filter_tag(filter: Filter, mode: FilterOptMode) -> Vec<Tag> {
    let mut tags: Vec<Tag> = vec![];

    if filter.ids.len() > 0 {
        let tag = Tag::Generic(
            TagKind::from("ids"),
            filter.ids.iter().map(|id| id.to_string()).collect(),
        );
        tags.push(tag);
    }
    if filter.authors.len() > 0 {
        let tag = Tag::Generic(
            TagKind::from("authors"),
            filter.authors.iter().map(|id| id.to_string()).collect(),
        );
        tags.push(tag);
    }
    if filter.kinds.len() > 0 {
        let tag = Tag::Generic(
            TagKind::from("kinds"),
            filter.kinds.iter().map(|id| id.to_string()).collect(),
        );
        tags.push(tag);
    }
    if filter.limit.is_some() {
        let tag = Tag::Generic(
            TagKind::from("limit"),
            vec![filter.limit.unwrap().to_string()],
        );
        tags.push(tag);
    }
    if filter.since.is_some() {
        let tag = Tag::Generic(
            TagKind::from("since"),
            vec![filter.since.unwrap().to_string()],
        );
        tags.push(tag);
    }
    if filter.until.is_some() {
        let tag = Tag::Generic(
            TagKind::from("until"),
            vec![filter.until.unwrap().to_string()],
        );
        tags.push(tag);
    }
    if filter.generic_tags.len() > 0 {
        for t in filter.generic_tags {
            let tag = Tag::Generic(
                TagKind::from(format!("#{:#?}", t.0.to_string().to_lowercase())),
                t.1.iter().map(|v| v.to_string()).collect(),
            );
            tags.push(tag);
        }
    }

    let tag = Tag::Generic(
        TagKind::from("mode"),
        vec![mode.to_string()],
    );
    tags.push(tag);

    let tag = Tag::Generic(
        TagKind::from("noscript"),
        vec!["wasm:msg:filter".to_string()],
    );
    tags.push(tag);

    return tags;
}
