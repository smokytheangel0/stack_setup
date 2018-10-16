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

//could be
    /*at the top-level directory of this distribution*/


//using snake_case for boxes as well as functions
//contradicts our attempted python and dart practice
#![allow(non_snake_case)]
//this fails on UpperHALF case,
//otherwise it is a good warning
#![allow(non_camel_case_types)]


///TODO
/// cargo install cargo-deb
///
/// RE: LINUX BROWSER COMPATIBILITY
/// use a grep a ps command on linux to see if
/// the browser is already running and then ask
/// the user to start their browser if its not
/// 
/// RE: pseudoPYTHON equality
/// keep it updated
/// 
/// RE: test triggered downloads
/// create a cleanup function to erase the
/// previously flutter *2 and now VSCode and git
/// switch test downloads
/// 
/// RE: browser tests
/// finish debugging opera logic
/// clean up browser tests within the limits of the vanilla test infrastructure
/// create macro parameterized test as well
/// 
/// RE: win debugger
/// need to set up a working msvc debugger on win
/// and gitignore the launch.json file
/// want to try out lldb on there even though it might not work
/// for now am using panic macros to reveal var contents during runtime
///
/// RE: download_complete spin check is using an entire core...
///
/// RE: windows quote bullshit
/// it seems to me like it would be better to handle these in an isolated manner that
/// doesnt affect the rest of the code, so we dont have to len around it nonstop

/// SETUP_DOWNLOADS NOTES:
///     ON MAC:
/// 
///     ON WIN:
/// 
///     ON LIN:
///         
/// 

//so far ~500 lines of function code
// ~100 lines in main
//and ~500 lines of test code
extern crate webbrowser;
extern crate indexmap;
use indexmap::IndexMap;
extern crate zip;
extern crate dirs;

use std::fs;
use std::io;
use std::env;
use std::fs::ReadDir;
use std::{thread, time};
use std::process::Command;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[cfg(windows)]
    extern crate winreg;
#[cfg(windows)]
    use winreg::RegKey;
#[cfg(windows)]
    use winreg::enums::*;



//#region py_check_dirs
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
//#endregion
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

//#region py_start_downloads
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
//#endregion
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
//version specific will break on update (current as of 2OCT)
            "https://github.com/git-for-windows/git/releases/download/v2.19.0.windows.1/Git-2.19.0-64-bit.exe"
        } else if cfg!(target_os = "macos") {
            "https://sourceforge.net/projects/git-osx-installer/files/git-2.19.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
        } else {
            "git browser install currently only supports Mac OS and Windows 10"
        }
    };
    testLIST[1] = String::from(gitURL);

//version specific will break on update
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

    } else if downloadNAME == "android" {
        webbrowser::open("https://developer.android.com/studio/#downloads")
                    .expect("there was an error opening the android studio web page in your browser");
        return testLIST;

    } else {
        testLIST[3] = "the switch branches have all been avoided !!!".to_string();
        return testLIST;
    }
    
}

