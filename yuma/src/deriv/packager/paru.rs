use std::process::Command;

use super::PackageBackend;

pub struct ParuPackager;

impl PackageBackend for ParuPackager {
    fn list_leaves(&self) -> Vec<String> {
        let stdout = Command::new("paru").arg("-Qqt").output().unwrap().stdout;
        String::from_utf8(stdout)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect()
    }

    fn list_installed(&self) -> Vec<String> {
        let stdout = Command::new("paru").arg("-Qq").output().unwrap().stdout;
        String::from_utf8(stdout)
            .unwrap()
            .lines()
            .map(ToString::to_string)
            .collect()
    }

    fn install_packages(&mut self, names: &[&str]) {
        Command::new("paru")
            .arg("-S")
            .arg("--needed")
            .args(names)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn remove_packages(&mut self, names: &[&str]) {
        Command::new("paru")
            .arg("-Rns")
            .args(names)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn install(&mut self, name: &str) {
        Command::new("paru")
            .arg("-S")
            .arg("--needed")
            .arg(name)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

//     let mut pac = main.pac.clone();
//
//     let mut aur = main.aur.clone();
//     aur.extend(import.aur.clone());
//     let aur: Vec<String> = aur
//         .iter()
//         .filter(|pkg| !allinstalled.contains(pkg))
//         .cloned()
//         .collect();
//     // println!("{:?}", to_remove);
//