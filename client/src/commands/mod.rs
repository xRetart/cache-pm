mod de_select;
mod files;
mod info;
mod install;
mod list;
mod uninstall;

pub use {
    de_select::{deselect, select},
    files::files,
    info::info,
    install::install,
    list::list,
    uninstall::uninstall,
};
