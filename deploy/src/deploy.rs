mod conf;

extern crate types;
use types::{NoscriptContent, NoscriptPayload, NOSCRIPT_KIND};

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

    // build Noscript Event
    let id = String::from("Japanese-Lang");
    let title = String::from("follow");
    let description = String::from("a noscript that filter japanese text only");
    let version = String::from("0.1.0");

    let content = build_content();

    let noscript_payload = NoscriptPayload {
        title: Some(title),
        description: Some(description),
        version: Some(version),
        ..Default::default()
    };

    let d_tags = create_d_tag(Some(id));
    let noscript_tags = create_noscript_payload_tag(noscript_payload);

    let event: Event = EventBuilder::new(
        Kind::Custom(NOSCRIPT_KIND.try_into().unwrap()),
        content.to_string(),
        vec![noscript_tags, d_tags].concat(),
    )
    .to_event(&my_keys)?;
    println!("{:#?}", event.id);
    client.send_event(event).await?;

    Ok(())
}

pub fn build_content() -> NoscriptContent {
    let wasm = read_wasm();
    let js_binding = read_js_binding();
    return NoscriptContent {
        wasm,
        binding: js_binding,
    };
}

pub fn read_wasm() -> String {
    let wasm_file_path = "../script/pkg/script_bg.wasm";
    let mut wasm_file = File::open(wasm_file_path).expect("Failed to open .wasm file");
    let mut wasm_bytes = Vec::new();
    wasm_file
        .read_to_end(&mut wasm_bytes)
        .expect("Failed to read .wasm file");

    let wasm_base64 = general_purpose::STANDARD.encode(&wasm_bytes);

    return wasm_base64;
}

pub fn read_js_binding() -> String {
    let file_path = "../script/pkg/script.js";
    let mut file = File::open(file_path).expect("Failed to open .wasm file");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .expect("Failed to read .wasm file");

    let base64 = general_purpose::STANDARD.encode(&bytes);

    //println!("Base64-encoded .wasm file:\n{}", wasm_base64);

    return base64;
}

pub fn create_d_tag(id: Option<String>) -> Vec<Tag> {
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

    if payload.runtime_version.is_some() {
        let tag = Tag::Generic(
            TagKind::from("runtime_version"),
            vec![payload.runtime_version.unwrap().to_string()],
        );
        tags.push(tag);
    }

    return tags;
}
