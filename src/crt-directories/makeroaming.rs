use directories::ProjectDirs;
use std::fs;

pub fn create_all_dirs() {
    roaming_dir();
    // local_dir();
    wallpaper_storage();
    logs_storage();
}

fn roaming_dir() {
    let project_dirs = ProjectDirs::from("com", "Dynamic-Wallpaper", "").unwrap();
    let dir = project_dirs.config_dir(); // i.e., Roaming/Dynamic-Wallpaper
    // TODO: Log the path of the directory
    fs::create_dir_all(dir).unwrap();
    // TODO: Log the creation of the directory
}

fn wallpaper_storage() {
    let project_dirs = ProjectDirs::from("com", "Dynamic-Wallpaper", "").unwrap();
    let dir = project_dirs.config_dir().join("wallpaper-storage"); // i.e., Roaming\Dynamic-Wallpaper\wallpaper-storage 
    // TODO: Log the path of the directory
    fs::create_dir_all(dir).unwrap();
    // TODO: Log the creation of the directory
}

fn logs_storage() {
    let project_dirs = ProjectDirs::from("com", "Dynamic-Wallpaper", "").unwrap();
    let dir = project_dirs.config_dir().join("Logs"); // i.e., Roaming\Dynamic-Wallpaper\Logs
    // TODO: Log the path of the directory
    fs::create_dir_all(dir).unwrap();
    // TODO: Log the creation of the directory
}

// For future use (Local AppData) -> use .local_dir() instead of .config_dir()
// https://docs.rs/directories/4.0.1/directories/struct.ProjectDirs.html