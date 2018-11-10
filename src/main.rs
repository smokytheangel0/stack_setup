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

#![allow(bad_style)]

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
/// RE: branching reduction
/// replace download folder branches with a single dirs::download_dir
/// 
/// RE: Versioning
/// After implementing the struct, add logic to update version string in struct upon startup
/// 

extern crate webbrowser;
extern crate indexmap;
use indexmap::IndexMap;
extern crate zip;
extern crate dirs;
extern crate rand;

use std::fs;
use std::io;
use std::env;
use std::fs::ReadDir;
use std::{thread, time};
use std::process::Command;
use std::io::prelude::*;

#[cfg(windows)]
    extern crate winreg;
#[cfg(windows)]
    use winreg::RegKey;
#[cfg(windows)]
    use winreg::enums::*;

/*
fn install_window_manager() {
    if cfg!(target_os = "linux") {
        Command::new("sudo").arg("apt").arg("install").arg("xdotool").status().expect("failed to install xdotool");
    }
}
*/

fn gather_unconfirmed() -> Vec<String> {
    let mut outVEC: Vec<String> = vec![];

    let pathBUFFER = dirs::download_dir().unwrap();
    let downloadsPATH: &str = pathBUFFER.to_str().unwrap();

    let filesInDownloads: ReadDir = {
        fs::read_dir(&downloadsPATH).expect("the read_dir that sets filesInDownloads broke")
    };

    for fileNAME in filesInDownloads {
        let fileNAME: String = fileNAME.expect("the pre string result which sets fileNAME has broken")
                                        .file_name()
                                        .into_string()
                                        .expect("the post string result which sets fileNAME has broken")
                                        .to_owned();

        if fileNAME.contains("Unconfirmed") {
            let fileNAME = fileNAME.clone();
            outVEC.push(fileNAME);
        }

    }
    outVEC

}

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
            "https://sourceforge.net/projects/git-osx-installer"
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
        if cfg!(target_os = "linux") {
            Command::new("xdg-open").arg(&umlURL).spawn().expect("failed to open with xdg");
        } else {
            webbrowser::open(&umlURL)
                        .expect("there was an error opening the star uml webpage in your browser");
        }
        return testLIST;

    } else if downloadNAME == "VSCode" {
        let vsURL: String = format!("https://code.visualstudio.com/docs/?dv={}", vsVersion); 
        let vsURL: &str = &vsURL[..];
        if cfg!(target_os = "linux") {
            Command::new("xdg-open").arg(&vsURL).spawn().expect("failed to open with xdg");
        } else {
            webbrowser::open(&vsURL)
                        .expect("there was an error opening the vs Code web page in your browser");
        }
        return testLIST;

    } else if downloadNAME == "git-" && !cfg!(target_os = "linux") {
        webbrowser::open(gitURL)
                    .expect("there was an error opening git in your browser");
        return testLIST;

    } else if downloadNAME == "android" {
        if cfg!(target_os = "linux") {
            Command::new("xdg-open").arg("https://developer.android.com/studio/#downloads").spawn().expect("failed to open with xdg");
        } else {
            webbrowser::open("https://developer.android.com/studio/#downloads")
                        .expect("there was an error opening the android studio web page in your browser");
        }
        return testLIST;

    } else {
        testLIST[3] = "the switch branches have all been avoided !!!".to_string();
        return testLIST;
    }
    
}

