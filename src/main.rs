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
use std::process::Command;
use std::fs;
use std::fs::ReadDir;

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
    //this works in Mac, Linux and Windows
    let mut outBOX = 0;

    //might definitely be a better way to do this
    let pathBuffer = env::current_dir().ok().unwrap();
    let pathBOX = pathBuffer.to_str().unwrap();

    let errorBOX = String::from("This program you've just run does not appear to be in the Downloads folder, please try running it again with it in the Downloads folder");
    
    if pathBOX.contains("Downloads") == false {
        println!("{}", errorBOX);
        outBOX += 1;
    }
    outBOX
    //this also needs to sys exit right here
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
///         gitURL = "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
///     elif platform.uname()[0] is "Linux":
///         vsVersion = "linux64_deb"
///         gitURL = "browser install currently only support Mac OS and Windows 10"
///     elif platform.uname()[0] is "Darwin":
///         vsVersion = "osx"
///         gitURL = "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
///     else:
///         vsVersion = "we currently only support Mac OS, Windows 10, and Linux"
///     testLIST[0] = vsVersion
///     testLIST[1] = gitURL
///
///     if fileBOX is "co_demo0-":
///         try:
///             webbrowser.open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
///         except:
///             print("there was an error opening the co_demo web page in your browser")
///     elif fileBOX is "flutter-":
///         try:
///             webbrowser.open("https://github.com/flutter/flutter/archive/master.zip")
///         except:
///             print("there was an error opening the flutter web page in your browser")
///     elif fileBOX is "VSCode-":
///         try:
///             webbrowser.open("https://code.visualstudio.com/docs/?dv={}"+vsVersion)
///         except:
///             print("there was an error opening the vs Code web page in your browser")
///     elif fileBOX is "git-" and platform.uname()[0] is not "Linux":
///         try:
///             webbrowser.open(gitURL)
///         except:
///             print("there was an error opening git in your browser")
///     elif fileBOX is "git-" and platform.uname()[0] is "Linux":
///         try:
///             print("your computer will ask for your password to install git")
///             os.system("sudo apt install git")
///         except:
///             print("there was an error installing git with apt")
///     elif fileBOX is "android-":
///         try:
///             webbrowser.open("https://developer.android.com/studio/#downloads")
///         except:
///             print("there was an error opening android studio in your web browser")
///     else:
///         testLIST[2] = "the switch branches have all been avoided"
///
///     return testLIST
///```
fn start_downloads(fileBOX: &str) -> Vec<String> {  
    //tests pass in linux, mac and windows

    //linux browser functionality strange unless user
    //already has a browser window open

    let mut testLIST = vec![
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
            "we currently only support Mac OS, Windows 10, and Linux"
        }
    };
    testLIST[0] = String::from(vsVersion);
    
    let gitURL: &str = {
        if cfg!(target_os = "windows") {
            "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
        } else if cfg!(target_os = "macos") {
            "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
        } else {
            "git browser install currently only support Mac OS and Windows 10"
        }
    };
    testLIST[1] = String::from(gitURL);
    
    if fileBOX == "co_demo0-" {
        webbrowser::open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
                    .expect("there was an error opening the co_demo web page in your browser");
        return testLIST;

    } else if fileBOX == "flutter-" {
        webbrowser::open("https://github.com/flutter/flutter/archive/master.zip")
                    .expect("there was an error opening the flutter web page in your browser");
        return testLIST;

    } else if fileBOX == "VSCode-" {
        let vsURL: String = format!("https://code.visualstudio.com/docs/?dv={}", vsVersion); 
        let vsURL: &str = &vsURL[..];
        webbrowser::open(&vsURL)
                    .expect("there was an error opening the vs Code web page in your browser");
        return testLIST;

    } else if fileBOX == "git-" && !cfg!(target_os = "linux") {
        webbrowser::open(gitURL)
                    .expect("there was an error opening git in your browser");
        return testLIST;

    } else if fileBOX == "git-" && cfg!(target_os = "linux") {
        println!("please enter your password to install git !>");
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

    } else if fileBOX == "android-" {
        webbrowser::open("https://developer.android.com/studio/#downloads")
                    .expect("there was an error opening the android studio web page in your browser");
        return testLIST;

    } else {
        testLIST[2] = "the switch branches have all been avoided !!!".to_string();
        return testLIST;
    }
    
}


