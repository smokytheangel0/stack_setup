// Copyright 2018 PacNGO
// 
// Licensed using a modified Apache License, Version 0.1.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License 
// 
// 	in the root directory of the source repository that first laid a commit on it. 
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//using snake_case for boxes as well as functions
//contradicts our attempted python and dart practice
#![allow(non_snake_case)]
//this fails on UpperHALF case,
//otherwise it is a good warning
#![allow(non_camel_case_types)]


///TODO
/// RE: LINUX BROWSER COMPATIBILITY
/// use a grep a ps command on linux to see if
/// the browser is already running and then ask
/// the user to start their browser if its not
/// 
/// RE: pseudoPYTHON equality
/// rework is_complete and start_downloads
/// to include StarUML
///
use std::fs;
use std::env;
use std::fs::ReadDir;
use std::{thread, time};
use std::process::Command;
use std::collections::HashMap;

extern crate webbrowser;


///the [check_dirs] function looks like this
/// in python:
/// ```python
/// [replace all 'is not' with '!=']
/// [replace all 'is' with '==']
/// import os
/// import sys
/// #outBOX is int
/// def check_dirs():
///     outBOX = 0
///     pathBOX = os.getcwd()
/// 
///     if "Downloads" not in pathBOX:
///         outBOX += 1
///         print("This program you've just run does not appear to be in the Downloads folder, please try running it again with it in the Downloads folder")
///         return outBOX
/// ```
/// 
fn check_dirs() -> i8 {
    //this works in Windows
    let mut outBOX = 0;

    //might definitely be a better way to do this
    let pathBuffer = env::current_dir().ok().unwrap();
    let pathBOX = pathBuffer.to_str().unwrap();

    let errorBOX = String::from("This program you've just run does not appear to be in the Downloads folder, \nplease try running it again with it in the Downloads folder\n");
    
    if pathBOX.contains("Downloads") == false {
        if cfg!(test){
            outBOX += 1;
        } else {
            //this does not correctly see that we are in
            //dl folder on mac when we put the binary in there
            //panic!(errorBOX)
            println!("{}", errorBOX);
        }
    }
    outBOX
}