//#region py_is_complete
///this is what the [download_complete] function is in python
///```python
/// [replace all 'is not' with '!=']
/// [replace all 'is' with '==']
/// import os
/// import platform
/// from pathlib import Path
/// TEST_FLAG = False
/// #outBOX is String
/// def download_complete(downloadNAME, &testPATH):
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
//#endregion
fn download_complete(downloadNAME: &str, testPATH: &str) -> String {
    println!("checking to see if the {} is complete", &downloadNAME);
    //this function called from main and the associated tests
    //confirmed working in Mac OS, Windows 10, and Ubuntu 18.04

    let outBOX: String = "None".to_string();

    let downloadsPATH: String = {
        if cfg!(windows){
            let path = dirs::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "\\Downloads\\";
            downloadsPATH
        }else if cfg!(unix){
            let path = dirs::home_dir().unwrap();
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
                                    
        //to ignore previous crdownloads, flag the first run, if any are found, keep its number
        //then add a branch inside unconfirmed branch that ignores that number and only returns true from a new number

        let found: String = {
            if fileNAME.contains(&downloadNAME) || 
                fileNAME.contains(&"Unconfirmed"[..]) || 
                fileNAME.contains(&alternateGIT[..]) ||
                fileNAME.contains(&alternateCODE[..]) 
            {
                //panic!("the fileNAME is: \n{}", fileNAME);

                if fileNAME.contains(&".partial"[..]) {
                    return "False".to_string();
                } else if fileNAME.contains(&".opdownload"[..]) {
                    return "False".to_string();
                } else if fileNAME.contains(&".download"[..]) {
                    return "False".to_string();
                } else if fileNAME.contains(&".part"[..]) {
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

//this now only needs to support unzipping android-studio on linux,
//the flutter and co_demo must be cloned after installing git and the like
fn extract_studio() {
    println!("extracting android studio");
    let downloadNAME = "android".to_string();

    let downloadsPATH: String = {
        if cfg!(windows){
            let path = dirs::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "\\Downloads\\";
            downloadsPATH
        }else if cfg!(unix){
            let path = dirs::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "/Downloads/";
            downloadsPATH
        } else {
            "we currently only support Windows 10, Ubuntu and Mac OS".to_string()
        }
    };
    
    let filesInDownloads = fs::read_dir(&downloadsPATH).expect("the read_dir that sets filesInDownloads broke");
    let mut filePATH: String = ".None".to_string();
    for fileNAME in filesInDownloads {
        let fileNAME: String = fileNAME.expect("the pre string result which sets fileNAME has broken")
                                        .file_name()
                                        .into_string()
                                        .expect("the post string result which sets fileNAME has broken")
                                        .to_owned();
        
        if fileNAME.contains(&downloadNAME)
        {   
            //never finds git during execution
            filePATH = {
                format!("{}{}", &downloadsPATH, &fileNAME)                
            }
        }
    }
    let len = filePATH.len();
    if filePATH[len-3..] == "zip".to_string() {
        let workingPATH: String = {            
            let path = dirs::home_dir().unwrap();
            let mut workingPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            workingPATH += "/Desktop/SDKs/";
            workingPATH
        };

        fs::create_dir_all(&workingPATH).expect("creating dirs failed");
        env::set_current_dir(&workingPATH).expect("setting cwd failed");
        let fname = {
            std::path::Path::new(&filePATH)
        };
        let file = fs::File::open(&fname).expect("failed to open the file at filepath");

        let mut archive = zip::ZipArchive::new(file).expect("failed to make an archive in memory from file");

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = file.sanitized_name();

            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).expect("failed to create directories");
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).expect("failed to extract file");
                    }
                }
                let mut outfile = fs::File::create(&outpath).expect("failed to create outfile");
                io::copy(&mut file, &mut outfile).expect("failed to copy outfile to output dir");
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }  
        }
    }
}

