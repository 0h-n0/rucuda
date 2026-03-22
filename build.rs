use std::env;
use std::path::PathBuf;

fn main() {
    let cuda_path = find_cuda_path();

    if let Some(ref path) = cuda_path {
        let lib_dir = if cfg!(target_os = "windows") {
            path.join("lib").join("x64")
        } else {
            let lib64 = path.join("lib64");
            if lib64.exists() {
                lib64
            } else {
                path.join("lib")
            }
        };

        println!("cargo:rustc-link-search=native={}", lib_dir.display());
    }

    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rerun-if-env-changed=CUDA_PATH");
    println!("cargo:rerun-if-env-changed=CUDA_HOME");
    println!("cargo:rerun-if-env-changed=CUDA_ROOT");
}

fn find_cuda_path() -> Option<PathBuf> {
    // Check environment variables
    for var in &["CUDA_PATH", "CUDA_HOME", "CUDA_ROOT"] {
        if let Ok(path) = env::var(var) {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
    }

    // Check standard locations
    let candidates = [
        "/usr/local/cuda",
        "/opt/cuda",
        "/usr/lib/cuda",
    ];

    for candidate in &candidates {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return Some(p);
        }
    }

    // nvcc in PATH - try to find CUDA from it
    // nvcc is typically at <cuda_path>/bin/nvcc
    if let Some(cuda_dir) = std::process::Command::new("which")
        .arg("nvcc")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let p = PathBuf::from(s);
            p.parent()?.parent().map(|d| d.to_path_buf())
        })
    {
        return Some(cuda_dir);
    }

    None
}
