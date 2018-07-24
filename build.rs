extern crate pkg_config;

use pkg_config::Config;

fn main() {
    Config::new()
        .atleast_version("0.7.0")
        .probe("libcap-ng")
        .unwrap();
}
