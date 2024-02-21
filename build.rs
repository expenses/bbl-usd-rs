use std::path::PathBuf;

pub fn main() {
    let dst = cmake::Config::new("bbl-usd")
        .define("VULKAN_SDK", std::env::var("VULKAN_SDK").unwrap())
        .define("BBL_LANGUAGES", "rust")
        .build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=openusd-c");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-search=native=/nix/store/k5104fk6ibg8706m6f5qda9i0x8glwb1-openusd-minimal/lib");

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

    println!("cargo:rustc-link-search=native=/nix/store/zvccb8sg75v33xv772rd394dghavlqsh-tbb-2021.8.0/lib");
    println!("cargo:rustc-link-lib=static=tbb");

    println!("cargo:rustc-link-search=native=/nix/store/gqmi81pfwdqabmjpsrsclhrkydczw99n-opensubdiv-3.5.1-static/lib");
    println!("cargo:rustc-link-lib=static=osdCPU");
    println!("cargo:rustc-link-lib=static=osdGPU");

    println!("cargo:rustc-link-search=native=/nix/store/9xpx0mi1lxld17x4q2k065r8966c878v-shaderc-2023.8-static/lib");
    println!("cargo:rustc-link-lib=static=shaderc_combined");

    println!("cargo:rustc-link-lib=X11");

    println!("cargo:rustc-link-search=native=/nix/store/9l8785vc3w2jbmcvjw4gkgrszrkg103z-libGL-1.7.0/lib");
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