fn download_complete(downloadNAME: &str, testPATH: &str, unconfirmedLIST: &Vec<String>) -> String {
    let outBOX: String = "None".to_string();

    let downloadsPATH = dirs::download_dir().expect("failed to unwrap path");
    
    let filesInDownloads: ReadDir = {
        if cfg!(test) {
            //the directory returns err for
            //one_False_opera and all_True
            fs::read_dir(&testPATH).expect("the read_dir that sets filesInDownloads broke")
        } else {
            fs::read_dir(&downloadsPATH.as_path()).expect("the read_dir that sets filesInDownloads broke")
        }
    };

    let alternateGIT: &str = {
        if cfg!(target_os = "windows") {
            if downloadNAME == "git-" {
                "Git-"
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
    'search: for fileNAME in filesInDownloads {
        let fileNAME: String = fileNAME.expect("the pre string result which sets fileNAME has broken")
                                        .file_name()
                                        .into_string()
                                        .expect("the post string result which sets fileNAME has broken")
                                        .to_owned();
                                    
        for previousDL in unconfirmedLIST.clone() {
            if fileNAME.contains(&previousDL){
                continue 'search
            } else {
                continue
            }
        }

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
                            let filePATH: String = format!("{}{}", &downloadsPATH.to_str().unwrap(), &fileNAME);
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

/*
fn focus_terminal() {
    if cfg!(target_os = "linux"){
        Command::new("xdotool").arg("search").arg("--name").arg("~\\/Downloads").arg("windowraise").spawn().expect("unable to raise terminal");
        println!("{:?}", "~\\/Downloads" );
    } else if cfg!(target_os = "windows") {
        //this script may run in the background and not quit, whenever the bug that causes
        //the window not to focus happens
        if cfg!(debug_assertions){
            Command::new("powershell").arg("-ExecutionPolicy").arg("ByPass").arg("-File").arg("focus_terminal_debug.ps1").spawn().expect("failed to focus terminal");
        } else {
            Command::new("powershell").arg("-ExecutionPolicy").arg("ByPass").arg("-File").arg("focus_terminal_release.ps1").spawn().expect("failed to focus terminal");   
        }
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg("-a").arg("Terminal").output().expect("unable to raise terminal");
    }
}
*/

fn extract_studio() {
    println!("extracting android studio !>");

    let downloadNAME = "android".to_string();

    let pathBUFFER = dirs::download_dir().unwrap();
    let downloadsPATH: &str = pathBUFFER.to_str().unwrap();
    
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
    //this part is not simple, should be and use enums and possibly allow config of the else at the bottom for weird ones like android
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
            if downloadNAME == "git-" {
                "Git-"
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
            Command::new("sudo").arg("apt").arg("-y").arg("install").arg("libgconf-2-4").arg("lib32stdc++6").arg("git-").output().expect("failed to install libgconf-2-4 and git");
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

        if !downloadNAME.contains(&"git-"[..]) {

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
            Command::new("git-").arg("clone").arg("https://github.com/flutter/flutter.git").output().expect("failed to clone flutter repo");
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
            Command::new("git-").arg("clone").arg("https://github.com/smokytheangel0/co_demo1.git").output().expect("failed to clone co_demo1 repo");
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
        if cfg!(target_os = "macos"){
            let path = dirs::home_dir().unwrap();
            let mut homePATH = path.to_str()
                                        .unwrap()
                                        .to_owned();
            homePATH += "/.bash_profile";
            homePATH
        } else if cfg!(target_os = "linux") {
            let path = dirs::home_dir().unwrap();
            let mut homePATH = path.to_str()
                                    .unwrap()
                                    .to_owned();
            homePATH += "/.bashrc";
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
                                Err(_) => OpenOptions::new().write(true).create_new(true).open(&homePATH).expect("could not create new bash_profile")
                            };

        if cfg!(target_os = "linux"){
            writeln!(fileBOX, "export ANDROID_HOME=$HOME/Android/Sdk").expect("failed to write linux android_home");
        } else if cfg!(target_os = "macos") {
            writeln!(fileBOX, "export ANDROID_HOME=$HOME/Library/Android/Sdk").expect("failed to write mac android_home");
        }
        writeln!(fileBOX, "export PATH=$HOME/Desktop/SDKs/flutter/bin:$PATH").expect("failed to write unix flutter path");
        writeln!(fileBOX, "export PATH=$ANDROID_HOME/tools:$PATH").expect("failed to write unix tools path");
        writeln!(fileBOX, "export PATH=$ANDROID_HOME/platform-tools:$PATH").expect("failed to write unix platform tools path");
        let cmdPATH = format!("source {}", &homePATH);
        println!("{:?}", &cmdPATH);
        Command::new(&cmdPATH).spawn().expect("failed to refresh bash_profile");
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

fn git_install_complete() -> bool {
    #[cfg(windows)]
    {
        //this works in win
        println!("\n");
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let environment = hklm.open_subkey("SOFTWARE\\GitForWindows");
        match environment {
            Result::Ok(val) => return true,
            Result::Err(err) => return false
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
                if folderNAME == "git-" {
                    return true
                } else {
                    continue
                }
        }
        return false
    }
}

fn android_install_complete() -> bool {
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
        Ok(_val) => return true,
        Err(_err) => return false
    }
}


enum DownloadStatus {
    NotStarted, // None
    InProgress, // False
    Complete    // True
}

struct DownloadItem {
    name: String,
    DownloadStatus: DownloadStatus,
    url: String,
    waitTIME: time::Duration,
    winBIN: String,
    unixBIN: String,
    winPATH: String,
    macPATH: String,
    linPATH: String
}

fn main() {
    //install_window_manager();
    let unconfirmedLIST = gather_unconfirmed();
    let mut downloadMAP: IndexMap<String, String> = [
        ("StarUML".to_string(),  "None".to_string()),
        ("git-".to_string(),      "None".to_string()),
        ("VSCode".to_string(),   "None".to_string()),
        ("android".to_string(),  "None".to_string())    
    ].iter().cloned().collect();

    let _testPATH = "None".to_string();
    for downloadNAME in downloadMAP.clone().keys() {
        let answerBOX = download_complete(&downloadNAME, &_testPATH, &unconfirmedLIST);

        if answerBOX == "True" {
            println!("{} is already downloaded !>\n", downloadNAME)
        } else {
            println!("{} has not already been downloaded !>\n", downloadNAME)
        }

        downloadMAP.insert(downloadNAME.to_string(), answerBOX);
    }    

    if cfg!(target_os = "windows") {
        println!("This is where we go over a few things first\nif you are using Edge browser,\n you must save each download as it comes up\notherwise the downloads should begin automatically\nplease check back with this terminal periodically \nto see if there are instructions that precede the next step\n\nfirst you need to close starUML as soon as it opens, ..>\nor we will wait for it to close ..>\n\nsecond, please close the VSCode window if it opens..>");
    } else if cfg!(target_os = "macos") {
        println!("This is where we go over a few things first\nthis process may seem too fast as it opens \na few tabs in your browser to download the items, \nthe android download you will have to select from the webpage, \nso keep an eye out for instructions in this terminal");
    } else if cfg!(target_os = "linux") {
        println!("This is where we go over a few things first\nif you are using Firefox browser, please save each file instead of opening it\n please close the browser window after each download has completed in order to start the next one\nplease check back with this terminal periodically \nto see if there are instructions that precede the next step\n\nfirst you need to close starUML as soon as it opens, ..>\nor we will wait for it to close ..>");
    }

    println!("\nare you ready to start ?>");
    print!("y/N ?> ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut inBOX0 = String::new();
    std::io::stdin().read_line(&mut inBOX0).expect("could not read the inBOX #>");
    println!("\n");

    if inBOX0.to_lowercase().contains("y") {
        let start = time::Instant::now();
        let promptTIME = time::Duration::from_secs(150);

        'download: loop {
            for downloadNAME in downloadMAP.clone().keys() {
                if downloadMAP[downloadNAME] == "None" {

                    if downloadNAME == "android" {
                        println!("\nplease start the android-studio download \n if you are a windows user:\n select the blue link that ends with '.exe'\n\nif you are a mac user:\n select the blue link that ends with '.dmg'\n\nif you are an Ubuntu user:\n select the blue link that ends in 'linux.zip'\n");
                    } else if downloadNAME == "git-" && cfg!(target_os = "linux") {
                        //skip git on linux
                        continue                
                    }else {
                        println!("starting {} download now!\n", downloadNAME);
                    }

                    start_downloads(&downloadNAME);


                    println!("waiting for the {} download to start, please save if asked...\n", &downloadNAME);

                    //this should wait on the download to start
                    let mut answerBOX = "None".to_string();
                    while answerBOX == "None" {
                        let sleepTIME = time::Duration::from_secs(5);
                        thread::sleep(sleepTIME);
                        answerBOX = download_complete(&downloadNAME, &_testPATH, &unconfirmedLIST);

                        if answerBOX == "None" {
                            if downloadNAME == "android" || (cfg!(target_os = "macos") && downloadNAME == "git") {
                                let sleepTIME = time::Duration::from_secs(5);
                                thread::sleep(sleepTIME);
                                //focus_terminal();
                            }

                        }

                        let elapsedTIME = time::Instant::now() - start;
                        if elapsedTIME > promptTIME {
                            //focus_terminal();
                        }
                    }
                    
                } else {
                    //if the download key's value is True (already complete), skip
                    continue
                }
            }

            println!("waiting for the android download to complete !>");

            for downloadNAME in downloadMAP.clone().keys() {
                if downloadNAME == "git-" && cfg!(target_os = "linux") {
                    //skip git on linux
                    downloadMAP.insert(downloadNAME.to_string(), "True".to_string());
                    continue

                } else  {
                    let sleepTIME = time::Duration::from_secs(1);
                    thread::sleep(sleepTIME);

                    let answerBOX = download_complete(&downloadNAME, &_testPATH, &unconfirmedLIST);
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
            }
        }

        if cfg!(target_os = "linux"){
            extract_studio();
        }

        for downloadNAME in downloadMAP.clone().keys() {
            install_downloads(&downloadNAME);
        }

        if !android_install_complete() {
            println!("starting the android SDK installer !>");
            if cfg!(target_os = "windows"){
                Command::new("powershell.exe").arg("Start-Process").arg("-FilePath")
                            .arg("'C:\\Program Files\\Android\\Android Studio\\bin\\studio64.exe'").arg("-Wait")
                            .output().expect("could not start android studio at the absolute path #>");

            } else if cfg!(target_os = "macos") {
                //this runs sporadically or too late
                //let sleepTIME = time::Duration::from_secs(5);
                //thread::sleep(sleepTIME);

                Command::new("open").arg("-a").arg("Android Studio")
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

        while !git_install_complete() {
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
        //linux needs source before running doctor and after restart...
        println!("install complete, please close this terminal and open a new one ..>\nthen type `flutter doctor --android-licenses` ..>");
        let mut inBOX1 = String::new();
        std::io::stdin().read_line(&mut inBOX1).expect("could not read the inBOX #>");
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

    #[test]
    fn unconfirmed_download(){
        use rand::{thread_rng, Rng};

        let downloadPATH = dirs::download_dir().expect("failed to unwrap path");
        env::set_current_dir(&downloadPATH.as_path()).expect("failed to set current dir to downloads dir");
        
        let mut range = thread_rng();
        let randomNUM = range.gen_range(0, 1000000).to_string();
        
        let randomNAME = "Unconfirmed ".to_string()+&randomNUM+&".crdownload".to_string();

        let downloadFILE = fs::File::create(&randomNAME).expect("failed to create partial archive");
        downloadFILE.sync_all().expect("failed to solidify the Unconfirmed file");

        let unconfirmedLIST = gather_unconfirmed();
        assert!(unconfirmedLIST.contains(&randomNAME));

        //might be good to do some teardown here but it is not necessary
 
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
                Result::Ok(_) => assert!(true),
                Result::Err(_) => panic!("git was not found in the registry")
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
                    if folderNAME == &"git-"[..] {
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

}
