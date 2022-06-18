mod de_select;
mod info;
mod install;
mod uninstall;
mod search;
mod files;

pub use {
    de_select::{deselect, select},
    info::info,
    install::install,
    uninstall::uninstall,
    search::search,
    files::files,
};