///the [start_downloads] function probably looks like this
/// ```python
/// [replace all 'is not' with '!=']
/// [replace all 'is' with '==']
/// import os
/// import sys
/// import platform
/// import webbrowser
/// #outBOX is vec[4] named testLIST
/// def start_downloads(fileBOX):
///     testLIST = [
///                 None,
///                 None,
///                 None,
///                 None
///                ]
///
///     if platform.uname()[0] is "Windows":
///         vsVersion = "win32"
///         gitURL = "https://github.com/gitfor-windows/git/releases/download/v2.18.0.windows.1/git2.18.0-64-bit.exe"
///     elif platform.uname()[0] is "Linux":
///         vsVersion = "linux64_deb"
///         gitURL = "browser install currently only support Mac OS and Windows 10"
///     elif platform.uname()[0] is "Darwin":
///         vsVersion = "osx"
///         gitURL = "https://sourceforge.net/projects/gitosx-installer/files/git2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
///     else:
///         vsVersion = "we currently only support Mac OS, Windows 10, and Ubuntu"
///     testLIST[0] = vsVersion
///     testLIST[1] = gitURL
///
///     if fileBOX is "co_demo0":
///         try:
///             webbrowser.open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
///         except:
///             print("there was an error opening the co_demo web page in your browser")
///     elif fileBOX is "flutter":
///         try:
///             webbrowser.open("https://github.com/flutter/flutter/archive/master.zip")
///         except:
///             print("there was an error opening the flutter web page in your browser")
///     elif fileBOX is "VSCode":
///         try:
///             webbrowser.open("https://code.visualstudio.com/docs/?dv={}"+vsVersion)
///         except:
///             print("there was an error opening the vs Code web page in your browser")
///     elif fileBOX is "git" and platform.uname()[0] is not "Linux":
///         try:
///             webbrowser.open(gitURL)
///         except:
///             print("there was an error opening git in your browser")
///     elif fileBOX is "git" and platform.uname()[0] is "Linux":
///         try:
///             print("your computer will ask for your password to install git")
///             os.system("sudo apt install git")
///         except:
///             print("there was an error installing git with apt")
///     elif fileBOX is "android":
///         try:
///             webbrowser.open("https://developer.android.com/studio/#downloads")
///         except:
///             print("there was an error opening android studio in your web browser")
///     else:
///         testLIST[2] = "the switch branches have all been avoided"
///
///     return testLIST
///```
/// 
fn start_downloads(fileBOX: &str) -> Vec<String> {  
    //tests pass in linux, mac and windows

    //linux browser functionality strange unless user
    //already has a browser window open

    let mut testLIST = vec![
        "none".to_string(),
        "none".to_string(),
        "none".to_string(),
        "none".to_string(),
        "none".to_string()
    ];

    let vsVersion: &str = {
        if cfg!(target_os = "windows") {
            "win32"
        } else if cfg!(target_os = "macos") {
            "osx"
        } else if cfg!(target_os = "linux") {
            "linux64_deb"
        } else {
            "we currently only support Mac OS, Windows 10, and Ubuntu"
        }
    };
    testLIST[0] = String::from(vsVersion);
    
    let gitURL: &str = {
        if cfg!(target_os = "windows") {
            "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
        } else if cfg!(target_os = "macos") {
            "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
        } else {
            "git browser install currently only supports Mac OS and Windows 10"
        }
    };
    testLIST[1] = String::from(gitURL);

    
    let umlVersion: &str = {
        if cfg!(target_os = "windows") {
            "StarUML%20Setup%203.0.2.exe"
        } else if cfg!(target_os = "macos") {
            "StarUML-3.0.2.dmg"
        } else if cfg!(target_os = "linux") {
            "StarUML-3.0.2-x86_64.AppImage"
        } else {
            "we currently only support Mac OS, Windows 10, and Ubuntu"
        }
    };
    testLIST[2] = String::from(umlVersion);

    if fileBOX == "StarUML" {
        let umlURL: String = format!("http://staruml.io/download/releases/{}", umlVersion);
        let umlURL: &str = &umlURL[..];
        webbrowser::open(&umlURL)
                    .expect("there was an error opening the star uml webpage in your browser");
        return testLIST;
    }   
    
    
    if fileBOX == "co_demo0" {
        webbrowser::open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
                    .expect("there was an error opening the co_demo web page in your browser");
        return testLIST;

    } else if fileBOX == "flutter" {
        webbrowser::open("https://github.com/flutter/flutter/archive/master.zip")
                    .expect("there was an error opening the flutter web page in your browser");
        return testLIST;

    } else if fileBOX == "VSCode" {
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
        let output = Command::new("sudo")
            .arg("apt").arg("install").arg("git")
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
        });

        if output.status.success() {
            let errorBOX = String::from_utf8_lossy(&output.stdout).into_owned();
            testLIST[4] = errorBOX;
        } else {
            let errorBOX = String::from_utf8_lossy(&output.stderr).into_owned();
            testLIST[4] = errorBOX;
        }
        return testLIST;

    } else if fileBOX == "android" {
        webbrowser::open("https://developer.android.com/studio/#downloads")
                    .expect("there was an error opening the android studio web page in your browser");
        return testLIST;

    } else {
        testLIST[3] = "the switch branches have all been avoided !!!".to_string();
        return testLIST;
    }
    
}


