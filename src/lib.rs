#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
mod bin;
pub use egui_extras::install_image_loaders;