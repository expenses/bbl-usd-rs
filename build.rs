use std::path::PathBuf;

pub fn main() {
    let var = |name| {
        std::env::var(name).expect(name)
    };

    let dst = cmake::Config::new("bbl-usd")
        .define("VULKAN_SDK", var("VULKAN_SDK"))
        .define("BBL_LANGUAGES", "rust")
        .build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=openusd-c");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-search=native={}", var("OPENUSD_LIB_DIR"));

    let libs = [
        "usd_ar",
        "usd_hdGp",
        "usd_hio",
        "usd_tf",
        "usd_usdLux",
        "usd_usdSkel",
        "usd_arch",
        "usd_hdsi",
        "usd_js",
        "usd_trace",
        "usd_usdMedia",
        "usd_usdSkelImaging",
        "usd_cameraUtil",
        "usd_hdSt",
        "usd_kind",
        "usd_ts",
        "usd_usdPhysics",
        "usd_usdUI",
        "usd_garch",
        "usd_hdx",
        "usd_ndr",
        "usd_usd",
        "usd_usdProc",
        "usd_usdUtils",
        "usd_geomUtil",
        "usd_hf",
        "usd_pcp",
        "usd_usdAppUtils",
        "usd_usdProcImaging",
        "usd_usdVol",
        "usd_gf",
        "usd_hgi",
        "usd_plug",
        "usd_usdGeom",
        "usd_usdRender",
        "usd_usdVolImaging",
        "usd_glf",
        "usd_hgiGL",
        "usd_pxOsd",
        "usd_usdHydra",
        "usd_usdRi",
        "usd_vt",
        "usd_hd",
        "usd_hgiInterop",
        "usd_sdf",
        "usd_usdImaging",
        "usd_usdRiPxrImaging",
        "usd_work",
        "usd_hdar",
        "usd_hgiVulkan",
        "usd_sdr",
        "usd_usdImagingGL",
        "usd_usdShade",
    ];

    for lib in libs {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    println!("cargo:rustc-link-search=native={}", var("TBB_LIB_DIR"));
    println!("cargo:rustc-link-lib=static=tbb");

    println!("cargo:rustc-link-search=native={}", var("OPENSUBDIR_LIB_DIR"));
    println!("cargo:rustc-link-lib=static=osdCPU");
    println!("cargo:rustc-link-lib=static=osdGPU");

    println!("cargo:rustc-link-search=native={}", var("SHADERC_LIB_DIR"));
    println!("cargo:rustc-link-lib=static=shaderc_combined");

    println!("cargo:rustc-link-lib=X11");

    println!("cargo:rustc-link-search=native={}", var("OPENGL_LIB_DIR"));
    println!("cargo:rustc-link-lib=GLX");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindgen::builder()
        .header(out_path.join("build/openusd-c.h").to_str().unwrap())
        .generate()
        .unwrap()
        .write_to_file(out_path.join("build/openusd.rs"))
        .unwrap();

    println!("cargo:rerun-if-changed=bbl-usd");
}
