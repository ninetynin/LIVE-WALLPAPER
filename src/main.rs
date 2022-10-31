use std::{io, path::PathBuf};

// import file
#[path = "./makedir.rs"] mod makedir;
#[path = "./setwallpaper.rs"] mod setwallpaper;
#[path = "./executepy.rs"] mod executepy;

fn main() {
    // -----------------
    makedir::create_all_dirs();
    // -----------------
    executepy::exectythepy();
    // -----------------
    setwallpaper::set_wallpaper_jpg();
    // -----------------
}