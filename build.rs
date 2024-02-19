use bbl_build::Config;
use std::path::PathBuf;

pub fn main() {
    /*let _dst = Config::new("openusd", "bbl-usd")
        .define("VULKAN_SDK", std::env::var("VULKAN_SDK").unwrap())
        .define("BBL_LANGUAGES", "rust")
        .build();*/

    let header_file = std::env::var("BBL_USD_HEADER_FILE").unwrap();
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let library_dir = std::env::var("BBL_USD_LIBRARY_DIR").unwrap();
    let openusd_dir = std::env::var("OPENUSD_LIBRARY_DIR").unwrap();

    let usd_libs = [
        "usd_arch"
,"usd_ar"
,"usd_cameraUtil"
,"usd_garch"
,"usd_geomUtil"
,"usd_gf"
,"usd_glf"
,"usd_hdar"
,"usd_hdGp"
,"usd_hdsi"
,"usd_hd"
,"usd_hdSt"
,"usd_hdx"
,"usd_hf"
,"usd_hgiGL"
,"usd_hgiInterop"
,"usd_hgi"
,"usd_hgiVulkan"
,"usd_hio"
,"usd_js"
,"usd_kind"
,"usd_ndr"
,"usd_pcp"
,"usd_plug"
,"usd_pxOsd"
,"usd_sdf"
,"usd_sdr"
,"usd_tf"
,"usd_trace"
,"usd_ts"
,"usd_usdAppUtils"
,"usd_usdGeom"
,"usd_usdHydra"
,"usd_usdImagingGL"
,"usd_usdImaging"
,"usd_usdLux"
,"usd_usdMedia"
,"usd_usdPhysics"
,"usd_usdProcImaging"
,"usd_usdProc"
,"usd_usdRender"
,"usd_usdRiPxrImaging"
,"usd_usdRi"
,"usd_usdShade"
,"usd_usdSkelImaging"
,"usd_usdSkel"
,"usd_usd"
,"usd_usdUI"
,"usd_usdUtils"
,"usd_usdVolImaging"
,"usd_usdVol"
,"usd_vt"
,"usd_work"
    ];

    println!("cargo:rustc-link-lib=openusd-c");
    println!("cargo:rustc-link-search={library_dir}");

    println!("cargo:rustc-link-search={openusd_dir}");

    for lib in usd_libs {
        println!("cargo:rustc-link-lib={lib}");
    }

    println!("cargo:rustc-link-lib=stdc++");


    bindgen::builder()
        .header(header_file)
        .generate()
        .unwrap()
        .write_to_file(out_path.join("openusd.rs"))
        .unwrap();

    println!("cargo:rerun-if-changed=bbl-usd");
}
