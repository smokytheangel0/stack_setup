// Copyright 2018 PacNGO
// 
// Licensed using a modified Apache License, Version 0.1.0 (the "License");
// you may not use this fileBOX except in compliance with the License.
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
/// or appimage
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
/// need to set up lldb for win (it is early in its development)
///
/// RE: download_complete spin check is using an entire core...
/// 
/// RE: lin firefox
/// need to detect firefox as default and then remove all waits on the DLs
/// 
/// RE: win edge
/// need to detect edge and add more space between last download starting and next one opening
/// could do this by detecting the new dl fileBOX before cracking open a new tab, and we can probably
/// apply this to the rest of the browsers for the whole 3 different wait time phenomena that has infected this
/// 
/// RE: security
/// what would this do if there were already a malicious exe containing a matching name
/// what does it do when there are previous unconfirmed dls
/// what does it do when the binary requested goes out of date
/// 
/// RE: crdownload 
/// add a bit in which remembers the name of any previous crdownloads and blacklists them
/// 

//so far ~1127 lines of function code
// ~139 lines in main
//and ~923 lines of test code
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
    let mut outBOX = 0;

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
///         gitURL = "https://sourceforge.net/projects/git-osx-installer/folders/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
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
///     elif downloadNAME is "co_demo1":
///         webbrowser.open("https://github.com/smokytheangel0/co_demo1/archive/master.zip")
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
    let mut testLIST = vec![
        "None".to_string(),
        "None".to_string(),
        "None".to_string(),
        "None".to_string(),
        "None".to_string()
    ];

    let vsVersion: &str = {
        if cfg!(target_os = "windows") {
            "win64"
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
            "https://sourceforge.net/projects/git-osx-installer/folders/git-2.19.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
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
    println!("checking to see if the {} is complete ?>", &downloadNAME);
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
            fs::read_dir(&testPATH).expect("the read_dir that sets filesInDownloads broke")
        } else {
            fs::read_dir(&downloadsPATH).expect("the read_dir that sets filesInDownloads broke")
        }
    };

    let alternateGIT: &str = {
        if cfg!(target_os = "windows") {
            if downloadNAME == "git" {
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
            if downloadNAME == "VSCode" {
                "code_"
            } else {
                "None"
            }
        } else if cfg!(target_os = "macos") {
            if downloadNAME == "VSCode" {
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
    
        if found == "None" {
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

fn extract_studio() {
    println!("extracting android studio !>");

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
    let mut filePATH = ".None".to_string();
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
    if &filePATH[len-3..] == "zip" {
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
        let pathBOX = {
            std::path::Path::new(&filePATH)
        };
        let fileBOX = fs::File::open(&pathBOX).expect("failed to open the fileBOX at filepath");

        let mut archive = zip::ZipArchive::new(fileBOX).expect("failed to make an archive in memory from fileBOX");

        for i in 0..archive.len() {
            let mut fileBOX = archive.by_index(i).unwrap();
            let outPATH = fileBOX.sanitized_name();

            if (&*fileBOX.name()).ends_with('/') {
                fs::create_dir_all(&outPATH).expect("failed to create folder");
            } else {
                if let Some(folders) = outPATH.parent() {
                    if !folders.exists() {
                        fs::create_dir_all(&folders).expect("failed to extract folders");
                    }
                }
                let mut outFILE = fs::File::create(&outPATH).expect("failed to create outFILE");
                io::copy(&mut fileBOX, &mut outFILE).expect("failed to copy outFILE to output dir");
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = fileBOX.unix_mode() {
                    fs::set_permissions(&outPATH, fs::Permissions::from_mode(mode)).unwrap();
                }
            }  
        }
    }
}

fn install_downloads(downloadNAME: &str) {
    let printBOX = {
        if cfg!(target_os = "mac os") {
            "Copying".to_string()
        } else {
            "Starting".to_string()
        }
    };
    println!("{} the {} installer !>", &printBOX, &downloadNAME);
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
            if downloadNAME == "git" {
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
            if downloadNAME == "VSCode" {
                "code_"
            } else {
                ".None"
            }
        } else if cfg!(target_os = "macos") {
            if downloadNAME == "VSCode" {
                "Visual Studio Code"
            } else {
                ".None"
            }
        } else {
            ".None"
        }
    };
    
    let filesInDownloads = fs::read_dir(&downloadsPATH).expect("the read_dir that sets filesInDownloads broke");
    let mut filePATH = ".None".to_string();
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
    if &filePATH[len-4..len-1] == "exe" ||
       &filePATH[len-3..len] == "deb" 
    {
        if cfg!(target_os = "linux") {
            Command::new("sudo").arg("apt").arg("-y").arg("install").arg("libgconf-2-4").arg("lib32stdc++6").arg("git").output().expect("failed to install libgconf-2-4 and git");
            Command::new("sudo").arg("dpkg").arg("-i").arg(&filePATH).output().expect("failed to install vscode");
        }else if cfg!(target_os = "windows") {
            Command::new("powershell.exe").arg("Start-Process").arg("-FilePath").arg(&filePATH).arg("-Wait").output().expect("failed to open exe");
        }
    }

    else if &filePATH[len-3..len] == "dmg" ||
        &filePATH[len-3..len] == "app" {        
        let mut volumePATH = ".None".to_string();
        let mut appPATH = ".None".to_string();

        if &filePATH[len-3..len] == "dmg" {
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
        if &filePATH[len-3..len] == "app" {
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

        if volumePATH != ".None" {
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
            Command::new(commandPATH).output().expect("failed to execute appimage");

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
            Command::new("sh").arg(&workingPATH).output().expect("failed to execute studio.sh");

        } else {
            return;
        }
    }
    //if mac, launch android studio to set path
    //install brew
}

fn clone_repo(downloadNAME: &str) {
    println!("cloning {} !>", &downloadNAME);
    
    let clonePATH = {
        if downloadNAME == "flutter" {
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

    if downloadNAME == "flutter" {
        fs::create_dir_all(&clonePATH).expect("failed to create SDK dir");
        env::set_current_dir(&clonePATH).expect("failed to set SDK dir as cwd");

        if cfg!(unix){
            Command::new("git").arg("clone").arg("https://github.com/flutter/flutter.git").output().expect("failed to clone flutter repo");
        } else {
            let output = Command::new("powershell.exe").arg("Start-Process").arg("-FilePath").arg("'C:\\Program Files\\Git\\bin\\git.exe'").arg("'clone https://github.com/flutter/flutter.git'").arg("-Wait").output().expect("failed to clone flutter repo");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
        return

    } else if downloadNAME == "co_demo1" {
        fs::create_dir_all(&clonePATH).expect("failed to create Code dir");
        env::set_current_dir(&clonePATH).expect("failed to set Code dir as cwd");

        if cfg!(unix){
            Command::new("git").arg("clone").arg("https://github.com/smokytheangel0/co_demo1.git").output().expect("failed to clone co_demo1 repo");
        } else {
            let output = Command::new("powershell.exe").arg("Start-Process").arg("-FilePath").arg("'C:\\Program Files\\Git\\bin\\git.exe'").arg("'clone https://github.com/smokytheangel0/co_demo1.git'").arg("-Wait").output().expect("failed to clone co_demo1 repo");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
        return

    } else {
        println!("{} is in the wrong function, it is in clone_repo(&downloadNAME)", &downloadNAME);
        return
    }
}

fn set_path() {
    println!("setting path !>");
    let homePATH = {
        if cfg!(unix){
            let path = dirs::home_dir().unwrap();
            let mut homePATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            homePATH += "/.bash_profile";
            homePATH
        } else {
            let path = dirs::home_dir().unwrap();
            let homePATH = path.to_str().unwrap().to_owned();
            homePATH
        }
    };

    #[cfg(unix)]
    {
        //need to create if none found
        //it returns a result to match
        let mut fileBOX = match OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&homePATH) {
                                Ok(val) => val,
                                Err(err) => OpenOptions::new().write(true).create_new(true).open(&homePATH).expect("could not create new bash_profile")
                            };

        if cfg!(target_os = "linux"){
            writeln!(fileBOX, "export ANDROID_HOME=$HOME/Android/Sdk").expect("failed to write linux android_home");
        } else if cfg!(target_os = "macos") {
            writeln!(fileBOX, "export ANDROID_HOME=$HOME/Library/Android/Sdk").expect("failed to write mac android_home");
        }
        writeln!(fileBOX, "export PATH=$HOME/Desktop/SDKs/flutter/bin:$PATH").expect("failed to write unix flutter path");
        writeln!(fileBOX, "export PATH=$ANDROID_HOME/tools:$PATH").expect("failed to write unix tools path");
        writeln!(fileBOX, "export PATH=$ANDROID_HOME/platform-tools:$PATH").expect("failed to write unix platform tools path");
        Command::new("sudo").arg(".").arg(&homePATH).output().expect("failed to refresh bash_profile");
    }

    #[cfg(windows)]
    {
        let addPATH = format!("{}\\Desktop\\SDKs\\flutter\\bin;{}\\AppData\\Local\\Android\\Sdk\\tools;{}\\AppData\\Local\\Android\\Sdk\\platform-tools;", &homePATH, &homePATH, &homePATH);
        let hklm = RegKey::predef(HKEY_CURRENT_USER);
        let environment = hklm.open_subkey("Environment").expect("could not open Environment key for flutter");
        let oldPATH: String = environment.get_value("Path").expect("could not open Path value for flutter");

        let mut cleanPATH: Vec<String> = vec![]; 
        let mut outPATH = "".to_string();
        if oldPATH.contains("%USERPROFILE%") {
            let pathVEC: Vec<&str> = oldPATH.split(";").collect();
            for path in &pathVEC {
                println!("{}", &path);
                let path = path.to_owned();
                let optBOX = path.rfind("%");
                let mut endINDEX: usize = 0;
                match optBOX {
                    Some(val) => endINDEX = val,
                    None => {cleanPATH.push(path.to_string()); continue}
                }
                endINDEX += 1;
                let mut outPATH = path.to_string();
                outPATH.replace_range(..endINDEX, &homePATH);
                cleanPATH.push(outPATH);          
            }
            outPATH = cleanPATH.join(";");
            outPATH = format!("'{}{}'", outPATH, addPATH);

        } else {
            outPATH = format!("'{};{}'", oldPATH, addPATH);
        }
        let androidPATH = format!("{}\\AppData\\Local\\Android\\Sdk;", &homePATH);

        Command::new("powershell.exe").arg("setx").arg("ANDROID_HOME").arg(&androidPATH).output().expect("failed to make android_home var");

        Command::new("powershell.exe").arg("set").arg("ANDROID_HOME").arg(&androidPATH).output().expect("failed to make android_home var");

        Command::new("powershell.exe").arg("setx").arg("Path").arg(&outPATH).output().expect("failed to set path");

        Command::new("powershell.exe").arg("set").arg("Path").arg(&outPATH).output().expect("failed to set path");
    }
}

fn git_install_complete() -> String {
    #[cfg(windows)]
    {
            //this works in win
            println!("\n");
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let environment = hklm.open_subkey("SOFTWARE\\GitForWindows");
            match environment {
                Result::Ok(val) => return "True".to_string(),
                Result::Err(err) => return "False".to_string()
            }
    }

    #[cfg(unix)]
    {
        println!("\n");
        let gitFOLDER = "/usr/bin/".to_owned();
        let programFOLDERS = fs::read_dir(&gitFOLDER).expect("No git app folder found");
        for folderNAME in programFOLDERS {
            let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                            .file_name()
                                            .into_string()
                                            .expect("the post string result which sets folderNAME has broken")
                                            .to_owned();
                if folderNAME == "git" {
                    return "True".to_owned()
                } else {
                    continue
                }
        }
        return "False".to_owned()
    }
}

fn android_install_complete() -> String {
        let androidFOLDER = {
            if cfg!(target_os = "windows"){
                let path = dirs::home_dir().unwrap();
                let mut androidFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();

                //might use path prefix to make this drive agnostic
                androidFOLDER += "\\AppData\\Local\\Android";
                androidFOLDER
            } else if cfg!(target_os = "macos") {
                let path = dirs::home_dir().unwrap();
                let mut androidFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();

                androidFOLDER += "/Library/Android";
                androidFOLDER

            }else {
                let path = dirs::home_dir().unwrap();
                let mut androidFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();
                androidFOLDER += "/Android";
                androidFOLDER
            }
        };

        let folderRESULT = fs::read_dir(&androidFOLDER);
        match folderRESULT {
            Ok(_val) => return "True".to_string(),
            Err(_err) => return "False".to_string()
        }
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

fn show_licences() {
    //show android licences
    //if mac show xcode licence

    //this and the run doctor functionality should be enclosed in a sh and ps1 script
    //this is because the paths no matter what we try, seem to never get refreshed properly
    let binPATH = {
        if cfg!(target_os = "windows"){
            let path = dirs::home_dir().unwrap();
            let mut binPATH = path.to_str()
                                .unwrap()
                                .to_owned();
            binPATH += "'\\Desktop\\SDKs\\flutter\\bin\\flutter.bat'";
            binPATH
        } else {
            let path = dirs::home_dir().unwrap();
            let mut binPATH = path.to_str()
                                .unwrap()
                                .to_owned();
            binPATH += "/Desktop/SDKs/flutter/bin/flutter";
            binPATH
        }
    };

    if cfg!(unix){
        Command::new("bash").arg(&binPATH).arg("doctor --android-licenses").spawn().expect("failed to run flutter doctor license command");
    } else {
        Command::new("powershell.exe").arg("Start-Process").arg("-FilePath").arg(&binPATH).arg("'doctor --android-licenses'").spawn().expect("failed to run flutter doctor license command");
    }
}

fn run_doctor() {
    println!("starting flutter for the first time !>");
    let binPATH = {
        if cfg!(target_os = "windows"){
            let path = dirs::home_dir().unwrap();
            let mut binPATH = path.to_str()
                                .unwrap()
                                .to_owned();
            binPATH += "\\Desktop\\SDKs\\flutter\\bin\\flutter.bat";
            binPATH = format!("'{}'", &binPATH);
            binPATH
        } else {
            let path = dirs::home_dir().unwrap();
            let mut binPATH = path.to_str()
                                .unwrap()
                                .to_owned();
            binPATH += "/Desktop/SDKs/flutter/bin/flutter";
            binPATH
        }
    };

    if cfg!(unix){
        Command::new("bash").arg(&binPATH).arg("doctor").output().expect("failed to run flutter doctor command");
    } else {
        Command::new("powershell.exe").arg("Start-Process").arg("-FilePath").arg(&binPATH).arg("'doctor'").arg("-Wait").output().expect("failed to run flutter command");
    }
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
// fileBOX at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This fileBOX may not be copied, modified, or distributed
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

    let _testPATH = "None".to_string();
    for downloadNAME in downloadMAP.clone().keys() {
        let answerBOX = download_complete(&downloadNAME, &_testPATH);

        if answerBOX == "True" {
            println!("{} is already complete !>\n", downloadNAME)
        } else {
            println!("{} has not yet been completed !>\n", downloadNAME)
        }

        downloadMAP.insert(downloadNAME.to_string(), answerBOX);
    }
    

    if cfg!(target_os = "windows") {
        println!("This is where we go over a few things first\nif you are using Edge browser,\n you must accept each download as it comes up\notherw the downloads should begin automatically\nplease check back with this terminal periodically \nto see if there are instructions that precede the next step\n\nfirst you need to close starUML as soon as it opens, ..>\nor we will wait for it to close ..>\n\nsecond, please close the VSCode window if it opens..>");
    } else if cfg!(target_os = "macos") {
        println!("This is where we go over a few things first\nthis process may seem too fast as it opens \na few tabs in your browser to download the items, \nthe android download you will have to select from the webpage, \nso keep an eye out for instructions in this terminal");
    } else if cfg!(target_os = "linux") {
        println!("This is where we go over a few things first\nif you are using Firefox browser, you must close the browser window \nafter each download has completed in order to start the next one\nplease check back with this terminal periodically \nto see if there are instructions that precede the next step");
    }

    println!("\nare you ready to start ?>");
    print!("y/N ?> ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut inBOX = String::new();
    std::io::stdin().read_line(&mut inBOX).expect("could not read the inBOX #>");
    println!("\n");

    if inBOX.to_lowercase().contains("y") {
        let now = time::Instant::now();
        let promptTIME = time::Duration::from_secs(150);

        'download: loop {
            for downloadNAME in downloadMAP.clone().keys() {
                if downloadMAP[downloadNAME] == "None" {

                    if downloadNAME == "android" {
                        println!("\nplease start the android-studio download \n if you are a windows user:\n select the blue link that ends with '.exe'\n\nif you are a mac user:\n select the blue link that ends with '.dmg'\n\nif you are an Ubuntu user:\n select the blue link that ends in 'linux.zip'\n")
                    } else if downloadNAME == "git" && cfg!(target_os = "linux") {
                        //skip git on linux
                        continue                
                    }else {
                        println!("starting {} download now!\n", downloadNAME);
                    }

                    start_downloads(&downloadNAME);
                    
                    println!("waiting for browser to download...\n");

                    //this whole thing could do with some cleanup
                    if downloadNAME == "android" {
                        let sleepTIME = time::Duration::from_secs(30);
                        thread::sleep(sleepTIME);
                    } else if downloadNAME == "git" && !cfg!(target_os = "linux") {
                        let sleepTIME = time::Duration::from_secs(20);
                        thread::sleep(sleepTIME);
                    } else {
                        let sleepTIME = time::Duration::from_secs(5);
                        thread::sleep(sleepTIME);
                    }
                    
                } else {
                    //if the key's value is True (already complete), skip
                    continue
                }
            }

            for downloadNAME in downloadMAP.clone().keys() {
                if downloadNAME == "git" && cfg!(target_os = "linux") {
                    //skip git on linux
                    downloadMAP.insert(downloadNAME.to_string(), "True".to_string());
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
                if downloadMAP[downloadNAME] == "True" {
                    completeNUM += 1;
                } else {
                    continue;
                }
            }

            if completeNUM == downloadMAP.keys().len() {
                println!("\nAll the downloads are complete !>\n");
                break 'download;

            } else if now.elapsed() > promptTIME {
                for downloadNAME in downloadMAP.clone().keys() {
                    if downloadMAP[downloadNAME] == "None" {
                        println!("The {} download has not started despite multiple attempts !>\n", downloadNAME.to_string())  
                    }
                }
            }
        }

        if cfg!(target_os = "linux"){
            extract_studio();
        }

        for downloadNAME in downloadMAP.clone().keys() {
            install_downloads(&downloadNAME);
        }

        if &android_install_complete() == "False" {
            if cfg!(target_os = "windows"){
                Command::new("powershell.exe").arg("Start-Process").arg("-FilePath")
                            .arg("'C:\\Program Files\\Android\\Android Studio\\bin\\studio64.exe'").arg("-Wait")
                            .output().expect("could not start android studio at the absolute path #>");

            } else if cfg!(target_os = "macos") {
                Command::new("open").arg("-a").arg("'Android Studio'")
                            .spawn().expect("could not start android studio at the absolute path #>");

            } else {
                let path = dirs::home_dir().unwrap();
                let mut sdkPATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
                sdkPATH += "/Desktop/SDKs//android-studio/bin/studio.sh";
                Command::new("bash").arg(&sdkPATH).output().expect("could not start android studio at the absolute path #>");
            }
        }

        while git_install_complete() == "False"{
            let sleepTIME = time::Duration::from_secs(20);
            thread::sleep(sleepTIME);
        }

        let cloneMAP: IndexMap<String, String> = [
            ("flutter".to_string(), "False".to_string()),
            ("co_demo1".to_string(),  "False".to_string()),
        ].iter().cloned().collect();

        for downloadNAME in cloneMAP.clone().keys() {
            clone_repo(&downloadNAME);
        }

        set_path();

        println!("install complete, please close this terminal and open a new one ..>\nthen type `flutter doctor --android-licenses` ")
    } else {
        panic!("you must accept to continue !>");
    }

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
            assert_eq!(currentPATH.contains("Desktop\\SDKs\\flutter\\bin;"), true)

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
    fn is_android_sdk_installed() {
        let androidFOLDER = {
            if cfg!(target_os = "windows"){
                let path = dirs::home_dir().unwrap();
                let mut androidFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();

                //might use path prefix to make this drive agnostic
                androidFOLDER += "\\AppData\\Local\\Android";
                androidFOLDER
            } else if cfg!(target_os = "macos") {
                let path = dirs::home_dir().unwrap();
                let mut androidFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();

                androidFOLDER += "/Library/Android";
                androidFOLDER

            }else {
                let path = dirs::home_dir().unwrap();
                let mut androidFOLDER = path.to_str()
                                    .unwrap()
                                    .to_owned();
                androidFOLDER += "/Android";
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
                if folderNAME.contains(&"Sdk"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            } else if cfg!(target_os = "macos") {
                if folderNAME.contains(&"Sdk"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }
            } else {
                if folderNAME.contains(&"Sdk"[..]) {
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
            assert_eq!(currentPATH.contains("AppData\\Local\\Android\\Sdk"), true);
            //tools on Path
            let hklm = RegKey::predef(HKEY_CURRENT_USER);
            let environment = hklm.open_subkey("Environment").expect("could not open Environment key for flutter");
            let currentPATH: String = environment.get_value("Path").expect("could not open Path value for flutter");
            assert_eq!(currentPATH.contains("AppData\\Local\\Android\\Sdk\\tools"), true);
            assert_eq!(currentPATH.contains("AppData\\Local\\Android\\Sdk\\platform-tools"), true);            

        }

    }

    #[test]
    #[ignore]
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
        //we need to do this via the registry in windows only
        //HKEY_LOCAL_MACHINE\SOFTWARE\GitForWindows match on result (err=> return false, ok()=> return true)
        #[cfg(windows)]
        {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let environment = hklm.open_subkey("SOFTWARE\\GitForWindows");
            match environment {
                Result::Ok(val) => assert!(true),
                Result::Err(err) => panic!("git was not found in the registry")
            }
        }
        #[cfg(unix)]
        {
            let gitFOLDER = "/usr/bin/".to_owned();
            let programFOLDERS = fs::read_dir(&gitFOLDER).expect("No git app folder found");
            for folderNAME in programFOLDERS {
                let folderNAME: String = folderNAME.expect("the pre string result which sets folderNAME has broken")
                                                .file_name()
                                                .into_string()
                                                .expect("the post string result which sets folderNAME has broken")
                                                .to_owned();
                    if folderNAME == &"git"[..] {
                        assert!(true);
                        return
                    } else {
                        continue
                    }
            }
            panic!("git binary was not found in /usr/bin!");
        }

    }

    #[test]
    fn is_vs_installed(){
        #[cfg(windows)]
        {
            //this false positives, no good for seeing if vscode is installed
            let hklm = RegKey::predef(HKEY_CLASSES_ROOT);
            let environment = hklm.open_subkey("VSCodeSourceFile");
            match environment {
                Result::Ok(val) => assert!(true),
                Result::Err(err) => panic!("vscode was not found in the registry")
            }
        }

        #[cfg(unix)]
        {
            let vsFOLDER = {
                if cfg!(target_os = "macos") {
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
                if cfg!(target_os = "macos") {
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

                if folderNAME.contains(&"co_demo1"[..]) {
                    assert_eq!(true, true);
                    return
                } else {
                    continue
                }

        }
        panic!("co_demo folder not found");

    }

    #[test]
    #[ignore]
    fn does_doctor_return_well(){
        assert_eq!(false, true);
    }

    
    #[test]
    #[ignore]
    fn main_(){
        //this test should include placing the folders from test in
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
            "co_demo1".to_string(), 
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
            "co_demo1".to_string(), 
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
            "co_demo1".to_string(), 
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
            "co_demo1".to_string(), 
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
            "co_demo1".to_string(), 
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
            "co_demo1".to_string(), 
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
            "co_demo1".to_string(), 
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
            "co_demo1".to_string(), 
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
            "co_demo1".to_string(), 
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
