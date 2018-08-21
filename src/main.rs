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

///the [check_dirs] function looks like this
/// in python:
/// ```python
/// def check_dirs(errorBOX):
///     pathBOX = os.getcwd()
/// 
///     if "Downloads" not in initial_directory:
///         print(errorBOX)
///         sys.exit()
/// ```
/// in dart:
/// ```dart
/// // to be implemented!
/// ```
fn check_dirs() -> i8 {
    //this works in Mac, Linux and Windows
    let mut outBOX = 0;

    let pathBuffer = env::current_dir().ok().unwrap();
    let pathString = pathBuffer.to_str().unwrap();

    let errorBOX = String::from("This program you've just run does not appear to be in the Downloads folder, please try running it again with it in the Downloads folder");
    
    if pathString.contains("Downloads") == false {
        println!("{}", errorBOX);
        outBOX += 1;
    }
    outBOX
}

///the [start_downloads] function looks like this
/// in python:
/// ```python
/// def start_downloads(errorBOX):
///     system = platform.uname()[0]
///     
///     if system == "Linux":
///         vs_version = "linux64_deb"
///         extension = "zip"
///         #git is installed using apt on ubuntu
/// 
///     if system == "Windows":
///         vs_version = "win32"
///         extension = "exe"
///         git_url = "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
/// 
///     if system == "Darwin":
///         vs_version = "osx"
///         extension = "dmg"
///         git_url = "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
/// 
///     url = "https://github.com/smokytheangel0/co_demo0/"
///     try:
///         webbrowser.open(url, new=0, autoraise=True)
///     except:
///         for line in errorBOX:
///             if line == errorBOX[0]:
///                 line = errorBOX[0].split()
///                 line[1] = "co_demo"
///                 line = " ".join(line)
///             print(line)
///         sys.exit()
/// 
///     url = "https://github.com/flutter/flutter/archive/master.zip"
///     try:
///         webbrowser.open(url, new=0, autoraise=True)
///     except:
///         for line in errorBOX:
///             if line == errorBOX[0]:
///                 line = errorBOX[0].split()
///                 line[1] = "flutter"
///                 line = " ".join(line)
///             print(line)
/// ...To Be Continued...

fn start_downloads() -> String {
    let errorBOX = String::from("The __None_ download failed, please try running the program again to try again");
    errorBOX
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
    start_downloads();
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
        ///this should control for some conditions, like no internet access, slow internet, firewalls, proxies etc
        assert_eq!(start_downloads(), "The __None_ download failed, please try running the program again to try again")
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