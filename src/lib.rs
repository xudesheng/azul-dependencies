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
pub extern crate xmlparser;
pub extern crate harfbuzz_sys;
// pub extern crate freetype_sys; // TODO: links = "freetype" crate!
// pub extern crate expat_sys; // TODO: links = "expat" crate!
// pub extern crate fontconfig_sys; // TODO: links = "fontconfig_sys" crate!

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
pub extern crate serde;
#[cfg(feature = "svg")]
pub extern crate lyon;
#[cfg(feature = "svg_parsing")]
pub extern crate usvg;
#[cfg(feature = "faster-hashing")]
pub extern crate twox_hash;

// Rust doesn't have the feature of re-exporting macros, so lazy_static! and log!
// have to be copy-pasted here, sadly.

// log-0.4.6/src/macros.rs -----------------------------------------------------

// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The standard logging macro.
///
/// This macro will generically log with the specified `Level` and `format!`
/// based argument list.
#[macro_export(local_inner_macros)]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= ::azul_dependencies::log::STATIC_MAX_LEVEL && lvl <= ::azul_dependencies::log::max_level() {
            ::azul_dependencies::log::__private_api_log(
                __log_format_args!($($arg)+),
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
            );
        }
    });
    ($lvl:expr, $($arg:tt)+) => (log!(target: __log_module_path!(), $lvl, $($arg)+))
}

/// Logs a message at the error level.
#[macro_export(local_inner_macros)]
macro_rules! error {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, ::azul_dependencies::log::Level::Error, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(::azul_dependencies::log::Level::Error, $($arg)*);
    )
}

/// Logs a message at the warn level.
#[macro_export(local_inner_macros)]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, ::azul_dependencies::log::Level::Warn, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(::azul_dependencies::log::Level::Warn, $($arg)*);
    )
}

/// Logs a message at the info level.
#[macro_export(local_inner_macros)]
macro_rules! info {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, ::azul_dependencies::log::Level::Info, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(::azul_dependencies::log::Level::Info, $($arg)*);
    )
}

/// Logs a message at the debug level.
#[macro_export(local_inner_macros)]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, ::azul_dependencies::log::Level::Debug, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(::azul_dependencies::log::Level::Debug, $($arg)*);
    )
}

/// Logs a message at the trace level.
#[macro_export(local_inner_macros)]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, ::azul_dependencies::log::Level::Trace, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!(::azul_dependencies::log::Level::Trace, $($arg)*);
    )
}

/// Determines if a message logged at the specified level in that module will
/// be logged.
///
/// This can be used to avoid expensive computation of log message arguments if
/// the message would be ignored anyway.
#[macro_export(local_inner_macros)]
macro_rules! log_enabled {
    (target: $target:expr, $lvl:expr) => {{
        let lvl = $lvl;
        lvl <= ::azul_dependencies::log::STATIC_MAX_LEVEL
            && lvl <= ::azul_dependencies::log::max_level()
            && ::azul_dependencies::log::__private_api_enabled(lvl, $target)
    }};
    ($lvl:expr) => {
        log_enabled!(target: __log_module_path!(), $lvl)
    };
}

