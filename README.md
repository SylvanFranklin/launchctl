# Launchctl
Tiny Rust wrapper library for MacOS service launcher `launchctl`. This library
offers a more intuitive interface for managing services on MacOS. Other Rust
crates exist for interfacing with cross platform launch services, I just wanted
one that zeroed in on MacOS. For more info about `launchctl` and `launchd` see
the [official apple docs](https://ss64.com/mac/launchctl.html).

### Limitations
Currently this crate does not support creating or modifying plist files. There
are other crates that can give you this behavior, or you can hard code them as
strings. 


### Usage
(see docs.rs for more info)

```rust
fn main() {
    // basic construction of a service
    let basic = Service::new("com.<owner>.<binary>".to_string(), PathBuf::from("/bin/ls"));

    // more advanced construction of a service
    let custom = Service {
        name: "com.<owner>.<binary>".to_string(),
        domain_target: "gui/501".to_string(),
        service_target: "gui/501/com.<owner>.<binary>".to_string(),
        uid: "501".to_string(),
        bin_path: "/bin/ls".into(),
        plist_path: "/Library/LaunchAgents/com.<owner>.<binary>.plist".to_string(),
        error_log_path: "/tmp/<binary>_<user>.err.log".to_string(),
        out_log_path: "/tmp/<binary>_<user>.out.log".to_string(),
    };

    // create a .plist file for the service
    // ...

    basic.start().unwrap();
}

```

### Installation

```cargo add launchctl```




