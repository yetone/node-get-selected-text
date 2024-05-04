#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use get_selected_text::get_selected_text as _get_selected_text;

#[napi]
pub fn get_selected_text() -> String {
    _get_selected_text().unwrap_or_default()
}