// The log macro above cannot invoke format_args directly because it uses
// local_inner_macros. A format_args invocation there would resolve to
// `::azul_dependencies::log::format_args` which does not exist. Instead invoke format_args here
// outside of local_inner_macros so that it resolves (probably) to
// core::format_args or std::format_args. Same for the several macros that
// follow.
//
// This is a workaround until we drop support for pre-1.30 compilers. At that
// point we can remove use of local_inner_macros, usè `::azul_dependencies::log::` when invoking
// local macros, and invoke format_args directly.
#[doc(hidden)]
#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format_args!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_file {
    () => {
        file!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_line {
    () => {
        line!()
    };
}


// -- lazy_static-1.2.0/src/lib.rs ---------------------------------------------

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __lazy_static_internal {
    // optional visibility restrictions are wrapped in `()` to allow for
    // explicitly passing otherwise implicit information about private items
    ($(#[$attr:meta])* ($($vis:tt)*) static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        __lazy_static_internal!(@MAKE TY, $(#[$attr])*, ($($vis)*), $N);
        __lazy_static_internal!(@TAIL, $N : $T = $e);
        lazy_static!($($t)*);
    };
    (@TAIL, $N:ident : $T:ty = $e:expr) => {
        impl ::azul_dependencies::lazy_static::__Deref for $N {
            type Target = $T;
            fn deref(&self) -> &$T {
                #[inline(always)]
                fn __static_ref_initialize() -> $T { $e }

                #[inline(always)]
                fn __stability() -> &'static $T {
                    __lazy_static_create!(LAZY, $T);
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::azul_dependencies::lazy_static::LazyStatic for $N {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
    };
    // `vis` is wrapped in `()` to prevent parsing ambiguity
    (@MAKE TY, $(#[$attr:meta])*, ($($vis:tt)*), $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $(#[$attr])*
        $($vis)* struct $N {__private_field: ()}
        #[doc(hidden)]
        $($vis)* static $N: $N = $N {__private_field: ()};
    };
    () => ()
}

#[macro_export(local_inner_macros)]
macro_rules! lazy_static {
    ($(#[$attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        // use `()` to explicitly forward the information about private items
        __lazy_static_internal!($(#[$attr])* () static ref $N : $T = $e; $($t)*);
    };
    ($(#[$attr:meta])* pub static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        __lazy_static_internal!($(#[$attr])* (pub) static ref $N : $T = $e; $($t)*);
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        __lazy_static_internal!($(#[$attr])* (pub ($($vis)+)) static ref $N : $T = $e; $($t)*);
    };
    () => ()
}

#[macro_export]
#[doc(hidden)]
macro_rules! __lazy_static_create {
    ($NAME:ident, $T:ty) => {
        static $NAME: ::azul_dependencies::lazy_static::lazy::Lazy<$T> = ::azul_dependencies::lazy_static::lazy::Lazy::INIT;
    };
}

// glium-0.22.0/src/macros.rs --------------------------------------------------

/// Returns an implementation-defined type which implements the `Uniform` trait.
#[macro_export]
macro_rules! uniform {
    () => {
        ::azul_dependencies::glium::uniforms::EmptyUniforms
    };

    ($field:ident: $value:expr) => {
        ::azul_dependencies::glium::uniforms::UniformsStorage::new(stringify!($field), $value)
    };

    ($field1:ident: $value1:expr, $($field:ident: $value:expr),+) => {
        {
            let uniforms = ::azul_dependencies::glium::uniforms::UniformsStorage::new(stringify!($field1), $value1);
            $(
                let uniforms = uniforms.add(stringify!($field), $value);
            )+
            uniforms
        }
    };

    ($($field:ident: $value:expr),*,) => {
        uniform!($($field: $value),*)
    };
}

/// Implements the `glium::vertex::Vertex` trait for the given type.
///
/// The parameters must be the name of the struct and the names of its fields.
#[macro_export]
macro_rules! implement_vertex {
    ($struct_name:ident, $($field_name:ident),+) => (
        impl ::azul_dependencies::glium::vertex::Vertex for $struct_name {
            #[inline]
            fn build_bindings() -> ::azul_dependencies::glium::vertex::VertexFormat {
                use std::borrow::Cow;

                // TODO: use a &'static [] if possible

                Cow::Owned(vec![
                    $(
                        (
                            Cow::Borrowed(stringify!($field_name)),
                            {
                                // calculate the offset of the struct fields
                                let dummy: $struct_name = unsafe { ::std::mem::uninitialized() };
                                let offset: usize = {
                                    let dummy_ref = &dummy;
                                    let field_ref = &dummy.$field_name;
                                    (field_ref as *const _ as usize) - (dummy_ref as *const _ as usize)
                                };
                                // NOTE: `glium::vertex::Vertex` requires `$struct_name` to have `Copy` trait
                                // `Copy` excludes `Drop`, so we don't have to `std::mem::forget(dummy)`
                                offset
                            },
                            {
                                fn attr_type_of_val<T: ::azul_dependencies::glium::vertex::Attribute>(_: &T)
                                    -> ::azul_dependencies::glium::vertex::AttributeType
                                {
                                    <T as ::azul_dependencies::glium::vertex::Attribute>::get_type()
                                }
                                let dummy: &$struct_name = unsafe { ::std::mem::transmute(0usize) };
                                attr_type_of_val(&dummy.$field_name)
                            },
                            false
                        )
                    ),+
                ])
            }
        }
    );

    ($struct_name:ident, $($field_name:ident normalize($should_normalize:expr)),+) => {
        impl ::azul_dependencies::glium::vertex::Vertex for $struct_name {
            #[inline]
            fn build_bindings() -> ::azul_dependencies::glium::vertex::VertexFormat {
                use std::borrow::Cow;

                // TODO: use a &'static [] if possible

                Cow::Owned(vec![
                    $(
                        (
                            Cow::Borrowed(stringify!($field_name)),
                            {
                                let dummy: &$struct_name = unsafe { ::std::mem::transmute(0usize) };
                                let dummy_field = &dummy.$field_name;
                                let dummy_field: usize = unsafe { ::std::mem::transmute(dummy_field) };
                                dummy_field
                            },
                            {
                                fn attr_type_of_val<T: ::azul_dependencies::glium::vertex::Attribute>(_: &T)
                                    -> ::azul_dependencies::glium::vertex::AttributeType
                                {
                                    <T as ::azul_dependencies::glium::vertex::Attribute>::get_type()
                                }
                                let dummy: &$struct_name = unsafe { ::std::mem::transmute(0usize) };
                                attr_type_of_val(&dummy.$field_name)
                            },
                            {
                                $should_normalize
                            }
                        )
                    ),+
                ])
            }
        }
    };

    ($struct_name:ident, $($field_name:ident),+,) => (
        implement_vertex!($struct_name, $($field_name),+);
    );
}