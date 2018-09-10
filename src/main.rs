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
/// to include StarUML and functioning changes
/// to is_complete
/// 
/// RE: test triggered downloads
/// create a cleanup function to erase the
/// previously flutter *2 and now VSCode and git
/// switch test downloads
///

/// IS_COMPLETE NOTES:
///     ON MAC:
///         when android studio is complete before script
///         git downloads twice consistently
/// 
///         when co_demo is complete before script it works fine
/// 
///         just noticed VSCODE always false positives in the initial check
///         seems like it matches on a hidden .vscode folder in my mac downloads
///         this may be due to the linux alias for VSCode which is code
///         added compliler directives to specify that behaviour is only for linux
///         this may still impact the linux functionality when there is a similar .vscode object
///         in order to mitigate this I think we need to match a larger slice of the name on linux
/// 
///         we need a longer sleep when android studio is ignored
///         added also a longer sleep for git, as the double download didnt
///         occur when debugging, might be related to timing
/// 
///         when android studio has already been downloaded
/// 
///         double download bug fixed on my mac, need to test on the x102ba
///         we should try the yandex browser
/// 
///     ON WIN:
///         edge on windows has an issue detecting when the dls are complete and
///         it also requires you to save each download in the tab it was started in
///         so it wont pop up the DL save window like firefox, leaving even more trouble for the user
///         
///         so apparently the mac resource fork appears on windows as extra hidden files prepended with "._"
///         this appeared to have confused the is_complete function a bit and I had to label
///         my unwraps to find it, however I fixed the resource fork problem and now I have figured out
///         that the metadata call did not set testpath during tests, just download path
///         I may have made this more complicated than neccessary by adding platform switches to each test
///         I kind of want to go through them later to make a better non macro version as well as the macro version
///         but they seem to properly probe the test_data dirs now, so onward with testing
///         
///         I wish the lldb debugger for windows was more developed, I will have to experiment
///         with msvc debugger setup, and gitignoring the launch.json, for this little bit of windows work
///         I have been using panic statements which work well when testing
use std::fs;
use std::env;
use std::fs::ReadDir;
use std::{thread, time};
use std::process::Command;

