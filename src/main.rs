use std::{fs::File, io::{Read, Write, BufReader, BufWriter}};

const AI_JS: &[u8] = include_bytes!("ai.js");
const PKG: &[u8] = include_bytes!("pkg.zip");

fn main() {
    modify_js();
    modify_html();
    create_ai_js();
    modify_manifest();
    unzip_pkg();
}

fn unzip_pkg() {
    let pkg = File::options().read(true).write(true).create(true).open("2.0.6_0/static/js/pkg.zip").unwrap();
    let mut pkg_buf = BufWriter::new(&pkg);
    pkg_buf.write_all(PKG).unwrap();
    let mut archive = zip::ZipArchive::new(&pkg).unwrap();

    std::fs::create_dir_all("2.0.6_0/static/js/pkg").unwrap();

    std::env::set_current_dir("2.0.6_0/static/js/pkg").unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

fn create_ai_js() {
    let ai = File::create("2.0.6_0/static/js/ai.js").unwrap();
    let mut ai_buf = BufWriter::new(&ai);
    ai_buf.write_all(AI_JS).unwrap();
}

fn modify_js() {
    let file = File::options().read(true).open("2.0.6_0/static/js/main.e0be237f.chunk.js").unwrap();
    let mut buf = String::new();
    {
        let mut reader = BufReader::new(&file);
        let _ = reader.read_to_string(&mut buf).unwrap();
    }
    if !buf.contains("start_ai") {
        let write_file = File::options().write(true).truncate(true).open("2.0.6_0/static/js/main.e0be237f.chunk.js").unwrap();
        let mut writer = BufWriter::new(&write_file);
        let _ = writer.write_all(buf.replace(r#"{var e;m("ingame"),Z.setShadow(!1),Z"#, r#"{globalThis.game = Z;start_ai();var e;m("ingame"),Z.setShadow(!1),Z"#).as_bytes()).unwrap();
    }
}

fn modify_manifest() {
    let file = File::options().read(true).open("2.0.6_0/manifest.json").unwrap();
    let mut buf = String::new();
    {
        let mut reader = BufReader::new(&file);
        let _ = reader.read_to_string(&mut buf).unwrap();
    }
    if !buf.contains("unsafe-eval") {
        let write_file = File::options().write(true).truncate(true).open("2.0.6_0/manifest.json").unwrap();
        let mut writer = BufWriter::new(&write_file);
        let _ = writer.write_all(buf.replace("'self'", "'self' 'unsafe-eval'").as_bytes()).unwrap();
    }
}

fn modify_html() {
    let file = File::options().read(true).open("2.0.6_0/index.html").unwrap();
    let mut buf = String::new();
    {
        let mut reader = BufReader::new(&file);
        let _ = reader.read_to_string(&mut buf).unwrap();
    }
    if !buf.contains("ai.js") {
        let write_file = File::options().write(true).truncate(true).open("2.0.6_0/index.html").unwrap();
        let mut writer = BufWriter::new(&write_file);
        let _ = writer.write_all(buf.replace(r#"<script src="/static/js/runtime-main.6317c19c.js"></script>"#, r#"<script src="/static/js/pkg/tetris_ai.js"></script><script src="/static/js/ai.js"></script><script src="/static/js/runtime-main.6317c19c.js"></script>"#).as_bytes()).unwrap();
    }
}