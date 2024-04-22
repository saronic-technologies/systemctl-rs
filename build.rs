fn main() {
    println!("cargo:rerun-if-env-changed=SYSTEMCTL_PATH");
}