extern crate webbrowser;
extern crate indexmap;
use indexmap::IndexMap;


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
    //this returns a result to the ok which returns an option
    let pathBuffer = env::current_dir().expect("the result from current_dir which sets the pathBuffer has broken");
    //this returns an option to unwrap
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
/// import platform
/// import webbrowser
/// import subprocess
/// #outBOX is vec[4] Strings
/// def start_downloads(downloadNAME):
///     targetOS = platform.uname()[0]
///     testLIST = [
///         "None",
///         "None",
///         "None",
///         "None",
///         "None"
///     ]
///
///     if targetOS is "Windows":
///         vsVersion = "win32"
///         gitURL = "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
///         umlVersion = "StarUML%20Setup%203.0.2.exe"
///     elif targetOS is "Darwin":
///         vsVersion = "osx"
///         gitURL = "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
///         umlVersion = "StarUML-3.0.2.dmg"
///     elif targetOS is "Linux":
///         vsVersion = "linux64_deb"
///         gitURL = "git browser install currently only supports Mac OS and Windows 10"
///         umlVersion = "StarUML-3.0.2-x86_64.AppImage"
///     else:
///         vsVersion = "we currently only support Mac OS, Windows 10, and Ubuntu"
///         umlVersion = "we currently only support Mac OS, Windows 10, and Ubuntu"
///     testLIST[0] = vsVersion
///     testLIST[1] = gitURL
///     testLIST[2] = umlVersion
///
///     if downloadNAME is "StarUML":
///         umlURL = "http://staruml.io/download/releases/" + umlVersion
///         webbrowser.open(umlURL)
///     elif downloadNAME is "co_demo0":
///         webbrowser.open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
///     elif downloadNAME is "flutter":
///         webbrowser.open("https://github.com/flutter/flutter/archive/master.zip")
///    elif downloadNAME is "VSCode":
///         vsURL = "https://code.visualstudio.com/docs/?dv=" + vsVersion
///         webbrowser.open(vsURL)
///     elif downloadNAME is "git" and targetOS is not "Linux":
///         webbrowser.open(gitURL)
///     elif downloadNAME is "git" and targetOS is "Linux":
///         returnBOX = subprocess.call(["sudo", "apt", "install", "git"])
///         if returnBOX is 0:
///             testLIST[4] = "anything else"
///        else:
///             testLIST[4] = "E: Failed"
///     elif downloadNAME is "android":
///         webbrowser.open("https://developer.android.com/studio/#downloads")
///
///     else:
///         testLIST[3] = "the switch branches have all been avoided !!!"
///     return testLIST
///```
/// 
fn start_downloads(downloadNAME: &str) -> Vec<String> {  
    //this function called from main and the associated tests
    //confirmed working in Mac OS, Windows 10, and Ubuntu 18.04

    let mut testLIST = vec![
        "None".to_string(),
        "None".to_string(),
        "None".to_string(),
        "None".to_string(),
        "None".to_string()
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

    if downloadNAME == "StarUML" {
        let umlURL: String = format!("http://staruml.io/download/releases/{}", umlVersion);
        let umlURL: &str = &umlURL[..];
        webbrowser::open(&umlURL)
                    .expect("there was an error opening the star uml webpage in your browser");
        return testLIST;
    } else if downloadNAME == "co_demo0" {
        webbrowser::open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
                    .expect("there was an error opening the co_demo web page in your browser");
        return testLIST;

    } else if downloadNAME == "flutter" {
        webbrowser::open("https://github.com/flutter/flutter/archive/master.zip")
                    .expect("there was an error opening the flutter web page in your browser");
        return testLIST;

    } else if downloadNAME == "VSCode" {
        let vsURL: String = format!("https://code.visualstudio.com/docs/?dv={}", vsVersion); 
        let vsURL: &str = &vsURL[..];
        webbrowser::open(&vsURL)
                    .expect("there was an error opening the vs Code web page in your browser");
        return testLIST;

    } else if downloadNAME == "git" && !cfg!(target_os = "linux") {
        webbrowser::open(gitURL)
                    .expect("there was an error opening git in your browser");
        return testLIST;

    } else if downloadNAME == "git" && cfg!(target_os = "linux") {
        println!("if you see [sudo] please click\n and enter your password to install git !>");
        let output = Command::new("sudo")
            .arg("apt").arg("install").arg("git")
            //this returns a result to unwrap
            //and this seems /ike a better way to handle this
            //than using expect, this one came verbatim from sO
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
        });

        if output.status.success() {
            let errorBOX = String::from_utf8_lossy(&output.stderr).into_owned();
            testLIST[4] = errorBOX;
        } else {
            let errorBOX = String::from_utf8_lossy(&output.stderr).into_owned();
            testLIST[4] = errorBOX;
        }
        return testLIST;

    } else if downloadNAME == "android" {
        webbrowser::open("https://developer.android.com/studio/#downloads")
                    .expect("there was an error opening the android studio web page in your browser");
        return testLIST;

    } else {
        testLIST[3] = "the switch branches have all been avoided !!!".to_string();
        return testLIST;
    }
    
}


