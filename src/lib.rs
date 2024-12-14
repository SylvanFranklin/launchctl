use bon::Builder;
use std::{fs, io::Error, process::Command};

#[cfg(test)]
mod tests;

/// Wrapper for the launchctl
/// for more information about services on mac see https://ss64.com/mac/launchctl.html
#[derive(Debug)]
#[allow(dead_code)]
#[derive(Builder)]
pub struct Service {
    /// Name of the service typically (com.<owner>.<bin name>)
    #[builder(into)]
    pub name: String,
    /// id of logged in user typically 501
    #[builder(into, default = "501")]
    pub uid: String,
    /// The target of the domain (gui/<uid>)
    #[builder(into, default = format!("gui/{}", uid))]
    pub domain_target: String,
    /// The target of the service (gui/<uid>/<service name>)
    #[builder(into, default = format!("{}/{}", domain_target, name))]
    pub service_target: String,
    /// Path to the plist file typically ~/Library/LaunchAgents/<service name>.plist
    #[builder(into, default = format!("~/Library/LaunchAgents/{}.plist", name))]
    pub plist_path: String,
    /// Path to the error log file default (/tmp/<bin name>_<user>.err.log)
    #[builder(into, default = format!("/tmp/{}_{}.err.log", name, uid))]
    pub error_log_path: String,
    /// Path to the out log file default (/tmp/<bin name>_<user>.out.log)
    #[builder(into, default = format!("/tmp/{}_{}.out.log", name, uid))]
    pub out_log_path: String,
}

#[allow(dead_code)]
impl Service {
    /// Effectively the same as calling start and then stop
    pub fn restart(&self) -> Result<(), Error> {
        self.stop()?;
        self.start()
    }

    /// Attempts to stop the service
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

    /// Attempts to start the service
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
