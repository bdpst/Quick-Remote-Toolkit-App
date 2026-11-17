use std::{env, fs, path::PathBuf, process::Command};

fn resource_compiler() -> PathBuf {
    if let Some(path) = env::var_os("QRT_RC") {
        return path.into();
    }
    if let Some(root) = env::var_os("ProgramFiles(x86)") {
        let root = PathBuf::from(root).join("Windows Kits/10/bin");
        let mut kits: Vec<_> = fs::read_dir(root)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let version: Option<Vec<u32>> = entry
                    .file_name()
                    .to_string_lossy()
                    .split('.')
                    .map(|s| s.parse().ok())
                    .collect();
                let compiler = entry.path().join("x64/rc.exe");
                version
                    .filter(|_| compiler.is_file())
                    .map(|v| (v, compiler))
            })
            .collect();
        kits.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some((_, compiler)) = kits.pop() {
            return compiler;
        }
    }
    "rc.exe".into()
}
fn quoted_path(path: PathBuf) -> String {
    path.to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}
fn main() {
    println!("cargo::rerun-if-changed=assets/QuickRemoteToolkit.ico");
    println!("cargo::rerun-if-changed=assets/application.manifest");
    println!("cargo::rerun-if-env-changed=QRT_RC");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Cargo manifest directory"));
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo output directory"));
    let resource = out.join("application.res");
    let script = out.join("application.rc");
    let version = env::var("CARGO_PKG_VERSION").expect("Cargo version");
    let numeric_version = format!("{},0", version.replace('.', ","));
    let contents = format!(
        r#"#pragma code_page(65001)
1 ICON "{}"
1 24 "{}"
1 VERSIONINFO
 FILEVERSION {numeric_version}
 PRODUCTVERSION {numeric_version}
 FILEFLAGSMASK 0x3fL
 FILEFLAGS 0
 FILEOS 0x40004L
 FILETYPE 0x1L
BEGIN
 BLOCK "StringFileInfo"
 BEGIN
  BLOCK "040904b0"
  BEGIN
   VALUE "CompanyName", "bdpst\0"
   VALUE "FileDescription", "Quick Remote Toolkit\0"
   VALUE "FileVersion", "{version}\0"
   VALUE "InternalName", "QuickRemoteToolkitRust\0"
   VALUE "OriginalFilename", "QuickRemoteToolkitRust.exe\0"
   VALUE "ProductName", "Quick Remote Toolkit\0"
   VALUE "ProductVersion", "{version}\0"
  END
 END
 BLOCK "VarFileInfo"
 BEGIN
  VALUE "Translation", 0x409, 1200
 END
END
"#,
        quoted_path(root.join("assets/QuickRemoteToolkit.ico")),
        quoted_path(root.join("assets/application.manifest"))
    );
    fs::write(&script, contents).expect("Write resource script");
    let status = Command::new(resource_compiler())
        .arg("/nologo")
        .arg("/fo")
        .arg(&resource)
        .arg(&script)
        .status()
        .expect("Windows SDK rc.exe required; set QRT_RC to its full path if necessary");
    assert!(status.success(), "Windows resource compilation failed");
    println!(
        "cargo::rustc-link-arg-bin=QuickRemoteToolkitRust={}",
        resource.display()
    );
}
