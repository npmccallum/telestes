// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: Apache-2.0

const GENERA: &[&str] = &[
    "application",
    "audio",
    "font",
    "image",
    "message",
    "model",
    "multipart",
    "text",
    "video",
];

fn main() {
    println!("// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>");
    println!("// SPDX-License-Identifier: Apache-2.0");
    println!("//");
    println!("// This file is auto-generated. Do not modify.");
    println!("// To regenerate the file, execute the following command:");
    println!("// cargo run --manifest-path=dbgen/Cargo.toml | rustfmt > src/db.rs");
    println!();
    println!("//! A Database of all IANA-registered Media Types");
    println!();
    println!("#![allow(missing_docs, clippy::panic)]");
    println!();
    println!("use crate::Essence;");
    println!();

    for genus in GENERA {
        let url = format!("https://www.iana.org/assignments/media-types/{}.csv", genus);
        let csv = ureq::get(&url).call().unwrap().into_string().unwrap();

        for line in csv.lines() {
            if line.contains("DEPRECATED") {
                continue;
            }

            let (_, rhs) = line.split_once(',').unwrap();
            let (mime, _) = rhs.split_once(',').unwrap();

            if !mime.starts_with(genus) {
                continue;
            }

            let species = &mime[genus.len() + 1..];
            let species = species
                .replace('+', "_")
                .replace('-', "_")
                .replace('_', "_")
                .replace('.', "_")
                .to_ascii_uppercase();

            println!("/// `{}`", mime);
            println!(
                "pub const {}_{}: Essence<&'static str> = match Essence::new_const(\"{}\") {{",
                genus.to_ascii_uppercase(),
                species,
                mime
            );
            println!("    Ok(essence) => essence,");
            println!("    Err(..) => panic!(\"invalid: {}\"),", mime);
            println!("}};");
            println!();
        }
    }
}
