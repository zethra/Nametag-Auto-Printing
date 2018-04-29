use std::process::Command;
use std::io;
use std::path::PathBuf;
use tempfile::TempDir;

lazy_static! {
    static ref TEMP_DIR: TempDir = TempDir::new().expect("Couldn't create tmp dir");
}

pub fn preview(name: &str) -> Result<PathBuf, &'static str> {
    let outpath = TEMP_DIR.path().join(format!("{}.png", name));
    let zoom;
    if name.len() > 7 {
        zoom = 120;
    } else if name.len() > 4 {
        zoom = 105;
    } else {
        zoom = 70;
    }
    
    match Command::new("openscad")
        .arg("-o").arg(&outpath)
        .args(&[format!("-D name=\"{}\"", name), 
                format!("-D chars={}", name.len()),
                format!("--camera=0,0,0,0,0,0,{}", zoom)])
        .arg("--imgsize=512,400")
        .arg("openscad/name.scad")
        .status() {
        Ok(status) => {
            if status.success() {
                return Ok(PathBuf::from(outpath))
            } else {
                return Err("Failed to generate preview");
            }
        }
        Err(_) => {
            return Err("Failed to generate preview");
        }
    }
}
