
#[path = "./makedir.rs"] mod makedir;

pub fn set_wallpaper_jpg() {
    let mut jpg_files = Vec::new();
    // let dir_val = crtDirectories::return_wallpaper_storage();
    // let dir_val = makedir::return_wallpaper_storage();

    let dir_val = makedir::storage_insideproject();
    
    for entry in std::fs::read_dir(dir_val).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap() == "jpg" {
            jpg_files.push(path);
        }
    }

    loop {
        for file in &jpg_files {
            let temp_str = file.to_str().unwrap();
            wallpaper::set_from_path(temp_str).unwrap();
            // std::thread::sleep(std::time::Duration::from_secs(1));
            std::thread::sleep(std::time::Duration::from_millis(100));
            println!("Set wallpaper to {}", temp_str);
        }
    }
}