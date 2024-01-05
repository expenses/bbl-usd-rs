use bbl_build::Config;

pub fn main() {
    let _dst = Config::new("openusd", "bbl-usd")
        .generator("Ninja")
        .profile("Release")
        .define("BBL_LANGUAGES", "rust")
        .define("CMAKE_PREFIX_PATH", "/home/ashley/projects/babble/install")
        .build();

    println!("cargo:rerun-if-changed=bbl-usd");
}