fn install_downloads(downloadNAME: &str) {
    println!("starting the {} installer", &downloadNAME);
    let downloadsPATH: String = {
        if cfg!(windows){
            let path = dirs::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "\\Downloads\\";
            downloadsPATH
        }else if cfg!(unix){
            let path = dirs::home_dir().unwrap();
            let mut downloadsPATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            downloadsPATH += "/Downloads/";
            downloadsPATH
        } else {
            "we currently only support Windows 10, Ubuntu and Mac OS".to_string()
        }
    };

    let alternateGIT: &str = {
        if cfg!(target_os = "windows") {
            if downloadNAME == "git".to_string() {
                "Git"
            } else {
                ".None"
            }
        } else {
            ".None"
        }
    };

    let alternateCODE: &str = {
        if cfg!(target_os = "linux") {
            if downloadNAME == "VSCode".to_string() {
                "code_"
            } else {
                ".None"
            }
        } else if cfg!(target_os = "macos") {
            if downloadNAME == "VSCode".to_string() {
                "Visual Studio Code"
            } else {
                ".None"
            }
        } else {
            ".None"
        }
    };
    
    let filesInDownloads = fs::read_dir(&downloadsPATH).expect("the read_dir that sets filesInDownloads broke");
    let mut filePATH: String = ".None".to_string();
    let mut properNAME = ".None".to_string();
    for fileNAME in filesInDownloads {
        let fileNAME: String = fileNAME.expect("the pre string result which sets fileNAME has broken")
                                        .file_name()
                                        .into_string()
                                        .expect("the post string result which sets fileNAME has broken")
                                        .to_owned();
        
        if fileNAME.contains(&downloadNAME) ||
           fileNAME.contains(&alternateCODE)||
           fileNAME.contains(&alternateGIT)
        {   
            //never finds git during execution
            filePATH = {
                if cfg!(target_os = "windows") {
                    format!("'{}{}'", &downloadsPATH, &fileNAME)
                } else {
                    format!("{}{}", &downloadsPATH, &fileNAME)
                }
            };
            properNAME = fileNAME;
        }
    }

    let len = filePATH.len();
    if filePATH[len-4..len-1] == "exe".to_string() ||
       filePATH[len-3..len] == "deb".to_string() 
    {
        let setupCMD = {
            if cfg!(target_os = "windows") {
                ["powershell.exe","Start-Process", "-FilePath"]
            } else if cfg!(target_os = "linux") {
                ["sudo", "dpkg", "-i"]
            } else {
                ["None",
                 "None",
                 "None"]
            }
        };
        if cfg!(target_os = "linux") {
            Command::new("sudo").arg("apt").arg("install").arg("libgconf-2-4").arg("git");
        }
        Command::new(&setupCMD[0])
            .arg(&setupCMD[1]).arg(&setupCMD[2]).arg(&filePATH)
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
        });

    }

    else if filePATH[len-3..len] == "dmg".to_string() ||
        filePATH[len-3..len] == "app".to_string() {
        
        let mut volumePATH: String = ".None".to_string();
        let mut appPATH: String = ".None".to_string();
        if filePATH[len-3..len] == "dmg".to_string() {
            let mountCMD = ["hdiutil", "mount"];
            Command::new(&mountCMD[0])
                .arg(&mountCMD[1]).arg(&filePATH)
                .output().expect("failed to execute mount cmd");

            if cfg!(target_os = "macos") {
                let foldersInVolumes = fs::read_dir("/Volumes/").expect("the read_dir that sets foldersInVolumes broke");
                for folderNAME in foldersInVolumes {
                    let folderNAME: String = folderNAME.expect("the pre string result which sets fileNAME has broken")
                                                    .file_name()
                                                    .into_string()
                                                    .expect("the post string result which sets fileNAME has broken")
                                                    .to_owned();
                    
                    let mut downloadCHARS: Vec<char> = downloadNAME.chars().collect();
                    downloadCHARS[0] = downloadCHARS[0].to_uppercase().nth(0).expect("downloadCHARS first index is out of bounds");
                    let upperNAME: String = downloadCHARS.into_iter().collect();

                    if folderNAME.contains(&upperNAME) {   
                        volumePATH = format!("{}{}", &"/Volumes/"[..], &folderNAME);
                        let filesInVolume = fs::read_dir(&volumePATH).expect("the read_dir that sets filesInVolume broke");
                        for itemNAME in filesInVolume {
                            let itemNAME = itemNAME.expect("the prestring result which sets the itemNAME has broken")
                                                .file_name()
                                                .into_string()
                                                .expect("the post string result which sets itemNAME has broken")
                                                .to_owned();

                            if itemNAME.contains(&".app"[..]) ||
                                itemNAME.contains(&".pkg"[..]) {
                                appPATH = format!("{}/{}", &volumePATH, &itemNAME);
                            }
                        }
                    }
                }
            }
        }
        if filePATH[len-3..len] == "app".to_string() {
            appPATH = filePATH.clone();
        }

        if !downloadNAME.contains(&"git"[..]) {

            let copyCMD = ["sudo", "cp", "-R"];
            Command::new(&copyCMD[0])
                            .arg(&copyCMD[1])
                            .arg(&copyCMD[2])
                            .arg(&appPATH)
                            .arg("/Applications")
                            .output().expect("failed to execute copy cmd");
        }

        if volumePATH != ".None".to_string() {
            let unmountCMD = ["hdiutil", "unmount"];
            Command::new(&unmountCMD[0])
                .arg(&unmountCMD[1]).arg(&volumePATH)
                .output().expect("failed to execute unmount cmd");
        }
        
    } else {
        if filePATH.contains(&"AppImage"[..]) {
            Command::new("chmod").arg("+x").arg(&filePATH).output().expect("failed to make AppImage executable");
            env::set_current_dir(&downloadsPATH).expect("setting cwd failed");
            let commandPATH = "./".to_string() + &properNAME;
            Command::new(commandPATH)
                            .output().expect("failed to execute appimage");
        } else if filePATH.contains(&"android"[..]) {
            let workingPATH: String = {
                if cfg!(unix){
                        let path = dirs::home_dir().unwrap();
                        let mut workingPATH = path.to_str()
                                                    .unwrap()
                                                    .to_owned();
                        workingPATH += "/Desktop/SDKs/android-studio/bin/studio.sh";
                        
                        workingPATH
                } else {
                    "the console install of star and android only works on linux".to_string()
                }
            };
            Command::new("chmod").arg("+x").arg(&workingPATH).output().expect("failed to make sh executable");
            Command::new("sh").arg(&workingPATH)
                            .output().expect("failed to execute studio.sh");

        } else {
            return;
        }
    }
    //if mac, launch android studio to set path
    //install brew
}

