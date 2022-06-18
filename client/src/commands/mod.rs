mod de_select;
mod files;
mod info;
mod install;
mod search;
mod uninstall;

pub use {
    de_select::{deselect, select},
    files::files,
    info::info,
    install::install,
    search::search,
    uninstall::uninstall,
};
