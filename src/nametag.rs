use super::db::models::*;
use tempfile::TempDir;
use reqwest;
use hyper::header::Headers;
use std::process::Command;
use std::path::PathBuf;

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

fn render_nametag(name: &str) -> Result<(), &'static str> {
    let outpath = TEMP_DIR.path().join(format!("{}.stl", name));
    if !outpath.exists() {
        match Command::new("openscad")
            .arg("-o").arg(&outpath)
            .args(&[format!("-D name=\"{}\"", name), 
                    format!("-D chars={}", name.len())])
            .arg("openscad/name.scad")
            .status() {
            Ok(status) => {
                if status.success() {
                    return Ok(())
                } else {
                    return Err("Failed to generate stl");
                }
            }
            Err(_) => {
                return Err("Failed to generate stl");
            }
        }
    } else {
        return Ok(())
    }
}

fn slice_nametag(name: &str, slic3r_conf: &str) -> Result<(), &'static str> {
    let outpath = TEMP_DIR.path().join(format!("{}.gcode", name));
    if !outpath.exists() {
        if let Some(inpath) = TEMP_DIR.path().join(format!("{}.stl", name)).to_str() {
            match Command::new("slic3r")
                .arg("-o").arg(&outpath)
                .args(&[format!("--load {}", slic3r_conf), 
                        format!("{}", inpath)])
                .arg("openscad/name.scad")
                .status() {
                Ok(status) => {
                    if status.success() {
                        return Ok(())
                    } else {
                        return Err("Failed to generate gcode");
                    }
                }
                Err(_) => {
                    return Err("Failed to generate gcode");
                }
            }
        } else {
            return Err("Invalid stl path")
        }
    } else {
        Ok(())
    }
}

header! { (XApiKey, "X-Api-Key") => [String] }

fn send_to_printer(name: &str, printer: Printer) -> Result<(), &'static str> {
    let gcodepath = TEMP_DIR.path().join(format!("{}.gcode", name));
    let client = reqwest::Client::new();
    let form = match reqwest::multipart::Form::new()
        .file("file", gcodepath) {
            Ok(form) => form,
            Err(_) => {
                return Err("Could not create form");
            }
    };
    let url = format!("http://{}/api/files/local", printer.ip);
    client.post(&url)
        .header(XApiKey::new(printer.key))
        .multipart(form)
        .send()
        .map_err(|_| "Failed to upload file to printer")?;
    Ok(())
}
