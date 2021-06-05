#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod index;
mod upload;
mod download;
mod process;

fn main() {
    rocket::ignite().mount("/", routes![index::index, upload::upload,
        download::download, download::static_file, process::process]).launch();
}