fn clone_repo(downloadNAME: &str) {
    println!("cloning: {}", &downloadNAME);
    
    let clonePATH = {
        if downloadNAME == "flutter".to_owned(){
            if cfg!(target_os = "windows"){
                let path = dirs::home_dir().unwrap();
                let mut clonePATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                clonePATH += "\\Desktop\\SDKs";
                clonePATH
            } else {
                let path = dirs::home_dir().unwrap();
                let mut clonePATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                clonePATH += "/Desktop/SDKs";
                clonePATH
            }

        } else {
            if cfg!(target_os = "windows"){
                let path = dirs::home_dir().unwrap();
                let mut clonePATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                clonePATH += "\\Desktop\\Code";
                clonePATH
            } else {
                let path = dirs::home_dir().unwrap();
                let mut clonePATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                clonePATH += "/Desktop/Code";
                clonePATH
            }
        }
    };

    if downloadNAME == "flutter".to_string() {
        fs::create_dir_all(&clonePATH).expect("failed to create SDK dir");
        //probably fails on this
        env::set_current_dir(&clonePATH).expect("failed to set SDK dir as cwd");
        Command::new("git").arg("clone").arg("https://github.com/flutter/flutter.git").output().expect("failed to clone flutter repo");
        return
    } else if downloadNAME == "co_demo0".to_string() {
        fs::create_dir_all(&clonePATH).expect("failed to create Code dir");
        //fails on this because dir was not created
        env::set_current_dir(&clonePATH).expect("failed to set Code dir as cwd");
        Command::new("git").arg("clone").arg("https://github.com/smokytheangel0/co_demo0.git").output().expect("failed to clone co_demo0 repo");
        return
    } else {
        println!("{} is in the wrong function, it is in clone_repo(&downloadNAME)", &downloadNAME);
        return
    }

}

fn set_path() {
    println!("setting path!");
    #[cfg(unix)]
    {
        let path = dirs::home_dir().unwrap();
        let mut homePATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
        homePATH += "/.bash_profile";

        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&homePATH)
                            .unwrap();

        if cfg!(target_os = "linux"){
            writeln!(file, "export ANDROID_HOME=$HOME/Android/Sdk").expect("failed to write linux android_home");
        } else if cfg!(target_os = "macos") {
            writeln!(file, "export ANDROID_HOME=$HOME/Library/Android/Sdk").expect("failed to write mac android_home");
        }
        writeln!(file, "export PATH=$HOME/Desktop/SDKs/flutter/bin:$PATH").expect("failed to write unix flutter path");
        writeln!(file, "export PATH=$ANDROID_HOME/tools:$PATH").expect("failed to write unix tools path");
        writeln!(file, "export PATH=$ANDROID_HOME/platform-tools:$PATH").expect("failed to write unix platform tools path");
        Command::new("source").arg(&homePATH).output().expect("failed to refresh bash_profile");
    }
    #[cfg(windows)]
    {
        let addPATH = "%USERPROFILE%\\Desktop\\SDKs\\flutter\\bin;%USERPROFILE\\AppData\\Local\\Android\\Sdk\\tools;%USERPROFILE\\AppData\\Local\\Android\\Sdk\\platform-tools;";
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let environment = hklm.open_subkey("Environment").expect("could not open Environment key for flutter");
        let oldPATH: String = environment.get_value("Path").expect("could not open Path value for flutter");
        let newPATH = oldPATH + &addPATH;
        Command::new("powershell.exe").arg("setx").arg("Path").arg(&newPATH).output().expect("failed to set path");
        Command::new("powershell.exe").arg("set").arg("Path").arg(&newPATH).output().expect("failed to set path");
        Command::new("powershell.exe").arg("setx").arg("ANDROID_HOME").arg("%USERPROFILE\\AppData\\Local\\Android\\Sdk;").output().expect("failed to make android_home var");
        Command::new("powershell.exe").arg("set").arg("ANDROID_HOME").arg("%USERPROFILE\\AppData\\Local\\Android\\Sdk;").output().expect("failed to make android_home var");
    }
}
fn git_install_complete() -> String {
    let mut gitFOLDER = {
        if cfg!(target_os = "windows"){
            //might use path prefix to make this drive agnostic
            let path = dirs::home_dir().unwrap();
            let mut gitFOLDER = path.to_str()
                                .unwrap()
                                .to_owned();
            gitFOLDER += "\\AppData\\Local\\Programs\\";
            gitFOLDER

        } else {
            let gitFOLDER = "/usr/bin/".to_owned();
            gitFOLDER
        }
    };
        for _iteration in 0..1 {
            println!("searching {}", &gitFOLDER);
            let programFOLDERS = fs::read_dir(&gitFOLDER).expect("No git app folder found");
            for folderNAME in programFOLDERS {
                let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                                .file_name()
                                                .into_string()
                                                .expect("the post string result which sets folderNAME has broken")
                                                .to_owned();
                if cfg!(target_os = "windows"){
                    if folderNAME.contains(&"Git"[..]) {
                        return "True".to_owned()
                    } else {
                        continue
                    }
                } else {
                    if folderNAME == &"git"[..] {
                        return "True".to_owned()
                    } else {
                        continue
                    }
                }

            }
            if cfg!(target_os = "windows") {
                gitFOLDER = "C:\\Program Files\\".to_owned();
                continue;
            } else {
                break;
            }
        }
    return "False".to_owned()
}

