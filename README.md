# Launchctl
Tiny Rust wrapper library for MacOS service launcher `launchctl`. This library
offers a more intuitive interface for managing services on MacOS. Other Rust
crates exist for interfacing with cross platform launch services. This library 
is specifically for MacOS. For more info about `launchctl` and `launchd` see
the [official apple docs](https://ss64.com/mac/launchctl.html).

### Install
```sh
cargo add launchctl
```
> [!NOTE]
> Coming soon is a CLI version which will make it easier to automate creating a service.

### Usage
The Service struct is the main entry point of this library. It uses a `bon` builder.
```rust
fn main() {
    // basic construction of a service
    let basic_service = Service::builder()
        .name("com.<owner name>.<binary name>")
        .build();

    // more advanced construction of a service
    let more_custom = Service::builder()
        .name("com.<owner name>.<binary name>")
        .
        .build();

    // create a .plist file for the service
    // ...

    basic.start().unwrap();
    custom.stop().unwrap();
}

```

### Limitations
Currently this crate does not support creating or modifying plist files. There
are other crates that can give you this behavior
[https://github.com/koenichiwa/launchd](https://github.com/koenichiwa/launchd), or you can hard code them as strings
which is what I prefer. 

Here is an example of how I do that in my [srhd](https://github.com/sylvanfranklin/srhd) crate.

```rs
pub fn install(ctl: &launchctl::Service) -> Result<(), Error> {
    let plist = format!(
"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple Computer//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
        <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
 	     <false/>
 	     <key>Crashed</key>
 	     <true/>
    </dict>
    <key>StandardOutPath</key>
    <string>/tmp/srhd_sylvanfranklin.out.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/srhd_sylvanfranklin.err.log</string>
    <key>ProcessType</key>
    <string>Interactive</string>
    <key>Nice</key>
    <integer>-20</integer>
</dict>
</plist>",
        ctl.name,
        ctl.bin_path.to_str().unwrap(),
    );

    Ok(fs::write(ctl.plist_path.clone(), plist)?)
}
```

# Contribution
Bro I love when people contribute or even submit issues. It's good for
everyone's career and understanding of everything, by all means open an issue or
a PR!

