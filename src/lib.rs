#[cfg_attr(feature = "svg", macro_use)]
pub extern crate glium;
pub extern crate gleam;

pub extern crate lazy_static;
pub extern crate euclid;
pub extern crate webrender;
pub extern crate rusttype;
pub extern crate app_units;
pub extern crate unicode_normalization;
pub extern crate tinyfiledialogs;
pub extern crate clipboard2;
pub extern crate font_loader;

#[cfg(feature = "logging")]
pub extern crate log;
#[cfg(feature = "svg")]
pub extern crate stb_truetype;
#[cfg(feature = "logging")]
pub extern crate fern;
#[cfg(feature = "logging")]
pub extern crate backtrace;
#[cfg(feature = "image_loading")]
pub extern crate image;
#[cfg(feature = "serde_serialization")]
#[cfg_attr(feature = "serde_serialization", macro_use)]
pub extern crate serde;
#[cfg(feature = "svg")]
pub extern crate lyon;
#[cfg(feature = "svg_parsing")]
pub extern crate usvg;
#[cfg(feature = "faster-hashing")]
pub extern crate twox_hash;

// #[cfg(not(target_os = "linux"))]
// extern crate nfd;