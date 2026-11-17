use std::{env, fmt::Write, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rerun-if-changed=../../assets/secretarybird.ico");
    println!("cargo:rerun-if-changed=../../assets/tray.ico");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo output directory"));
    let mut resources = fs::read_to_string("app.rc").expect("base tray resources");
    // Each group contains the original ICO files, each preceded by its little-endian u32 size.
    for (device, base) in [("headset", 1000), ("mouse", 1200)] {
        let mut id = base;
        for group in 0..6 {
            let pack = format!("../../assets/battery-{device}-{group}.icons");
            println!("cargo:rerun-if-changed={pack}");
            let bytes = fs::read(pack).expect("read battery icon pack");
            let mut remaining = bytes.as_slice();
            while !remaining.is_empty() {
                let (size, rest) = remaining.split_at(4);
                let size = u32::from_le_bytes(size.try_into().expect("four-byte icon length"));
                let (icon, rest) = rest.split_at(size as usize);
                let file = out_dir.join(format!("battery-{id}.ico"));
                fs::write(&file, icon).expect("write battery icon");
                let path = file.to_string_lossy().replace('\\', "/");
                writeln!(resources, "{id} ICON \"{path}\"").expect("write tray resource");
                id += 1;
                remaining = rest;
            }
        }
        assert_eq!(id, base + 101, "expected an icon for every percentage");
    }
    let manifest_dir = env::current_dir().expect("current crate directory");
    let root = manifest_dir.to_string_lossy().replace('\\', "/");
    let resources = resources.replace("\"../../assets/", &format!("\"{root}/../../assets/"));
    let script = out_dir.join("secbird-tray.rc");
    fs::write(&script, resources).expect("write Windows resource script");
    embed_resource::compile(script, embed_resource::NONE);
}
