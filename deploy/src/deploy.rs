mod conf;
mod types;

use types::{FilterOptMode, NoscriptPayload, NOSCRIPT_KIND};

use base64::{engine::general_purpose, Engine};
use nostr_sdk::prelude::*;
use std::{fs::File, io::Read, str::FromStr};

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
    let pks = vec![
        "fa984bd7dbb282f07e16e7ae87b26a2a7b9b90b7246a44771f0cf5ae58018f52",
        "9be0be0e64d38a29a9cec9a5c8ef5d873c2bfa5362a4b558da5ff69bc3cbb81e",
        "de7ecd1e2976a6adb2ffa5f4db81a7d812c8bb6698aa00dcf1e76adb55efd645",
        "3356de61b39647931ce8b2140b2bab837e0810c0ef515bbe92de0248040b8bdd",
        "76c71aae3a491f1d9eec47cba17e229cda4113a0bbb6e6ae1776d7643e29cafa",
        "97c70a44366a6535c145b333f973ea86dfdc2d7a99da618c40c64705ad98e322",
        "6b9da920c4b6ecbf2c12018a7a2d143b4dfdf9878c3beac69e39bb597841cc6e",
        "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d",
        "45c41f21e1cf715fa6d9ca20b8e002a574db7bb49e96ee89834c66dac5446b7a",
        "460c25e682fda7832b52d1f22d3d22b3176d972f60dcdc3212ed8c92ef85065c",
    ];
    let authors: Vec<XOnlyPublicKey> = pks
        .iter()
        .map(|pk| XOnlyPublicKey::from_str(pk).unwrap())
        .collect();
    let filter: Filter = Filter::new()
        .kinds(vec![Kind::TextNote, Kind::LongFormTextNote])
        .authors(authors);

    let id = "talking-nostr";

    let noscript_payload = NoscriptPayload {
        title: Some("Talking Nostr".to_string()),
        description: Some("a list of selected people talking about nostr".to_string()),
        version: Some("0.1.1".to_string()),
        source_code: Some("https://github.com/digi-monkey/noscript-boilerplate/tree/talking-nostr".to_string()),
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

    let tag = Tag::Generic(TagKind::from("mode"), vec![mode.to_string()]);
    tags.push(tag);

    let tag = Tag::Generic(
        TagKind::from("noscript"),
        vec!["wasm:msg:filter".to_string()],
    );
    tags.push(tag);

    return tags;
}

/*

1
:
"fa984bd7dbb282f07e16e7ae87b26a2a7b9b90b7246a44771f0cf5ae58018f52"
2
:
"9be0be0e64d38a29a9cec9a5c8ef5d873c2bfa5362a4b558da5ff69bc3cbb81e"
3
:
"de7ecd1e2976a6adb2ffa5f4db81a7d812c8bb6698aa00dcf1e76adb55efd645"
4
:
"3356de61b39647931ce8b2140b2bab837e0810c0ef515bbe92de0248040b8bdd"
5
:
"76c71aae3a491f1d9eec47cba17e229cda4113a0bbb6e6ae1776d7643e29cafa"
6
:
"97c70a44366a6535c145b333f973ea86dfdc2d7a99da618c40c64705ad98e322"
7
:
"6b9da920c4b6ecbf2c12018a7a2d143b4dfdf9878c3beac69e39bb597841cc6e"
8
:
"3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d"
9
:
"45c41f21e1cf715fa6d9ca20b8e002a574db7bb49e96ee89834c66dac5446b7a"
10
:
"460c25e682fda7832b52d1f22d3d22b3176d972f60dcdc3212ed8c92ef85065c"


*/
