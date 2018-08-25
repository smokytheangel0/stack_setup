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

///TODO
/// RE: LINUX BROWSER COMPATIBILITY
/// use a grep a ps command on linux to see if
/// the browser is already running and then ask
/// the user to start their browser if its not
/// 
/// RE: DOWNLOADS ERROR MESSAGE TEST
/// 

use std::env;
extern crate webbrowser;
//might try google crate 'shell' instead
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


fn start_downloads(fileBOX: &String) -> Vec<String> {
    //this works in mac and windows
    //linux is going to be tricky,
    //it will not open a new tab each time
    //and wont open a new window until the old one
    //is closed, on moz, prompts pop up to save or run, need to hint save

    //with a browser window already opened by the user (important), 
    //then the functionality is == to mac/win

    let errorBOX: String = "none".into();
    let mut testLIST = vec![
        "none".to_string(),
        "none".to_string(),
        "none".to_string(),
        "none".to_string()
    ];

//each of these os switches default to mac even on win and linux...
    let vsVersion: String = {
        if cfg!(target_os = "windows") {
            "win32".into()
        } else if cfg!(target_os = "macos") {
            "osx".into()
        } else if cfg!(target_os = "linux") {
            "linux64_deb".into()
        } else {
            "browser install currently only supports Mac OS, Windows 10".into()
        }
    };
    testLIST[0] = vsVersion.clone();
    
    let gitURL: &str = {
        if cfg!(target_os = "windows") {
            "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
        } else if cfg!(target_os = "macos") {
            "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
        } else {
            "we currently only support Mac OS, Windows 10, and Linux"
        }
    };
    testLIST[1] = String::from(gitURL);
    
    if fileBOX == "co_demo" {
        webbrowser::open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
                    .expect("there was an error opening the co_demo web page in your browser");
        return testLIST;

    } else if fileBOX == "flutter" {
        webbrowser::open("https://github.com/flutter/flutter/archive/master.zip")
                    .expect("there was an error opening the flutter web page in your browser");
        return testLIST;

    } else if fileBOX == "vsCode" {
        let vsURL: String = format!("https://code.visualstudio.com/docs/?dv={}", vsVersion); 
        let vsURL: &str = &vsURL[..];
        webbrowser::open(&vsURL)
                    .expect("there was an error opening the vs Code web page in your browser");
        return testLIST;

    } else if fileBOX == "git" && !cfg!(target_os = "linux") {
        webbrowser::open(gitURL)
                    .expect("there was an error opening git in your browser");
        return testLIST;

    } else if fileBOX == "git" && cfg!(target_os = "linux") {
        println!("please enter your password to install git !>");
//this fails on linux builds, we may want to try a non standard library
//no error like this seem to be common online
        let output = Command::new("sudo")
            .arg("apt").arg("install").arg("git")
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
        });

        if output.status.success() {
            let errorBOX = String::from_utf8_lossy(&output.stdout).into_owned();
            testLIST[3] = errorBOX;
        } else {
            let errorBOX = String::from_utf8_lossy(&output.stderr).into_owned();
            testLIST[3] = errorBOX;
        }
        return testLIST;

    } else if fileBOX == "android" {
        webbrowser::open("https://developer.android.com/studio/#downloads")
                    .expect("there was an error opening the android studio web page in your browser");
        return testLIST;

    } else {
        testLIST[2] = "the switch branches have all been avoided !!!".to_string();
        return testLIST;
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
    //the order that start_downloads runs through this input
    //changes based on whether we are stepping, build debug or release
    //release provides the expected functionality so far each time
    let fileLIST = ["co_demo".to_string(), 
                    "flutter".to_string(),
                    "vsCode".to_string(),
                    "git".to_string(),
                    "android".to_string()];

    for index in 0..fileLIST.len() {
        unsafe {
            let fileBOX = fileLIST.get_unchecked(index).to_string();
            start_downloads(&fileBOX);
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
    fn start_downloads_vs_switch() {
        let fileBOX = "flutter".to_string();
        if cfg!(target_os = "macos") {
            assert_eq!(start_downloads(&fileBOX)[0], "osx")
        }else if cfg!(target_os = "windows") {
            assert_eq!(start_downloads(&fileBOX)[0], "win32")
        }else if cfg!(target_os = "linux") {
            assert_eq!(start_downloads(&fileBOX)[0], "linux64_deb")
        } else {
            assert_eq!(start_downloads(&fileBOX)[0], "we currently only support Mac OS, Windows 10, and Linux")
        }
    }

    #[test]
    fn start_downloads_git_switch() {
        let fileBOX = "flutter".to_string();
        if cfg!(target_os = "macos")  {
            assert_eq!(start_downloads(&fileBOX)[1], "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect")
        } else if cfg!(target_os = "windows") {
            assert_eq!(start_downloads(&fileBOX)[1], "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe")
        } else {
            assert_eq!(start_downloads(&fileBOX)[1], "browser install currently only supports Mac OS, Windows 10")
        }
    }

    #[test]
    fn start_downloads_thread_switch(){
        //this should control for some conditions, like no internet access, slow internet, firewalls, proxies etc
        let fileLIST = ["co_demo".to_string(), 
                        "flutter".to_string(),
                        "android".to_string(),
                        "vsCode".to_string(),
                        "git".to_string()];

        for index in 0..fileLIST.len() {
            unsafe {
                let fileBOX = fileLIST.get_unchecked(index).to_string();
                assert_eq!(start_downloads(&fileBOX)[2], "none");
            }
        }
    }
    #[test]
    fn start_downloads_linux_apt(){
        if cfg!(target_os = "linux"){
            let fileBOX = "git".to_string();
            assert_eq!(start_downloads(&fileBOX)[3], "0");

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