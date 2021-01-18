use std::path::PathBuf;

pub fn probe_lua() -> PathBuf {
    #[cfg(feature = "picolua")]
    let artifacts = pico_lua_src::Build::new().build(pico_lua_src::PicoLua54);

    #[cfg(not(feature = "module"))]
    artifacts.print_cargo_metadata();

    artifacts.include_dir().to_owned()
}
