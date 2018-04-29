use std::process::Command;
use std::process::ExitStatus;
use std::io;

pub fn preview(name: &str) -> io::Result<ExitStatus> {
    let zoom;
    if name.len() > 7 {
        zoom = 120;
    } else if name.len() > 4 {
        zoom = 105;
    } else {
        zoom = 70;
    }
    
     Command::new("openscad")
        .arg("-o").arg(format!("{}.png", name))
        .args(&[format!("-D name=\"{}\"", name), 
                format!("-D chars={}", name.len()),
                format!("--camera=0,0,0,0,0,0,{}", zoom)])
        .arg("--imgsize=512,400")
        .arg("openscad/name.scad")
        .status()
}
