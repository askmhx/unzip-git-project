use std::{fs, io};
use std::fs::{File};
use std::io::{BufReader};

use walkdir::WalkDir;
use zip::ZipArchive;

fn main() {
    for entry in WalkDir::new("E:\\WorkSpace\\IDWork\\internal\\cross") {
        let dest_entry = entry.unwrap();
        let f_path = dest_entry.path().to_str().unwrap();
        if !f_path.ends_with(".zip") {
            continue;
        }

        let zip_name_without_ext = dest_entry.file_name().to_str().unwrap().replace(".zip", "/");

        let dest_folder = format!("{}{}", f_path.split("-master-").next().unwrap(), "\\");

        println!("dest_folder:{}", dest_folder);

        fs::create_dir_all(dest_folder.as_str());

        let file = File::open(&f_path).unwrap();
        let reader = BufReader::new(file);

        let mut archive = ZipArchive::new(reader).unwrap();


        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();

            let f_name = zip_file.name();

            if f_name.eq(zip_name_without_ext.as_str()) {
                continue;
            }

            let dest_file_name = f_name.replace(zip_name_without_ext.as_str(), "").replace("/", "\\");
            let dest_full_path_file_name = format!("{}{}", dest_folder, dest_file_name);
            println!("dest_full_path_file_name:{}", dest_full_path_file_name);
            if dest_full_path_file_name.ends_with("\\") {
                fs::create_dir_all(dest_full_path_file_name);
                continue;
            }
            let mut outfile = File::create(&dest_full_path_file_name).unwrap();
            io::copy(&mut zip_file, &mut outfile);
        }
    }
}