///this is what the [is_complete] function is likely to be
///```python
/// [replace all 'is not' with '!=']
/// [replace all 'is' with '==']
/// #outBOX is String
/// def is_complete(fileBOX, completeNUM):
///     filesInDownloads = os.listdir('.')
///     for downloadNAME in filesInDownloads:
///         if fileBOX in downloadNAME or "crdownload" in downloadNAME:
///             if 'part' in downloadNAME:
///                 outBOX = False
///             elif 'partial'in downloadNAME:
///                 outBOX = False
///             elif 'crdownload' in downloadNAME:
///                 outBOX = False
///             else:
///                 outBOX = True
///             break
///         else:
///             outBOX = None
///     if outBOX is False:
///         print(fileBOX[:-1]+" is still transfering...")
///     elif outBOX is None:
///         print(fileBOX[:-1]+" still has not been started...")
///     return outBOX
/// ```
fn is_complete(fileBOX: &str, completeNUM: i16) -> String {
    //this sets the path to the downloads folder
    let downloadsPATH: String = {
        if cfg!(windows){
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "\\Downloads";
            downloadsPATH
        }else if cfg!(unix){
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "/Downloads";
            downloadsPATH
        } else {
            "we currently only support Windows 10, Ubuntu and Mac OS".to_string()
        }
    }; 

    //this sets the path to the test_data directory
    let testPATH: String = {
 
        if cfg!(windows){
            let path = env::home_dir().unwrap();
            let mut testPATH = path.to_str().unwrap().to_owned();
            if completeNUM == 5 {
                testPATH += "\\Desktop\\share\\test_data\\five_complete";
            } else {
                testPATH += "\\Desktop\\share\\test_data\\three_complete";
            }
            testPATH
        }else if cfg!(unix){
            let path = env::home_dir().unwrap();
            let mut testPATH = path.to_str().unwrap().to_owned();
            if completeNUM == 5 {
                testPATH += "/Desktop/share/test_data/five_complete";
            } else {
                testPATH += "/Desktop/share/test_data/three_complete";
            }
            testPATH
        } else {
            "we currently only support Windows 10, Ubuntu and Mac OS".to_string()
        }
    };    
   
    //this responds with an error which is why we didnt
    //but the unwrap() should panic...
    //despite that it actually unpacks into the proper string
    //down below...
    let filesInDownloads: ReadDir = {
        if cfg!(test) {
            fs::read_dir(&testPATH).unwrap()
        } else {            
            fs::read_dir(&downloadsPATH).unwrap()
        }
    };

    //how many unwraps can one rapper stack if
    //one rapper could stack unwraps delicately
    let outBOX: String = "None".to_string();
    for downloadBOX in filesInDownloads {
        let downloadNAME = &downloadBOX.unwrap()
                                        .file_name()
                                        .into_string()
                                        .unwrap();

        let found: String = {
            //this was an amazing oversight, that literally took a debugger to see was wrong
            //'in' || '.contains()' != '==', cos duh
            if downloadNAME.to_owned().contains(&fileBOX) || downloadNAME.to_owned().contains(&"crdownload"[..]) {

                if downloadNAME.to_owned().contains(&"part"[..]) {
                    return "False".to_string();
                } else if downloadNAME.to_owned().contains(&"partial"[..]) {
                    return "False".to_string();

                //we should do something special for crdownload,
                //it ought to count a int up to one when it sees a cr dl, but
                //keep going till it exhausts the for loop looking for a match
                //after the for loop is done with no match, it can check if there was a crdownload
                //and assume then that the file is in progress and change the None return
                //to a False
                } else if downloadNAME.to_owned().contains(&"crdownload"[..]) {
                    //unconfirmed += 1
                    //continue
                    return "False".to_string();

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
    return outBOX    
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
    let fileLIST = [
                    "git-".to_string(),
                    "co_demo0-".to_string(), 
                    "flutter-".to_string(),
                    "VSCode-".to_string(),
                    "android-".to_string()
                ];

    //this probably doesnt make sense anymore,
    //we couldnt get the internals of the [is_complete]
    //function working, so its a moot point
    /*
    for index in 0..fileLIST.len() {
        unsafe {
            let fileBOX = fileLIST.get_unchecked(index).to_string();
            loop {
                let answerBOX = is_complete(&fileBOX);
                if answerBOX == "None".to_string() {
                    start_downloads(&fileBOX);
                } else if answerBOX == "False".to_string() {
                    continue
                } else if answerBOX == "True".to_string() {
                    break
                }
            }
        }
    }
    */

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
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "\\Downloads";
            env::set_current_dir(&downloadsPATH);
        } else if cfg!(unix){
            let path = env::home_dir().unwrap();
            let mut downloadsPATH = path.to_str().unwrap().to_owned();
            downloadsPATH += "/Downloads";
            env::set_current_dir(&downloadsPATH);
        }else{
            panic!("we currently only support Windows 10, Mac OS and Linux");
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
            assert_eq!(start_downloads(&fileBOX)[0], "we currently only support Mac OS, Windows 10, and Linux")
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
            assert_eq!(start_downloads(&fileBOX)[1], "browser install currently only supports Mac OS, Windows 10")
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
                        "git-".to_string(),
                        "co_demo0-".to_string(), 
                        "flutter-".to_string(),
                        "VSCode-".to_string(),
                        "android-".to_string()
                    ];

        for index in 0..fileLIST.len() {
            unsafe {
                let fileBOX = fileLIST.get_unchecked(index).to_string();
                assert_eq!(start_downloads(&fileBOX)[2], "none");
            }
        }
    }
    */

    // start_downloads_linux_apt is at the bottom cos it brings up the sudo prompt

    #[test]
    fn is_complete_offline_switch() {
        //this should be where we check the test-data folders to ensure
        //the function gives a good canned result
        //later we will have an online version
        //which will try five times or something and clean up
        let fileLIST = [
                        "git-".to_string(),
                        "co_demo0-".to_string(), 
                        "flutter-".to_string(),
                        "VSCode-".to_string(),
                        "android-".to_string()
                    ];

        let mut testLIST: Vec<String> = [].to_vec();
        let completeNUM: i16 = 5;
        for index in 0..fileLIST.len() {
            unsafe {
                let fileBOX = fileLIST.get_unchecked(index).to_string();
                //is_complete(&fileBOX, completeNUM)
                let outBOX = is_complete(&fileBOX, completeNUM);
                testLIST.push(outBOX);
            }
        }
        
        assert_eq!(testLIST[0], "True");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "True");
        

        let mut testLIST: Vec<String> = [].to_vec();
        let completeNUM: i16 = 2;
        for index in 0..fileLIST.len() {
            unsafe {
                let fileBOX = fileLIST.get_unchecked(index).to_string();
                //is_complete(&fileBOX, completeNUM)
                let outBOX = is_complete(&fileBOX, completeNUM);
                testLIST.push(outBOX);
            }
        }

        //this is the last one, the VScode fails
        //because it detects the crdownload before it
        //detects the vscode installer, so it returns false
        //until all files are done
        assert_eq!(testLIST[0], "False");
        assert_eq!(testLIST[1], "True");
        assert_eq!(testLIST[2], "True");
        assert_eq!(testLIST[3], "True");
        assert_eq!(testLIST[4], "False");
        
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