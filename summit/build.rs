use anyhow::{anyhow, Error, Result};
use html_minifier::HTMLMinifierHelper;
use std::{
    env,
    fs::{self, File},
    io::Read,
    iter,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let debug = env::var_os("DEBUG_BUILD").map_or(false, |s| s == "1" || s == "true");
    let config = BuildConfig::new()?;
    log_with_debug(debug, format!("{config:?}"));
    rerun_if_change(debug, &config)?;
    println_with_debug(
        debug,
        format!(
            "cargo:rustc-env=TEMPLATES_DIR={}",
            // TEMPLATES_DIR is from the perspective of sailfish.toml, which currently resides in
            // the repo root.
            config.templates_dir().to_string_lossy()
        ),
    );
    if config.minify {
        minify_templates(debug, &config)?;
    }
    Ok(())
}
/// Indicate for cargo to rerun if any file in this project that are embedded in the binary are
/// changed. Eg, templates, css, js, etc.
fn rerun_if_change(debug: bool, paths: &BuildConfig) -> Result<()> {
    for tmpl_res in paths.list_src_templates() {
        let tmpl = tmpl_res?;
        println_with_debug(
            debug,
            format!("cargo:rerun-if-changed={}", tmpl.to_string_lossy()),
        );
    }
    Ok(())
}
#[derive(Debug)]
struct BuildConfig {
    // pub repo_dir: PathBuf,
    // pub out_dir: PathBuf,
    pub minify: bool,
    pub src_templates_dir: PathBuf,
    pub out_templates_dir: PathBuf,
}
impl BuildConfig {
    pub fn new() -> Result<Self> {
        let minify = !cfg!(debug_assertions);
        let repo_dir = Path::new("..").canonicalize()?;
        let out_dir =
            PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| anyhow!("OUT_DIR not specified"))?);
        let templates_dir = repo_dir.join("templates");
        let out_templates_dir = Path::new(&out_dir).join("templates");
        Ok(Self {
            minify,
            // repo_dir,
            // out_dir,
            src_templates_dir: templates_dir,
            out_templates_dir,
        })
    }
    /// The active templates dir. Either in the src, or minified out.
    pub fn templates_dir(&self) -> &Path {
        if self.minify {
            &self.out_templates_dir
        } else {
            &self.src_templates_dir
        }
    }
    /// List the files under this dir. Failures to list are propagated as an `Iterator::Item` error,
    /// for flat_map usage.
    fn list_dir(&self, path: &Path) -> Box<dyn Iterator<Item = Result<PathBuf>>> {
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(err) => return Box::new(iter::once(Err(Error::from(err)))),
        };
        let paths = entries
            .map(|entry_res| entry_res.map_err(Error::from))
            .map(|entry_res| entry_res.map(|entry| entry.path()));
        Box::new(paths)
    }
    /// List a path, if it's a dir it calls [`Self::list_dir`]. If it's a file it calls
    /// `iter::once(file)`. If it's an error it calls `iter::once(err)`.
    ///
    /// Useful for flat-mapping.
    fn list_path_res(
        &self,
        path_res: Result<PathBuf>,
    ) -> Box<dyn Iterator<Item = Result<PathBuf>>> {
        let path = match path_res {
            Ok(path) => path,
            Err(err) => return Box::new(iter::once(Err(err))),
        };
        if path.is_file() {
            Box::new(iter::once(Ok(path)))
        } else {
            self.list_dir(&path)
        }
    }
    fn list_files_recur(&self, path: &Path) -> impl Iterator<Item = Result<PathBuf>> + '_ {
        self.list_dir(path)
            .flat_map(|path_res| self.list_path_res(path_res))
    }
    pub fn list_src_templates(&self) -> impl Iterator<Item = Result<PathBuf>> + '_ {
        self.list_files_recur(self.src_templates_dir.as_path())
    }
    /// List both src and out templates.
    pub fn list_templates(&self) -> impl Iterator<Item = Result<SrcOutPaths>> + '_ {
        self.list_src_templates().map(|src_template_res| {
            src_template_res
                .map(|src| {
                    let rel = match src.strip_prefix(&self.src_templates_dir) {
                        Ok(rel) => rel,
                        Err(err) => return Err(Error::from(err)),
                    };
                    let out = self.out_templates_dir.join(rel);
                    Ok(SrcOutPaths { src, out })
                })
                // Need to flatten, since the Ok variant map above created a nested
                // Result<Result<_>>>.
                .map_or_else(Err, |res| res)
        })
    }
}
#[derive(Debug)]
struct SrcOutPaths {
    pub src: PathBuf,
    pub out: PathBuf,
}
fn minify_templates(_debug: bool, config: &BuildConfig) -> Result<()> {
    for res in config.list_templates() {
        let SrcOutPaths { src, out } = res?;
        // log_with_debug(debug, format!(""));
        let out_parent = out
            .parent()
            .ok_or_else(|| anyhow!("parent did not exist for: {}", out.to_string_lossy()))?;
        fs::create_dir_all(out_parent).expect("failed to create parent dir to template dst");
        let mut input_file = File::open(src)?;
        let mut output_file = File::create(out)?;
        let mut buffer = [0u8; 256];
        let mut html_minifier_helper = HTMLMinifierHelper::new();
        loop {
            let c = input_file.read(&mut buffer)?;
            if c == 0 {
                break;
            }
            html_minifier_helper.digest(&buffer[..c], &mut output_file)?;
        }
    }
    Ok(())
}
fn log_with_debug(debug: bool, s: impl AsRef<str>) {
    if debug {
        let s = s.as_ref();
        println!("cargo:warning={s}");
    }
}
// NIT: Might be worth making this into a macro to avoid string alloc? /shrug it's a build script.
fn println_with_debug(debug: bool, s: impl AsRef<str>) {
    let s = s.as_ref();
    log_with_debug(debug, s);
    println!("{s}");
}
