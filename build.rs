#[cfg(feature = "fresh")]
extern crate bindgen;

use std::env;
#[cfg(feature = "fresh")]
use std::fs;
use std::path::{PathBuf};

fn main() {
  let mkl_dir = PathBuf::from(
      env::var("MKL_HOME")
        .or_else(|_| env::var("MKLROOT"))
        .unwrap_or_else(|_| "/usr/local".to_string())
  );

  #[cfg(all(feature = "mklml_gnu", feature = "mklml_intel"))]
  {
    panic!("enable only one of 'mklml_gnu' or 'mklml_intel'");
  }

  println!("cargo:rustc-link-search=native={}", mkl_dir.join("lib").display());

  #[cfg(feature = "mklml_gnu")]
  {
    if cfg!(target_os = "linux") {
      println!("cargo:rustc-link-lib=mklml_gnu");
    } else {
      unimplemented!();
    }
    println!("cargo:rustc-link-lib=gomp");
  }

  #[cfg(feature = "mklml_intel")]
  {
    if cfg!(target_os = "linux") {
      println!("cargo:rustc-link-lib=mklml_intel");
    } else if cfg!(target_os = "macos") {
      println!("cargo:rustc-link-lib=mklml");
    } else {
      unimplemented!();
    }
    println!("cargo:rustc-link-lib=iomp5");
  }

  #[cfg(feature = "fresh")]
  {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let gensrc_dir = manifest_dir.join("gensrc");
    fs::create_dir(&gensrc_dir).ok();

    println!("cargo:rerun-if-changed={}", gensrc_dir.join("_mkl_cblas.rs").display());
    fs::remove_file(gensrc_dir.join("_mkl_cblas.rs")).ok();
    bindgen::Builder::default()
      .clang_arg(format!("-I{}", mkl_dir.join("include").display()))
      .header("wrapped_mkl_cblas.h")
      .whitelist_type("CBLAS_LAYOUT")
      .whitelist_type("CBLAS_TRANSPOSE")
      .whitelist_type("CBLAS_UPLO")
      .whitelist_type("CBLAS_DIAG")
      .whitelist_type("CBLAS_SIDE")
      .whitelist_type("CBLAS_STORAGE")
      .whitelist_type("CBLAS_IDENTIFIER")
      .whitelist_type("CBLAS_OFFSET")
      .whitelist_type("CBLAS_ORDER")
      .whitelist_function("cblas_sdot")
      .whitelist_function("cblas_ddot")
      .whitelist_function("cblas_snrm2")
      .whitelist_function("cblas_dnrm2")
      .whitelist_function("cblas_saxpy")
      .whitelist_function("cblas_daxpy")
      .whitelist_function("cblas_sscal")
      .whitelist_function("cblas_dscal")
      .whitelist_function("cblas_sgemv")
      .whitelist_function("cblas_dgemv")
      .whitelist_function("cblas_sgemm")
      .whitelist_function("cblas_dgemm")
      .rustfmt_bindings(true)
      .generate()
      .expect("bindgen failed to generate mkl cblas bindings")
      .write_to_file(gensrc_dir.join("_mkl_cblas.rs"))
      .expect("bindgen failed to write mkl cblas bindings");
  }
}