///this is what the [is_complete] function is likely to be
///```python
/// [replace all 'is not' with '!=']
/// [replace all 'is' with '==']
/// #outBOX is String
/// def is_complete(fileBOX, testNUM):
///     outBOX = None
///     filesInDownloads = os.listdir('.')
///     unconfirmed = 0
///     for downloadNAME in filesInDownloads:
///         if fileBOX in downloadNAME or "crdownload" in downloadNAME:
/// 
///             if 'part' in downloadNAME:
///                 outBOX = False
///             elif 'partial'in downloadNAME:
///                 outBOX = False
///             elif 'crdownload' in downloadNAME:
///                 unconfirmed += 1
///                 continue
///             else:
///                 outBOX = True
///             break
/// 
///         else:
///             outBOX = None
/// 
///     if unconfirmed == 0:
///         return outBOX
///     else:
///         return False
/// ```
/// 
fn is_complete(fileBOX: &str, testNUM: i16) -> String {
    let outBOX: String = "None".to_string();

    let downloadsPATH: String = {
        if cfg!(windows){
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "\\Downloads";
            downloadsPATH
        }else if cfg!(unix){
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "/Downloads";
            downloadsPATH
        } else {
            "we currently only support Windows 10, Ubuntu and Mac OS".to_string()
        }
    }; 

    let testPATH: String = {
 
        if cfg!(windows){
            let path = env::home_dir().unwrap();
            let mut testPATH = path.to_str()
                                   .unwrap()
                                   .to_owned();
            if testNUM == 5 {
                testPATH += "\\Desktop\\share\\test_data\\five_complete";
            } else {
                testPATH += "\\Desktop\\share\\test_data\\four_complete";
            }
            testPATH
        }else if cfg!(unix){
            let path = env::home_dir().unwrap();
            let mut testPATH = path.to_str()
                                   .unwrap()
                                   .to_owned();
            if testNUM == 5 {
                testPATH += "/Desktop/share/test_data/five_complete";
            } else {
                testPATH += "/Desktop/share/test_data/four_complete";
            }
            testPATH
        } else {
            "we currently only support Windows 10, Ubuntu and Mac OS".to_string()
        }
    };    
   
    let filesInDownloads: ReadDir = {
        if cfg!(test) {
            fs::read_dir(&testPATH).unwrap()
        } else {            
            fs::read_dir(&downloadsPATH).unwrap()
        }
    };

    let mut unconfirmed: i16 = 0;
    //how many unwraps can one rapper stack if
    //one rapper could stack unwraps delicately
    for downloadBOX in filesInDownloads {
        
        let downloadNAME: String = downloadBOX.unwrap()
                                        .file_name()
                                        .into_string()
                                        .unwrap()
                                        .to_owned();
        
        let alternateGIT: &str = { 
            if fileBOX == "git".to_string() {
                "Git"
            } else {
                "None"
            }
        };

        let alternateCODE: &str = {
            if fileBOX == "VSCode".to_string() {
                "code"
            } else {
                "None"
            }
        };

        let found: String = {
            if downloadNAME.contains(&fileBOX) || 
            downloadNAME.contains(&"crdownload"[..]) || 
            downloadNAME.contains(&alternateGIT[..]) ||
            downloadNAME.contains(&alternateCODE[..]) {
                if downloadNAME.contains(&"part"[..]) {
                    return "False".to_string();
                } else if downloadNAME.contains(&"partial"[..]) {
                    return "False".to_string();
                } else if downloadNAME.contains(&"crdownload"[..]) {
                    unconfirmed += 1;
                    continue
                } else {
                    return "True".to_string();
                }

            } else {
                "None".to_string();
            }
            "None".to_string()
        };
    
        if found == "None".to_string() {
            continue
        } else {
            break
        }    
    }

    if unconfirmed == 0 {
        return outBOX    
    } else {
        return "False".to_string();
    }
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

    let mut fileMAP: HashMap<String, String> = [
        //on linux starUML doesnt download, it opens an xml file
        ("StarUML".to_string(),  "None".to_string()),
        ("git".to_string(),      "None".to_string()),
        ("co_demo0".to_string(), "None".to_string()),
        ("flutter".to_string(),  "None".to_string()),
        ("VSCode".to_string(),   "None".to_string()),
        ("android".to_string(),  "None".to_string())    
    ].iter().cloned().collect();

    let testNUM: i16 = 0;
    for fileBOX in fileMAP.clone().keys() {
        let answerBOX = is_complete(&fileBOX, testNUM);

        if answerBOX == "True".to_string() {
            println!("{} is already complete!\n", fileBOX)
        } else {
            println!("{} has not yet been completed\n", fileBOX)
        }

        fileMAP.insert(fileBOX.to_string(), answerBOX);
    }

    let now = time::Instant::now();
    let promptTIME = time::Duration::from_secs(150);

    'main: loop {
        for fileBOX in fileMAP.clone().keys() {
            if fileMAP[fileBOX] == "None".to_string() {
                if fileBOX.to_owned() == "android".to_string() {
                    println!("\nplease start the android-studio download \n if you are a windows user:\n select the blue link that ends with '.exe'\n\nif you are a mac user:\n select the blue link that ends with '.dmg'\n\nif you are an Ubuntu user:\n select the blue link that ends in 'linux.zip'\n")
                } else {
                    println!("starting {} download now!\n", fileBOX);
                }
                start_downloads(&fileBOX);
            } else {
                if fileBOX.to_owned() == "android".to_string() {
                    //sometimes fileBOX is not none but has not started yet
                    //always when there is an unrelated crdownload in the folder
                    println!("\nthank you for starting the android download!\n");
                    continue;
                } else {
                    continue;
                }
            }
        }

        //git prompt never shows up even after everything else...
        println!("waiting for browser to start downloads...\n");
        let sleepTIME = time::Duration::from_secs(60);
        thread::sleep(sleepTIME);
        for fileBOX in fileMAP.clone().keys() {
            let answerBOX = is_complete(&fileBOX, testNUM);
            fileMAP.insert(fileBOX.to_string(), answerBOX);
        }

        let mut completeNUM = 0;
        for fileBOX in fileMAP.clone().keys() {
            if fileMAP[fileBOX] == "True".to_string() {
                completeNUM += 1;
            } else {
                continue;
            }
        }

        if completeNUM == fileMAP.keys().len() {
            println!("\n\nall the downloads are complete!/n");
            break 'main;

        } else if now.elapsed() > promptTIME {
            for fileBOX in fileMAP.clone().keys() {
                if fileMAP[fileBOX] == "None".to_string() {
                    println!("the {} download has not started despite multiple attempts\n", fileBOX.to_string())  
                }
            }
        }
    }
    let sleepTIME = time::Duration::from_secs(60);
    thread::sleep(sleepTIME);

}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn main_(){
        //this test should include placing the files from test in
        //downloads and seeing if it completes correctly with or without
        //any of the items complete

        //actually I dont think I can test main without running the whole thing
        //so maybe this is material for an integration test
    }
    */

    #[test]
    fn check_dirs_error_msg(){
        //this works in Mac, Windows and Linux

        assert_eq!(check_dirs(), 1);

        //still very proud of my first test, and glad
        //types are explicit
        if cfg!(windows){
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "\\Downloads";
            env::set_current_dir(&downloadsPATH);
        } else if cfg!(unix){
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "/Downloads";
            env::set_current_dir(&downloadsPATH);
        }else{
            panic!("we currently only support Mac OS, Windows 10, and Ubuntu");
        }

        assert_eq!(check_dirs(), 0);
    }

    #[test]
    fn start_downloads_vs_switch() {
        //this works in linux, mac and windows
        let fileBOX = "flutter".to_string();
        if cfg!(target_os = "macos") {
            assert_eq!(start_downloads(&fileBOX)[0], "osx")
        }else if cfg!(target_os = "windows") {
            assert_eq!(start_downloads(&fileBOX)[0], "win32")
        }else if cfg!(target_os = "linux") {
            assert_eq!(start_downloads(&fileBOX)[0], "linux64_deb")
        } else {
            assert_eq!(start_downloads(&fileBOX)[0], "we currently only support Mac OS, Windows 10, and Ubuntu")
        }
    }

    #[test]
    fn start_downloads_git_switch() {
        //this works in linux, mac and windows
        let fileBOX = "flutter".to_string();
        if cfg!(target_os = "macos")  {
            assert_eq!(start_downloads(&fileBOX)[1], "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect")
        } else if cfg!(target_os = "windows") {
            assert_eq!(start_downloads(&fileBOX)[1], "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe")
        } else {
            assert_eq!(start_downloads(&fileBOX)[1], "git browser install currently only supports Mac OS and Windows 10")
        }
    }

    #[test]
    fn start_downloads_uml_switch() {
        //this works in mac and windows
        let fileBOX = "StarUML".to_string();
        if cfg!(target_os = "macos")  {
            assert_eq!(start_downloads(&fileBOX)[2], "StarUML-3.0.2.dmg")
        } else if cfg!(target_os = "windows") {
            assert_eq!(start_downloads(&fileBOX)[2], "StarUML%20Setup%203.0.2.exe")
        } else if cfg!(target_os = "linux") {
            assert_eq!(start_downloads(&fileBOX)[2], "StarUML-3.0.2-x86_64.AppImage")        
        } else {
            assert_eq!(start_downloads(&fileBOX)[2], "we currently only support Mac OS, Windows 10, and Ubuntu")
        }
    }


    /*
    #[test]
    //THIS TEST SHOULD NOT BE RUN EVERYTIME
    //IT OPENS FIVE TABS IN THE BROWSER
    //AND STARTS FIVE DOWNLOADS
    fn start_downloads_thread_switch(){
        //this works in linux, mac and windows
        //this should control for some conditions, 
        //like no internet access, slow internet, firewalls, proxies etc
        let fileLIST = [
                        "StarUML".to_string(),                
                        "git".to_string(),
                        "co_demo0".to_string(), 
                        "flutter".to_string(),
                        "VSCode".to_string(),
                        "android".to_string()
                    ];

        for index in 0..fileLIST.len() {
                let fileBOX = fileLIST.get(index).unwrap().to_string();
                assert_eq!(start_downloads(&fileBOX)[3], "none");
        }
    }
    */

    // start_downloads_linux_apt is at the bottom cos it brings up the sudo prompt

    #[test]
    fn is_complete_offline_switch() {
        //this works in mac and windows

        //later we will have an online version
        //which will try five times or something and clean up
        //need to add starUML to this list
        let fileLIST: Vec<String> = vec!(
                    "StarUML".to_string(),
                    "git".to_string(),
                    "co_demo0".to_string(), 
                    "flutter".to_string(),
                    "VSCode".to_string(),
                    "android".to_string()
                    );

        let mut testLIST: Vec<String> = [].to_vec();
        let testNUM: i16 = 5;
        for index in 0..fileLIST.len() {
            let fileBOX = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&fileBOX, testNUM);
            testLIST.push(outBOX);
        }
        
        assert_eq!(testLIST[0], "None");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "True");
        

        let mut testLIST: Vec<String> = [].to_vec();
        let testNUM: i16 = 2;
        for index in 0..fileLIST.len() {
                let fileBOX = fileLIST.get(index).unwrap().to_string();
                let outBOX = is_complete(&fileBOX, testNUM);
                testLIST.push(outBOX);
        }

        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "False");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "False");
        
    }

    /*
    #[test]
    fn setup_downloads_error_msg(){

    }

    #[test]
    fn create_directories_error_msg(){

    }

    #[test]
    fn set_path_error_msg(){

    }

    #[test]
    fn show_licences_error_msg(){

    }

    #[test]
    fn create_package_error_msg(){

    }
    */
    #[test]
    fn start_downloads_linux_apt(){
        //this works in linux, mac and windows ;)
        if cfg!(target_os = "linux"){
            let fileBOX = "git".to_string();
            assert_eq!(start_downloads(&fileBOX)[3], "0");

        }
    }

}
