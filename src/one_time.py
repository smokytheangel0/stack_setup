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


import sys
import platform
import webbrowser
def start_downloads(fileBOX):
    testLIST = [
                None,
                None,
                None,
                None
               ]

    if platform.uname()[0] == "Windows":
        vsVersion = "win32"
        gitURL = "https://github.com/git-for-windows/git/releases/downloadNAME/v2.18.0.windows.1/Git-2.18.0-64-bit.exe"
    elif platform.uname()[0] == "Linux":
        vsVersion = "linux64_deb"
        gitURL = "browser install currently only support Mac OS and Windows 10"
    elif platform.uname()[0] == "Darwin":
        vsVersion = "osx"
        gitURL = "https://sourceforge.net/projects/git-osx-installer/files/git-2.18.0-intel-universal-mavericks.dmg/downloadNAME?use_mirror=autoselect"
    else:
        vsVersion = "we currently only support Mac OS, Windows 10, and Linux"
    testLIST[0] = vsVersion
    testLIST[1] = gitURL

    if fileBOX == "co_demo0-":
        try:
            webbrowser.open("https://github.com/smokytheangel0/co_demo0/archive/master.zip")
        except:
            print("there was an error opening the co_demo web page in your browser")
    elif fileBOX == "flutter-":
        try:
            webbrowser.open("https://github.com/flutter/flutter/archive/master.zip")
        except:
            print("there was an error opening the flutter web page in your browser")
    elif fileBOX == "VSCode-":
        try:
            webbrowser.open("https://code.visualstudio.com/docs/?dv="+vsVersion)
        except:
            print("there was an error opening the vs Code web page in your browser")
    elif fileBOX == "git-" and platform.uname()[0] != "Linux":
        try:
            webbrowser.open(gitURL)
        except:
            print("there was an error opening git in your browser")
    elif fileBOX == "git-" and platform.uname()[0] == "Linux":
        try:
            print("your computer will ask for your password to install git")
            os.system("sudo apt install git")
        except:
            print("there was an error installing git with apt")
    elif fileBOX == "android":
        try:
            webbrowser.open("https://developer.android.com/studio/#downloads")
        except:
            print("there was an error opening android studio in your web browser")
    else:
        testLIST[2] = "the switch branches have all been avoided"
        print(testLIST[2])


import os
import time
#outBOX == bool
def is_complete(fileBOX):
    filesInDirectory = os.listdir('.')
    for downloadNAME in filesInDirectory:
        if fileBOX in downloadNAME or "crdownload" in downloadNAME:
            if 'part' in downloadNAME:
                outBOX = False
            elif 'partial'in downloadNAME:
                outBOX = False
            elif 'crdownload' in downloadNAME:
                outBOX = False
            else:
                outBOX = True
            break
        else:
            outBOX = None
    if outBOX == False:
        print(fileBOX[:-1]+" is still transfering...")
    elif outBOX == None:
        print(fileBOX[:-1]+" still has not been started...")
    return outBOX

def main():
    filesNeeded = {
                'co_demo0-' : None, 
                'flutter-': None, 
                'VSCode-': None, 
                'git-': None, 
                'android-': None
            }
    androidTimeOut = 0
    filesInDirectory = [None]
    while filesInDirectory == [None]:
        filesInDirectory = os.listdir('.')
    else:
        while all(filesNeeded.values()) != True:
            for fileBOX in filesNeeded.keys():
                done = False
                #while done is None or False, check it
                while done != True:
                    done = is_complete(fileBOX)
                #if done is True, say complete
                else:
                    #if done is True, say complete
                    if done == True and filesNeeded[fileBOX] != True:
                        print(fileBOX[:-1]+' completed!')
                        filesNeeded[fileBOX] = True
                    elif filesNeeded[fileBOX] == True:
                        pass
                    #if done is False,
                    else:
                        #check if it is android
                        if fileBOX == 'android-studio-ide':
                            #if this is the first time it hasnt been started
                            if androidTimeOut == 0:
                                #open the downloadNAME
                                start_downloads(fileBOX)
                                filesNeeded[fileBOX] = False
                                androidTimeOut += 1
                            else:
                                #if this is the second time around,
                                #warn the user
                                if androidTimeOut == 1:                                    
                                    print("please start the android downloadNAME in your browser")
                                    androidTimeOut += 1
                                #if the user has been warned, ignore until complete
                                else:
                                    pass
                        else:
                            start_downloads(fileBOX)
                            filesNeeded[fileBOX] = False

if __name__ == "__main__":
    main()
