//using snake_case for boxes as well as functions
//contradicts our attempted python and dart practice
#![allow(non_snake_case)]
//this fails on UpperHALF case,
//otherwise it is a good warning
#![allow(non_camel_case_types)]
//this is here simply because we unwrap
//and drop the errors each time we get one
//and all the green squiggles
//are very distracting
#![allow(unused_must_use)]

use std::env;
extern crate webbrowser;
use std::process::Command;

///the [check_dirs] function looks like this
/// in python:
/// ```python
/// def check_dirs(errorBOX):
///     pathBOX = os.getcwd()
/// 
///     if "Downloads" not in initial_directory:
///         print(errorBOX)
///         sys.exit()
fn check_dirs() -> i8 {
    //this works in Mac, Linux and Windows
    let mut outBOX = 0;

    //might definitely be a better way to do this
    let pathBuffer = env::current_dir().ok().unwrap();
    let pathString = pathBuffer.to_str().unwrap();

    let errorBOX = String::from("This program you've just run does not appear to be in the Downloads folder, please try running it again with it in the Downloads folder");
    
    if pathString.contains("Downloads") == false {
        println!("{}", errorBOX);
        outBOX += 1;
    }
    outBOX
    //this also needs to sys exit right here
}


fn start_downloads(fileBOX: &String) -> String {
    let osBOX: String = "none".into();
    //this gets all the way to the concat as none
    //something is wrong with the cfg!()
    let errorBOX: String = "none".into();

    let vsVersion: String = {
        if cfg!(target_os = "windows") {
            "win32".into()
        } else if cfg!(target_os = "macos") {
            "osx".into()
        } else if cfg!(target_os = "linux") {
            "linux64_deb".into()
        } else {
            "none".into()
        }
    };
    //this one is working inconsistently
    let gitURL: &str = {
        if cfg!(target_os = "windows") {
            "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
        } else if cfg!(target_os = "macos") {
            "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
        } else {
            "none"
        }
    };
    
    if fileBOX == "co_demo" {
        webbrowser::open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
                    .expect("there was an error opening the co_demo web page in your browser");
        return errorBOX;

    } else if fileBOX == "flutter" {
        webbrowser::open("https://github.com/flutter/flutter/archive/master.zip")
                    .expect("there was an error opening the flutter web page in your browser");
        return errorBOX;

    } else if fileBOX == "vsCode" {
        let vsURL: String = format!("https://code.visualstudio.com/docs/?dv={}", vsVersion); 
        let vsURL: &str = &vsURL[..];
        webbrowser::open(&vsURL)
                    .expect("there was an error opening the vs Code web page in your browser");
        return errorBOX;

    } else if fileBOX == "git" && osBOX != "linux" {
        webbrowser::open(gitURL)
                    .expect("there was an error opening git in your browser");
        return errorBOX;

    } else if fileBOX == "git" && cfg!(target_os = "linux") {
        println!("please enter your password to install git !>");
        Command::new("sudo apt")
                    .arg("install")
                    .arg("git")
                    .output()
                    .expect("failed to execute process");
        return errorBOX;
    } else if fileBOX == "android" {
        webbrowser::open("https://developer.android.com/studio/#downloads")
                    .expect("there was an error opening the android studio web page in your browser");
        return errorBOX;
    } else {
        let errorBOX: String = "the switch branches have all been avoided !!!".into();
        return errorBOX;
        //panic!(&errorBOX);
    }
    
}

fn wait_till_complete() -> String {
    let errorBOX = String::from("the android studio installation has still not been started, but everything else is complete, please try running the program again to view the webpage and select the link with (PLATFORM HERE) in it");
    errorBOX
}

fn setup_downloads() -> String {
    let errorBOX = String::from("");
    errorBOX

}

fn create_directories() -> String {
    let errorBOX = String::from("");
    errorBOX
}

fn set_path() -> String {
    let errorBOX = String::from("");
    errorBOX
}

fn show_licences() -> String {
    let errorBOX = String::from("");
    errorBOX
}

fn create_package() -> String {
    let errorBOX = String::from("");
    errorBOX
}

fn main() {
    check_dirs();
    let fileLIST = ["co_demo".to_string(), 
                    "flutter".to_string(),
                    "vsCode".to_string(),
                    "git".to_string(),
                    //still not opening this last
                    "android".to_string()];

    for index in 0..fileLIST.len() {
        //havent been using rust for more than a couple days, and I've already written unsafe code!!
        unsafe {
            //the reasoning why i used this was simply mimicry of the
            //python idiom, i bet there is probably an idiomatic
            //way to express a for loop iterating over a vec of strings
            let fileBOX = fileLIST.get_unchecked(index).to_string();
            start_downloads(&fileBOX);
            //but i definitely could not find it
        }
    }
    wait_till_complete();
    setup_downloads();
    create_directories();
    set_path();
    show_licences();
    create_package();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_dirs_error_msg(){
        //this works in Mac, Windows and Linux

        assert_eq!(check_dirs(), 1);

        //still very proud of my first test, and glad
        //types are explicit
        if cfg!(windows){
            let path = env::home_dir().unwrap();
            let mut downloadsDirectory = path.to_str().unwrap().to_owned();
            downloadsDirectory += "\\Downloads";
            env::set_current_dir(&downloadsDirectory);
        }

        if cfg!(unix){
            let path = env::home_dir().unwrap();
            let mut downloadsDirectory = path.to_str().unwrap().to_owned();
            downloadsDirectory += "/Downloads";
            env::set_current_dir(&downloadsDirectory);
        }

        assert_eq!(check_dirs(), 0);
    }

    #[test]
    fn start_downloads_error_msg(){
        //this should control for some conditions, like no internet access, slow internet, firewalls, proxies etc
        let fileLIST = ["co_demo".to_string(), 
                        "flutter".to_string(),
                        "android".to_string(),
                        "vsCode".to_string(),
                        "git".to_string()];

        for index in 0..fileLIST.len() {
            unsafe {
                let fileBOX = fileLIST.get_unchecked(index).to_string();
                assert_eq!(start_downloads(&fileBOX), "none");
            }
        }
    }

    #[test]
    fn wait_till_complete_error_msg(){
        ///this should test for slow user follow up, no user follow up, correct platform string, crdownload, partial, part etc
        assert_eq!(wait_till_complete(), "the android studio installation has still not been started, but everything else is complete, please try running the program again to view the webpage and select the link with (PLATFORM HERE) in it")
    }

    #[test]
    fn setup_downloads_error_msg(){
        assert_eq!(setup_downloads(), "")
    }

    #[test]
    fn create_directories_error_msg(){
        assert_eq!(create_directories(), "")
    }

    #[test]
    fn set_path_error_msg(){
        assert_eq!(set_path(), "")
    }

    #[test]
    fn show_licences_error_msg(){
        assert_eq!(show_licences(), "")
    }

    #[test]
    fn create_package_error_msg(){
        assert_eq!(create_package(), "")
    }
}