fn setup_xcode() -> String {
    //ask user if they have apple ID
    //prompt user to open mac store or open from here
    //spin check xcode or other determining location to
    //see if its done and remind like android if not done

    //MUST HAVE LATEST MACOS FOR XCODE
    let errorBOX = String::from("");
    errorBOX
}

fn show_licences() -> String {
    //show android licences
    //if mac show xcode licence
    let errorBOX = String::from("");
    errorBOX
}

fn flutter_doctor() -> String {
    //see what flutter doctor looks like from stdout and stderror
    //
    let errorBOX = "".to_string();
    errorBOX
}

fn background_test() -> String {
    let errorBOX = String::from("");
    errorBOX
}
enum DownloadStatus {
    NotStarted,
    InProgress,
    Complete
}
// Copyright 2012-2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! cfg_if {
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        __cfg_if_items! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    }
}

fn main() {
    check_dirs();

    let mut downloadMAP: IndexMap<String, String> = [
        ("StarUML".to_string(),  "None".to_string()),
        ("git".to_string(),      "None".to_string()),
        ("VSCode".to_string(),   "None".to_string()),
        ("android".to_string(),  "None".to_string())    
    ].iter().cloned().collect();

    let _testPATH: String = "None".to_string();
    for downloadNAME in downloadMAP.clone().keys() {
        let answerBOX = download_complete(&downloadNAME, &_testPATH);

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
                    let sleepTIME = time::Duration::from_secs(30);
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
                let sleepTIME = time::Duration::from_secs(1);
                thread::sleep(sleepTIME);
                let answerBOX = download_complete(&downloadNAME, &_testPATH);
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

    if cfg!(target_os = "linux"){
        extract_studio();
    }

    for downloadNAME in downloadMAP.clone().keys() {
        install_downloads(&downloadNAME);
//might want to check if each install is complete before continuing
//only on windows though, this will spread out all the admin prompts
    }
    while git_install_complete() == "False"{
        let sleepTIME = time::Duration::from_secs(20);
        thread::sleep(sleepTIME);
    }
    let cloneMAP: IndexMap<String, String> = [
        ("co_demo0".to_string(), "False".to_string()),
        ("flutter".to_string(),  "False".to_string()),
    ].iter().cloned().collect();

    for downloadNAME in cloneMAP.clone().keys() {
        clone_repo(&downloadNAME);
    }

    set_path();
    println!("{}", &"install, clone and paths complete"[..]);
    let sleepTIME = time::Duration::from_secs(60);
    thread::sleep(sleepTIME);
}

//the tests all fail properly on mac
//but the git, vs, and flutter extracted tests pass on linux on a bare install (see image)
//windows is untested
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn is_xcode_installed(){
        let foldersInApplications = fs::read_dir("/Applications/").expect("the read_dir that sets foldersInApplications broke");
        for folderNAME in foldersInApplications {
            let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                            .file_name()
                                            .into_string()
                                            .expect("the post string result which sets folderNAME has broken")
                                            .to_owned();
            
            if folderNAME.contains(&"Xcode"[..]) {
                assert_eq!(true, true);
                return
            } else {
                continue
            }

        }
        panic!("xCode installation not found");
    }

    /*
    might need to have this piggyback off flutter doctor
    #[test]
    #[cfg(target_os = "macos")]
    fn xcode_license_accepted(){

    }
    */

    #[test]
    fn is_flutter_extracted(){
        let sdkPATH = {
            if cfg!(target_os = "windows"){
                let path = dirs::home_dir().unwrap();
                let mut sdkPATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                sdkPATH += "\\Desktop\\SDKs\\";
                sdkPATH
            } else {
                let path = dirs::home_dir().unwrap();
                let mut sdkPATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                sdkPATH += "/Desktop/SDKs/";
                sdkPATH
            }
        };


        let foldersInSDKs = fs::read_dir(&sdkPATH).expect("No flutter repo folder found");
        for folderNAME in foldersInSDKs {
            let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                            .file_name()
                                            .into_string()
                                            .expect("the post string result which sets folderNAME has broken")
                                            .to_owned();
            
            if folderNAME.contains(&"flutter"[..]) {
                assert_eq!(true, true);
                return
            } else {
                continue
            }

        }
        panic!("the flutter folder was not found");
    }

    #[test]
    fn is_flutter_on_path(){
        #[cfg(unix)]
        {
            let path = dirs::home_dir().unwrap();
            let mut homePATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            homePATH += "/.bash_profile";
            let output = Command::new("cat").arg(&homePATH).output().expect("could not cat bash_profile");
            assert_eq!(String::from_utf8_lossy(&output.stdout).contains("$HOME/Desktop/SDKs/flutter/bin:$PATH"), true);
        }

        #[cfg(windows)]
        {
            let hklm = RegKey::predef(HKEY_CURRENT_USER);
            let environment = hklm.open_subkey("Environment").expect("could not open Environment key for flutter");
            let currentPATH: String = environment.get_value("Path").expect("could not open Path value for flutter");
            assert_eq!(currentPATH.contains("%USERPROFILE%\\Desktop\\SDKs\\flutter\\bin;"), true)

        }
    }

    #[test]
    fn is_android_installed(){
        let androidFOLDER = {
            if cfg!(target_os = "windows"){
                //might use path prefix to make this drive agnostic
                let androidFOLDER = "C:\\Program Files\\".to_owned();
                androidFOLDER
            } else if cfg!(target_os = "macos") {
                let androidFOLDER = "/Applications/".to_owned();
                androidFOLDER

            }else {
                let path = dirs::home_dir().unwrap();
                let mut androidFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();
                androidFOLDER += "/Desktop/SDKs/";
                androidFOLDER
            }
        };
        let programFOLDERS = fs::read_dir(&androidFOLDER).expect("No android app folder found");
        for folderNAME in programFOLDERS {
            let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                            .file_name()
                                            .into_string()
                                            .expect("the post string result which sets folderNAME has broken")
                                            .to_owned();
            if cfg!(target_os = "windows"){
                if folderNAME.contains(&"Android"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            } else if cfg!(target_os = "macos") {
                if folderNAME.contains(&"Android Studio"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            } else {
                if folderNAME.contains(&"android-studio"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            }

        }
        panic!("the android studio installation was not found");
    }

    #[test]
    fn is_android_on_path(){
        let androidPATHs = {
            if cfg!(target_os = "linux") {
                let androidPATH = "$HOME/Android/Sdk".to_owned();
                let androidPATHs = [
                                    androidPATH, 
                                    "$ANDROID_HOME/tools:$PATH".to_owned(), 
                                    "$ANDROID_HOME/platform-tools:$PATH".to_owned()
                                    ];
                androidPATHs
            } else {
                let androidPATH = "$HOME/Library/Android/Sdk".to_owned();
                let androidPATHs = [
                                    androidPATH, 
                                    "$ANDROID_HOME/tools:$PATH".to_owned(), 
                                    "$ANDROID_HOME/platform-tools:$PATH".to_owned()
                                    ];
                androidPATHs
            }
        };
        
        #[cfg(unix)]
        {
            let path = dirs::home_dir().unwrap();
            let mut homePATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            homePATH += "/.bash_profile";
            let output = Command::new("cat").arg(&homePATH).output().expect("could not cat bash_profile");
            println!("androidPATH is: {}", &androidPATHs[0]);
            println!("currentPATH is: {}", String::from_utf8_lossy(&output.stdout));
            assert_eq!(String::from_utf8_lossy(&output.stdout).contains(&androidPATHs[0]), true);
            assert_eq!(String::from_utf8_lossy(&output.stdout).contains(&androidPATHs[1]), true);
            assert_eq!(String::from_utf8_lossy(&output.stdout).contains(&androidPATHs[2]), true);

        }

        #[cfg(windows)]
        {
            //ANDROID_HOME
            let hklm = RegKey::predef(HKEY_CURRENT_USER);
            let environment = hklm.open_subkey("Environment").expect("could not open Environment key for flutter");
            let currentPATH: String = environment.get_value("ANDROID_HOME").expect("could not open Path value for flutter");
            assert_eq!(currentPATH.contains("%USERPROFILE\\AppData\\Local\\Android\\Sdk;"), true);
            //tools on Path
            let hklm = RegKey::predef(HKEY_CURRENT_USER);
            let environment = hklm.open_subkey("Environment").expect("could not open Environment key for flutter");
            let currentPATH: String = environment.get_value("Path").expect("could not open Path value for flutter");
            assert_eq!(currentPATH.contains("%USERPROFILE\\AppData\\Local\\Android\\Sdk\\tools;"), true);
            assert_eq!(currentPATH.contains("%USERPROFILE\\AppData\\Local\\Android\\Sdk\\platform-tools;"), true);            

        }

    }

    #[test]
    fn android_license_accepted(){
        assert_eq!(false, true);
    }

    #[test]
    fn is_staruml_installed(){
        let starFOLDER = {
            if cfg!(target_os = "windows"){
                //might use path prefix to make this drive agnostic
                let starFOLDER = "C:\\Program Files\\".to_owned();
                starFOLDER
            } else if cfg!(target_os = "macos") {
                let starFOLDER = "/Applications/".to_owned();
                starFOLDER

            }else {
                let path = dirs::home_dir().unwrap();
                let mut starFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();
                starFOLDER += "/Downloads/";
                starFOLDER
            }
        };
        let programFOLDERS = fs::read_dir(&starFOLDER).expect("No star app folder found");
        for folderNAME in programFOLDERS {
            let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                            .file_name()
                                            .into_string()
                                            .expect("the post string result which sets folderNAME has broken")
                                            .to_owned();
            if cfg!(target_os = "windows"){
                if folderNAME.contains(&"StarUML"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            } else if cfg!(target_os = "macos") {
                if folderNAME.contains(&"StarUML"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            } else {
                if folderNAME.contains(&"StarUML"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            }

        }
        panic!("starUML installation not found");
    }

    #[test]
    fn is_git_installed(){
        let gitFOLDER = {
            if cfg!(target_os = "windows"){
                //this could be either AppData or Program Files
                let path = dirs::home_dir().unwrap();
                let mut gitFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();
                gitFOLDER += "\\AppData\\Local\\Programs\\";
                gitFOLDER

            } else {
                let gitFOLDER = "/usr/bin/".to_owned();
                gitFOLDER
            }
        };
        
        for _iteration in 0..1 {
            println!("searching {}", &gitFOLDER);
            let programFOLDERS = fs::read_dir(&gitFOLDER).expect("No git app folder found");
            for folderNAME in programFOLDERS {
                let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                                .file_name()
                                                .into_string()
                                                .expect("the post string result which sets folderNAME has broken")
                                                .to_owned();
                if cfg!(target_os = "windows"){
                    if folderNAME.contains(&"Git"[..]) {
                        assert_eq!(true, true);
                        return
                    } else {
                        continue
                    }
                } else {
                    if folderNAME == &"git"[..] {
                        assert_eq!(true, true);
                        return
                    } else {
                        continue
                    }
                }

            }
            if cfg!(target_os = "windows") {
                let gitFOLDER = "C:\\Program Files\\".to_owned();
                continue;
            } else {
                break;
            }
        }
        panic!("git installation not found");

    }

    #[test]
    fn is_vs_installed(){
        let vsFOLDER = {
            if cfg!(target_os = "windows"){
                //might use path prefix to make this drive agnostic
                let path = dirs::home_dir().unwrap();
                let mut vsFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();
                vsFOLDER += "\\AppData\\Local\\Programs\\";
                vsFOLDER

            } else if cfg!(target_os = "macos") {
                let vsFOLDER = "/Applications/".to_owned();
                vsFOLDER

            }else {
                let vsFOLDER = "/usr/bin/".to_owned();
                vsFOLDER
            }
        };
        let programFOLDERS = fs::read_dir(&vsFOLDER).expect("No vscode app folder found");
        for folderNAME in programFOLDERS {
            let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                            .file_name()
                                            .into_string()
                                            .expect("the post string result which sets folderNAME has broken")
                                            .to_owned();
            if cfg!(target_os = "windows"){
                if folderNAME.contains(&"Microsoft VS Code"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            } else if cfg!(target_os = "macos") {
                if folderNAME.contains(&"Visual Studio Code"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            } else {
                if folderNAME == &"code"[..] {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            }

        }
        panic!("the VSCode folder installation was not found");
    }

    #[test]
    fn is_co_demo_extracted(){
        let coFOLDER = {
            if cfg!(target_os = "windows"){
                let path = dirs::home_dir().unwrap();
                let mut coFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();
                coFOLDER += "\\Desktop\\Code\\";
                coFOLDER
            } else {
                let path = dirs::home_dir().unwrap();
                let mut coFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();
                coFOLDER += "/Desktop/Code/";
                coFOLDER
            }
        };
        let programFOLDERS = fs::read_dir(&coFOLDER).expect("No co_demo repo folder found");
        for folderNAME in programFOLDERS {
            let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                            .file_name()
                                            .into_string()
                                            .expect("the post string result which sets folderNAME has broken")
                                            .to_owned();

                if folderNAME.contains(&"co_demo0"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }

        }
        panic!("co_demo folder not found");

    }

    #[test]
    fn does_doctor_return_well(){
        assert_eq!(false, true);
    }

    
    #[test]
    #[ignore]
    fn main_(){
        //this test should include placing the files from test in
        //downloads and seeing if it completes correctly with or without
        //any of the items complete

        //actually I dont think I can test main without running the whole thing
        //so maybe this is material for an integration test
    }


    #[test]
    fn check_dirs_error_msg(){
        //this works in Mac, Windows and Linux

        assert_eq!(check_dirs(), 1);

        //still very proud of my first test, and glad
        //types are explicit
        if cfg!(windows){
            let path = dirs::home_dir().unwrap();
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "\\Downloads";
            env::set_current_dir(&downloadsPATH).expect("could not set working directory to downloadsPATH");
        } else if cfg!(unix){
            let path = dirs::home_dir().unwrap();
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "/Downloads";
            env::set_current_dir(&downloadsPATH).expect("could not set working directory to downloadsPATH");
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
    
    
    #[test]
    #[ignore]
    //THIS TEST SHOULD NOT BE RUN EVERYTIME
    //IT OPENS FIVE TABS IN THE BROWSER
    //AND STARTS FIVE DOWNLOADS
    fn start_downloads_thread_switch(){
        //this works in linux, mac and windows
        //this should control for some conditions, 
        //like no internet access, slow internet, firewalls, proxies etc
        let fileLIST = [
                        "StarUML".to_string(),                
                        "VSCode".to_string(),
                        "android".to_string()
                    ];

        for index in 0..fileLIST.len() {
                let downloadNAME = fileLIST.get(index).unwrap().to_string();
                assert_eq!(start_downloads(&downloadNAME)[3], "None");
        }
    }


    // start_downloads_linux_apt is at the bottom cos it brings up the sudo prompt
            //at this rate we should just feed in the path before things get too confusing
            //if we do these as macro parameterized tests then it will be much shorter

            //these entirely fail directory discovery in windows

    fn is_complete_switch_paths() -> String {
        let testPATH: String = { 
            if cfg!(windows){
                let path = dirs::home_dir().unwrap();
                let mut testPATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                testPATH += "\\Desktop\\share\\test_data\\";
                testPATH

            } else if cfg!(unix){
                let path = dirs::home_dir().unwrap();
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
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
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
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
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
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
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
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
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
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
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
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
            testLIST.push(outBOX);
        }
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        //returns true instead of false, havent added identifier to download_complete logic yet
        assert_eq!(testLIST[5], "False");

    }
    
    #[test]
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
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
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
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
    #[ignore]
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
            let outBOX = download_complete(&downloadNAME, &testPATH);
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
    #[ignore]
    fn start_downloads_linux_apt(){
        if cfg!(target_os = "linux"){
            let downloadNAME = "git".to_string();
            assert_eq!(start_downloads(&downloadNAME)[3], "None");

        }
    }

}
