mod de_select;
mod info;
mod install;
mod search;
mod files;

pub use {
    de_select::{deselect, select},
    info::info,
    install::install,
    search::search,
    files::files,
};