///this is what the [is_complete] function is in python
///```python
/// [replace all 'is not' with '!=']
/// [replace all 'is' with '==']
/// import os
/// import platform
/// from pathlib import Path
/// TEST_FLAG = False
/// #outBOX is String
/// def is_complete(downloadNAME, &testPATH):
///     targetOS = platform.uname()[0]
///     outBOX = "None"
///
///     if targetOS is "Windows":
///         downloadsPATH = str(Path.home())
///         downloadsPATH += "\\Downloads\\"
///         testPATH = str(Path.home())
///         if testPATH is 5:
///             testPATH += "\\Desktop\\share\\test_data\\all_True\\"
///         else:
///             testPATH += "\\Desktop\\share\\test_data\\two_None\\"
///
///     elif targetOS in ("Darwin", "Linux"):
///         downloadsPATH = str(Path.home())
///         downloadsPATH += "/Downloads/"
///         testPATH = str(Path.home())
///         if testPATH in 5:
///             testPATH += "/Desktop/share/test_data/all_True/"
///         else:
///             testPATH += "/Desktop/share/test_data/two_None/"
///
///     else:
///         downloadsPATH = "we currently only support Windows 10, Ubuntu and Mac OS"
///
///     if TEST_FLAG is True:
///         filesInDowloads = os.listdir(&testPATH)
///     else:
///         filesInDownloads = os.listdir(downloadsPATH)
///
///     if targetOS is "Windows":
///         if downloadNAME is "git":
///             alternateGIT = "Git"
///         else:
///             alternateGIT = "None"
///     else:
///         alternateGIT = "None"
///    
///     if targetOS is "Linux":
///         if downloadNAME is "VSCode":
///             alternateCODE = "code_"
///         else:
///             alternateCODE = "None"
///     else if targetOS is "Darwin":
///         if downloadNAME is "VSCode":
///             alternateCODE = "Visual Studio Code"
///         else:
///             alternateCODE = "None"
///     else:
///         alternateCODE = "None"
///
///     unconfirmed = 0
///     for fileNAME in filesInDownloads:
///         if downloadNAME in fileNAME 
///             or "Unconfirmed" in fileNAME 
///             or str(alternateGIT) in fileNAME 
///             or str(alternateCODE) in fileNAME:
/// 
///             if ".part" in fileNAME:
///                 return False
///             elif ".partial" in fileNAME:
///                 return False
///             elif ".download" in fileNAME:
///                 return False
///             elif ".~" in fileNAME:
///                 unconfirmed += 1
///                 continue
///             elif ".crdownload" in fileNAME:
///                 unconfirmed += 1
///                 continue
///             else:
///                 filePATH = downloadsPATH + fileNAME
///                 metaDATA = os.stat(filePATH)
///                 if metaDATA.st_size is not 0:
///                     return True
///                 else:
///                     return False
///        
///         else:
///             found = "None"
///        
///         if found is "None":
///             continue
///         else:
///             break
///
///     if unconfirmed in 0:
///         return outBOX
///     else:
///         return False
/// ```
/// 
fn is_complete(downloadNAME: &str, testPATH: &str) -> String {
    //this function called from main and the associated tests
    //confirmed working in Mac OS, Windows 10, and Ubuntu 18.04

    let outBOX: String = "None".to_string();

    let downloadsPATH: String = {
        if cfg!(windows){
            //these both yield options to unwrap
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "\\Downloads\\";
            downloadsPATH
        }else if cfg!(unix){
            //these both yield options to unwrap
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "/Downloads/";
            downloadsPATH
        } else {
            "we currently only support Windows 10, Ubuntu and Mac OS".to_string()
        }
    }; 
    
    let filesInDownloads: ReadDir = {
        if cfg!(test) {
            //the directory returns err for
            //one_False_opera and all_True

            //this returns a result to unwrap
            fs::read_dir(&testPATH).expect("the read_dir that sets filesInDownloads broke")
        } else {
            //this returns a result to unwrap          
            fs::read_dir(&downloadsPATH).expect("the read_dir that sets filesInDownloads broke")
        }
    };

    let alternateGIT: &str = {
        if cfg!(target_os = "windows") {
            if downloadNAME == "git".to_string() {
                "Git"
            } else {
                "None"
            }
        } else {
            "None"
        }
    };

    let alternateCODE: &str = {
        if cfg!(target_os = "linux") {
            if downloadNAME == "VSCode".to_string() {
                "code_"
            } else {
                "None"
            }
        } else if cfg!(target_os = "macos") {
            if downloadNAME == "VSCode".to_string() {
                "Visual Studio Code"
            } else {
                "None"
            }
        } else {
            "None"
        }
    };

    let mut unconfirmed: i16 = 0;
    //how many unwraps can one rapper stack if
    //one rapper could stack unwraps delicately
    for fileNAME in filesInDownloads {
        //these both return results to unwrap
        let fileNAME: String = fileNAME.expect("the pre string result which sets fileNAME has broken")
                                        .file_name()
                                        .into_string()
                                        .expect("the post string result which sets fileNAME has broken")
                                        .to_owned();

        //firefox works on mac


        //safari uses .download only so should be safe matching
        //it also appears to unpack zips by default
        //it also changes the name on vscode so it dont match
        //android studio on safari does not show the list of dls,
        //and opens at the bottom of the page but the link is at top
        //besides that though it works now on mac
        if downloadNAME == "android" {
            let debugBOX = 0;
        }
        let found: String = {
            if fileNAME.contains(&downloadNAME) || 
                fileNAME.contains(&"Unconfirmed"[..]) || 
                fileNAME.contains(&alternateGIT[..]) ||
                fileNAME.contains(&alternateCODE[..]) 
            {
                //panic!("the fileNAME is: \n{}", fileNAME);

                if fileNAME.contains(&".part"[..]) {
                    return "False".to_string();
                } else if fileNAME.contains(&".download"[..]) {
                    return "False".to_string();
                } else if fileNAME.contains(&".partial"[..]) {
                    return "False".to_string();
                } else if fileNAME.contains(&".~"[..]){
                    unconfirmed += 1;
                    continue
                } else if fileNAME.contains(&".crdownload"[..]) {
                    unconfirmed += 1;
                    continue
                } else {
                    let metaDATA = {
                        if cfg!(test) {
                            let filePATH: String = format!("{}{}", &testPATH, &fileNAME);
                            fs::metadata(filePATH).expect("the filesize metadata failed to set during test run")
                        } else {
                            let filePATH: String = format!("{}{}", &downloadsPATH, &fileNAME);
                            fs::metadata(filePATH).expect("the filesize metadata failed to set during build run")
                        }
                    };
                    if metaDATA.len() != 0 {
                        return "True".to_string();
                    } else {
                        return "False".to_string();
                    }
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
    //safari unpacks zips by default
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
    //this misleadingly named thing is where
    //we just integrate the contents of co_demo with
    //a new locally created flutter project

    //it turns out that .idea and .packages have local paths
    //but are git ignored normally so removing them from the repo
    //might mean that it runs without this step
    let errorBOX = String::from("");
    errorBOX
}

enum DownloadStatus {
    NotStarted,
    InProgress,
    Complete
}

fn main() {
    let path = env::home_dir().unwrap();
    let mut testPATH = path.to_str()
                        .unwrap()
                        .to_owned();
    println!("first stage of testpath: \n {}", testPATH);
    testPATH += "\\Desktop\\share\\test_data\\";
    println!("second stage of testpath: \n {}", testPATH);
    testPATH += "all_True";
    println!("the third stage of testpath: \n {}", testPATH);
    //this comes out all right
    check_dirs();

    let mut downloadMAP: IndexMap<String, String> = [
        ("StarUML".to_string(),  "None".to_string()),
        ("git".to_string(),      "None".to_string()),
        ("co_demo0".to_string(), "None".to_string()),
        ("flutter".to_string(),  "None".to_string()),
        ("VSCode".to_string(),   "None".to_string()),
        ("android".to_string(),  "None".to_string())    
    ].iter().cloned().collect();

    let _testPATH: String = "None".to_string();
    for downloadNAME in downloadMAP.clone().keys() {
        let answerBOX = is_complete(&downloadNAME, &_testPATH);

        if answerBOX == "True".to_string() {
            println!("{} is already complete!\n", downloadNAME)
        } else {
            println!("{} has not yet been completed\n", downloadNAME)
        }

        downloadMAP.insert(downloadNAME.to_string(), answerBOX);
    }
    //needs to give instructions to user before the crazy tab storm
    let now = time::Instant::now();
    let promptTIME = time::Duration::from_secs(150);

    'main: loop {
        for downloadNAME in downloadMAP.clone().keys() {
            if downloadMAP[downloadNAME] == "None".to_string() {
                if downloadNAME.to_owned() == "android".to_string() {
                    println!("\nplease start the android-studio download \n if you are a windows user:\n select the blue link that ends with '.exe'\n\nif you are a mac user:\n select the blue link that ends with '.dmg'\n\nif you are an Ubuntu user:\n select the blue link that ends in 'linux.zip'\n")
                } else {
                    println!("starting {} download now!\n", downloadNAME);
                }
                let testLIST = start_downloads(&downloadNAME);
                
                println!("waiting for browser to download...\n");

                if downloadNAME.to_owned() == "android".to_string() {
                    let sleepTIME = time::Duration::from_secs(20);
                    thread::sleep(sleepTIME);
                } else if downloadNAME.to_owned() == "git".to_string() {
                    let sleepTIME = time::Duration::from_secs(20);
                    thread::sleep(sleepTIME);
                } else {
                    let sleepTIME = time::Duration::from_secs(5);
                    thread::sleep(sleepTIME);
                }

                if !testLIST[4].contains("E: Failed") || !testLIST[4].contains("None") {
                    downloadMAP.insert(downloadNAME.to_string(),"True".to_string());
                }
            } else {
                continue
            }
        }

        for downloadNAME in downloadMAP.clone().keys() {
            if downloadNAME.to_owned() == "git" && cfg!(target_os = "linux") {
                continue
            } else  {
                let answerBOX = is_complete(&downloadNAME, &_testPATH);
                if downloadNAME == "android" {
                    let thisBOX = 1;
                }
                downloadMAP.insert(downloadNAME.to_string(), answerBOX);
            }
        }

        let mut completeNUM = 0;
        for downloadNAME in downloadMAP.clone().keys() {
            if downloadMAP[downloadNAME] == "True".to_string() {
                completeNUM += 1;
            } else {
                continue;
            }
        }

        if completeNUM == downloadMAP.keys().len() {
            println!("\n\nall the downloads are complete!\n");
            break 'main;

        } else if now.elapsed() > promptTIME {
            for downloadNAME in downloadMAP.clone().keys() {
                if downloadMAP[downloadNAME] == "None".to_string() {
                    println!("the {} download has not started despite multiple attempts\n", downloadNAME.to_string())  
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
            //these return options to unwrap
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "\\Downloads";
            env::set_current_dir(&downloadsPATH);
        } else if cfg!(unix){
            //these return options to unwrap
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
        let downloadNAME = "VSCode".to_string();
        if cfg!(target_os = "macos") {
            assert_eq!(start_downloads(&downloadNAME)[0], "osx")
        }else if cfg!(target_os = "windows") {
            assert_eq!(start_downloads(&downloadNAME)[0], "win32")
        }else if cfg!(target_os = "linux") {
            assert_eq!(start_downloads(&downloadNAME)[0], "linux64_deb")
        } else {
            assert_eq!(start_downloads(&downloadNAME)[0], "we currently only support Mac OS, Windows 10, and Ubuntu")
        }
        //need a cleanup func to erase the DL'd VSCode
    }

    #[test]
    fn start_downloads_git_switch() {
        //this works in linux, mac and windows
        let downloadNAME = "git".to_string();
        if cfg!(target_os = "macos")  {
            assert_eq!(start_downloads(&downloadNAME)[1], "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect")
        } else if cfg!(target_os = "windows") {
            assert_eq!(start_downloads(&downloadNAME)[1], "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe")
        } else {
            assert_eq!(start_downloads(&downloadNAME)[1], "git browser install currently only supports Mac OS and Windows 10")
        }
        //need a cleanup func to erase the DL'd git
    }

    #[test]
    fn start_downloads_uml_switch() {
        //this works in mac windows and linux
        let downloadNAME = "StarUML".to_string();
        if cfg!(target_os = "macos")  {
            assert_eq!(start_downloads(&downloadNAME)[2], "StarUML-3.0.2.dmg")
        } else if cfg!(target_os = "windows") {
            assert_eq!(start_downloads(&downloadNAME)[2], "StarUML%20Setup%203.0.2.exe")
        } else if cfg!(target_os = "linux") {
            assert_eq!(start_downloads(&downloadNAME)[2], "StarUML-3.0.2-x86_64.AppImage")        
        } else {
            assert_eq!(start_downloads(&downloadNAME)[2], "we currently only support Mac OS, Windows 10, and Ubuntu")
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
                let downloadNAME = fileLIST.get(index).unwrap().to_string();
                assert_eq!(start_downloads(&downloadNAME)[3], "None");
        }
    }
    */

    // start_downloads_linux_apt is at the bottom cos it brings up the sudo prompt
            //at this rate we should just feed in the path before things get too confusing
            //if we do these as macro parameterized tests then it will be much shorter

            //these entirely fail directory discovery in windows
    fn is_complete_switch_paths() -> String {
        let testPATH: String = { 
            if cfg!(windows){
                //these yield options to unwrap
                let path = env::home_dir().unwrap();
                let mut testPATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                testPATH += "\\Desktop\\share\\test_data\\";
                testPATH

            } else if cfg!(unix){
                //these yield options to unwrap
                let path = env::home_dir().unwrap();
                let mut testPATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                testPATH += "/Desktop/share/test_data/";
                testPATH

            } else {
                "we currently only support Windows 10, Ubuntu and Mac OS".to_string()
            }
        };
        return testPATH    
    }

    #[test]
    fn is_complete_switch_all_true() {
        //cannot find all_True directory
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "all_True\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "all_True/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "True");

    }

    #[test]
    fn is_complete_switch_two_none() {
        //test passes on mac
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "two_None\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "two_None/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "None");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "None");
    }

    #[test]
    fn is_complete_switch_one_false_chrome() {
        //test passes on mac
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_chrome\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_chrome/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "False");

    }

    #[test]
    //returns a None
    fn is_complete_switch_one_false_edge() {
        //test passes on win
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_edge\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_edge/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "False");

    }

    #[test]
    fn is_complete_switch_one_false_firefox() {
        //test passes on mac and win
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_firefox\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_firefox/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "False");

    }


    #[test]
    fn is_complete_switch_one_false_opera() {
        //cannot find opera directory
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_opera\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "all_True/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        //returns true instead of false, havent added identifier to is_complete logic yet
        assert_eq!(testLIST[5], "False");

    }
    
    #[test]
    fn is_complete_switch_one_false_safari() {
        //test passes on mac and win
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_safari\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_safari/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "False");

    }

    #[test]
    fn is_complete_switch_one_false_yandex() {
        //this one needs logic work
        //test passes on win
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_yandex\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_yandex/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        //this one thought VSCode was partial
        assert_eq!(testLIST[4], "True");
        assert_eq!(testLIST[5], "False");

    }

    #[test]
    fn is_complete_switch_one_false_uc() {
        //test passes on win
        let fileLIST: Vec<String> = vec!(
            "StarUML".to_string(),
            "git".to_string(),
            "co_demo0".to_string(), 
            "flutter".to_string(),
            "VSCode".to_string(),
            "android".to_string()
            );

        let mut testLIST: Vec<String> = [].to_vec();
        let testPATH: String = {
            if cfg!(windows) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_uc\\";
                testPATH
            } else if cfg!(unix) {
                let mut testPATH = is_complete_switch_paths();
                testPATH += "one_False_uc/";
                testPATH
            } else {
                let testPATH = "we currently only support Windows 10, Ubuntu and Mac OS".to_string();
                testPATH
            }
        };
        for index in 0..fileLIST.len() {
            //this returns an option to unwrap
            let downloadNAME = fileLIST.get(index).unwrap().to_string();
            let outBOX = is_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
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
        if cfg!(target_os = "linux"){
            let downloadNAME = "git".to_string();
            assert_eq!(start_downloads(&downloadNAME)[3], "None");

        }
    }

}
