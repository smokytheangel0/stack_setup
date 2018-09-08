# Copyright 2018 PacNGO
# 
# Licensed using a modified Apache License, Version 0.1.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License 
# 
# 	in the root directory of the source repository that first laid a commit on it. 
# 
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.


import os
import sys
#outBOX is int
def check_dirs():
    outBOX = 0
    pathBOX = os.getcwd()

    if "Downloads" not in pathBOX:
        outBOX += 1
        print("This program you've just run does not appear to be in the Downloads folder, please try running it again with it in the Downloads folder")
        return outBOX

import os
import platform
from pathlib import Path
TEST_FLAG = False
#outBOX is String
def is_complete(downloadNAME, testNUM):
    targetOS = platform.uname()[0]
    outBOX = "None"

    if targetOS == "Windows":
        downloadsPATH = str(Path.home())
        downloadsPATH += "\\Downloads"
        testPATH = str(Path.home())
        if testNUM == 5:
            testPATH += "\\Desktop\\share\\test_data\\five_complete"
        else:
            testPATH += "\\Desktop\\share\\test_data\\four_complete"

    elif targetOS == "Darwin" or targetOS == "Linux":
        downloadsPATH = str(Path.home())
        downloadsPATH += "/Downloads"
        testPATH = str(Path.home())
        if testNUM == 5:
            testPATH += "/Desktop/share/test_data/five_complete"
        else:
            testPATH += "/Desktop/share/test_data/four_complete"

    else:
        downloadsPATH = "we currently only support Windows 10, Ubuntu and Mac OS"

    if TEST_FLAG == True:
        filesInDowloads = os.listdir(testPATH)
    else:
        filesInDownloads = os.listdir(downloadsPATH)

    if targetOS == "Windows":
        if downloadNAME == "git":
            alternateGIT = "Git"
        else:
            alternateGIT = "None"
    else:
        alternateGIT = "None"
    
    if targetOS == "Linux":
        if downloadNAME == "VSCode":
            alternateCODE = "code"
        else:
            alternateCODE = "None"
    else:
        alternateCODE = "None"

    unconfirmed = 0
    for fileNAME in filesInDownloads:
        if downloadNAME in fileNAME or "crdownload" in fileNAME or str(alternateGIT) in fileNAME or str(alternateCODE) in fileNAME:
            if "part" in fileNAME:
                return False
            elif "partial" in fileNAME:
                return False
            elif "crdownload" in fileNAME:
                unconfirmed += 1
                continue
            else:
                return True
        
        else:
            found = "None"
        
        if found == "None":
            continue
        else:
            break

    if unconfirmed == 0:
        return outBOX
    else:
        return False

import platform
import webbrowser
import subprocess
#outBOX is vec[4] Strings
def start_downloads(downloadNAME):
    targetOS = platform.uname()[0]
    testLIST = [
        "None",
        "None",
        "None",
        "None",
        "None"
    ]

    if targetOS == "Windows":
        vsVersion = "win32"
        gitURL = "https://github.com/git-for-windows/git/releases/download/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
        umlVersion = "StarUML%20Setup%203.0.2.exe"
    elif targetOS == "Darwin":
        vsVersion = "osx"
        gitURL = "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/download?use_mirror=autoselect"
        umlVersion = "StarUML-3.0.2.dmg"
    elif targetOS == "Linux":
        vsVersion = "linux64_deb"
        gitURL = "git browser install currently only supports Mac OS and Windows 10"
        umlVersion = "StarUML-3.0.2-x86_64.AppImage"
    else:
        "we currently only support Mac OS, Windows 10, and Ubuntu"
    testLIST[0] = vsVersion
    testLIST[1] = gitURL
    testLIST[2] = umlVersion

    if downloadNAME == "StarUML":
        umlURL = "http://staruml.io/download/releases/" + umlVersion
        webbrowser.open(umlURL)
    elif downloadNAME == "co_demo0":
        webbrowser.open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
    elif downloadNAME == "flutter":
        webbrowser.open("https://github.com/flutter/flutter/archive/master.zip")
    elif downloadNAME == "VSCode":
        vsURL = "https://code.visualstudio.com/docs/?dv=" + vsVersion
        webbrowser.open(vsURL)
    elif downloadNAME == "git" and not targetOS == "linux":
        webbrowser.open(gitURL)
    elif downloadNAME == "git" and targetOS == "linux":
        returnBOX = subprocess.call(["sudo", "apt", "install", "git"], shell=True, check=True)
        if returnBOX == 0:
            testLIST[4] == "anything else"
        else:
            testLIST[4] == "E: Failed"
    elif downloadNAME == "android":
        webbrowser.open("https://developer.android.com/studio/#downloads")

    else:
        testLIST[3] = "the switch branches have all been avoided !!!"

    return testLIST
            

import platform
import time
def main():
    targetOS = platform.uname()[0]
    check_dirs()

    downloadMAP = {
        "StarUML": "None",
        "git": "None",
        "co_demo0": "None",
        "flutter": "None",
        "VSCode": "None",
        "android": "None",
    }

    testNUM = 0

    for downloadNAME in downloadMAP.keys():
        answerBOX = is_complete(downloadNAME, testNUM)

        if answerBOX == True:
            print("{0} is already complete!\n".format(downloadNAME))
        else:
            print("{0} has not yet been completed\n".format(downloadNAME))

        downloadMAP[downloadNAME] = answerBOX

    timeSTART = time.time()
    while True:
        for downloadNAME in downloadMAP.keys():
            if downloadMAP[downloadNAME] == "None":
                if downloadNAME == "android":
                    print("\nplease start the android-studio download \n if you are a windows user:\n select the blue link that ends with '.exe'\n\nif you are a mac user:\n select the blue link that ends with '.dmg'\n\nif you are an Ubuntu user:\n select the blue link that ends in 'linux.zip'\n")
                else:
                    print("starting {0} download now!\n".format(downloadNAME))

                testLIST = start_downloads(downloadNAME)

                print("waiting for browser to download...\n")

                if downloadNAME == "android":
                    time.sleep(20)
                elif downloadNAME == "git":
                    time.sleep(10)
                else:
                    time.sleep(5)

                if "E: Failed" not in testLIST[4] and "None" not in testLIST[4]:
                    downloadMAP[downloadNAME] = True
            else:
                continue

        for downloadNAME in downloadMAP.keys():
            if downloadNAME == "git" and not targetOS == "Linux":
                continue
            else:
                answerBOX = is_complete(downloadNAME, testNUM)
        
        completeNUM = 0
        for downloadNAME in downloadMAP.keys():
            if downloadMAP[downloadNAME] == True:
                completeNUM += 1
            else:
                continue
        
        if completeNUM == len(downloadMAP):
            print("\n\nall the downloads are complete!\n")
            break
        
        elif time.time() - timeSTART > 150:
            for downloadNAME in downloadMAP.keys():
                if downloadMAP[downloadNAME] == "None":
                    print("the {} download has not started despite multiple attempts\n".format(downloadNAME))
    
    time.sleep(60)

if __name__ == "__main__":
    main()
