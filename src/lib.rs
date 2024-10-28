use std::{fs, io::Error, path::PathBuf, process::Command};

/// Wrapper for the launchctl system command to manage services
/// for more information about services on mac see https://ss64.com/mac/launchctl.html
#[derive(Debug)]
#[allow(dead_code)]
pub struct Service {
    /// Name of the service typically (com.<owner>.<bin name>)
    pub name: String,
    /// The target of the domain (gui/<uid>)
    pub domain_target: String,
    /// The target of the service (gui/<uid>/<service name>)
    pub service_target: String,
    /// id of logged in user typically 501
    pub uid: String,
    /// Path to the binary that will be launched
    pub bin_path: PathBuf,
    /// Path to the plist file typically ~/Library/LaunchAgents/<service name>.plist
    pub plist_path: String,
    /// Path to the error log file default (/tmp/<bin name>_<user>.err.log)
    pub error_log_path: String,
    /// Path to the out log file default (/tmp/<bin name>_<user>.out.log)
    pub out_log_path: String,
}

#[allow(dead_code)]
impl Service {
    /// Sets up all necessary variables for the service in a common use case
    /// for a more custom setup use construct the struct directly
    pub fn new(name: String, bin_path: PathBuf) -> Self {
        // for now saying that uid is always 501
        Service {
            domain_target: "gui/501".to_string(),
            service_target: format!("gui/501/{}", &name),
            uid: "501".to_string(),
            bin_path,
            plist_path: format!("/Library/LaunchAgents/{}.plist", &name),
            error_log_path: format!("/tmp/{}_{}.err.log", &name, &"501"),
            out_log_path: format!("/tmp/{}_{}.out.log", &name, &"501"),
            name,
        }
    }

    pub fn restart(&self) -> Result<(), Error> {
        self.stop()?;
        self.start()
    }

    /// Attemps to stop the service
    pub fn stop(&self) -> Result<(), Error> {
        if !self.is_bootstrapped() {
            // in this case we just try to kill the service just in case it is running
            // without being bootstrapped, it will fail silently if it is not running
            self.cmd()
                .args(vec!["kill", "SIGTERM", &self.service_target])
                .status()?;
        } else {
            // safely bootout the service
            self.cmd()
                .args(vec!["bootout", &self.domain_target, &self.plist_path])
                .status()?;
        }

        Ok(())
    }

    /// Attemps to start the service
    pub fn start(&self) -> Result<(), Error> {
        self.create_log_files()?;

        if !self.is_bootstrapped() {
            // first enable the service, then it can be bootstrapped
            self.cmd()
                .args(vec!["enable", &self.service_target])
                .status()?;
            self.cmd()
                .args(vec!["bootstrap", &self.domain_target, &self.plist_path])
                .status()?;
        } else {
            // since we already have the service bootstrapped we can just kickstart it
            self.cmd()
                .args(vec!["kickstart", &self.plist_path])
                .status()?;
        }

        Ok(())
    }
    fn cmd(&self) -> Command {
        // This makes an assumption that launchctl will always be in /bin
        // it also takes self in case this needs to be changed
        let mut command = Command::new("/bin/launchctl");
        command
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        return command;
    }

    /// checks if the log files exist, if not, creates them
    fn create_log_files(&self) -> Result<(), Error> {
        if !fs::metadata(&self.error_log_path).is_ok() {
            fs::write(&self.error_log_path, "")?;
        }

        if !fs::metadata(&self.out_log_path).is_ok() {
            fs::write(&self.out_log_path, "")?;
        }

        Ok(())
    }

    fn is_bootstrapped(&self) -> bool {
        self.cmd()
            .args(vec!["print", &self.service_target])
            .status()
            .unwrap_or_else(|_| panic!("Failed to check bootstrap for: {}", &self.name))
            .success()
    }
}
