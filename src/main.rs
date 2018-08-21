use std::env;

fn check_dirs() -> i8 {
    //this works in Mac, Linux and Windows
    let mut outBOX: i8 = 0;
    let path = env::current_dir().ok().unwrap();
    let initialDirectory = path.to_str().unwrap();
    let errorBOX = String::from("This program you've just run does not appear to be in the Downloads folder, please try running it again with it in the Downloads folder");
    if !initialDirectory.contains("Downloads") {
        println!("{}", errorBOX);
        outBOX += 1;
    }
    outBOX
}

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
        ///this should test for slow user follow up, no user follow up, correct platform string format, crdownload, partial, part etc
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