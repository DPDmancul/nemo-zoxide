fn main() {
    pkg_config::Config::new().probe("libnemo-extension").unwrap();
}
