//! Template project for Flipper Zero.
//! This app prints "Hello, Rust!" to the console then exits.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;
// Required for allocator
extern crate flipperzero_alloc;

use core::ffi::CStr;

use flipperzero::println;
use flipperzero_rt::{entry, manifest};

use flipperzero::{
    dialogs::{self, DialogMessage, DialogMessageButton, DialogsApp},
    gui::canvas::Align,
};

// Define the FAP Manifest for this application
manifest!(
    name = "Flipper Zero Rust",
    app_version = 1,
    has_icon = true,
    // See https://github.com/flipperzero-rs/flipperzero/blob/v0.11.0/docs/icons.md for icon format
    icon = "mtechMono.icon",
);

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    // To customize the dialog, use the DialogMessage API:
    let mut dialogs = DialogsApp::open();
    let mut message = DialogMessage::new();

    message.set_header(c"Make your move!", 0, 0, Align::Left, Align::Top);
    message.set_text(
        c"Choose one of the following:",
        0,
        26,
        Align::Left,
        Align::Top,
    );
    message.set_buttons(Some(c"Rock"), Some(c"Paper"), Some(c"Scissor"));

    let button = dialogs.show_message(&message);

    // ... or use dialog::alert() to display a simple message:
    match button {
        DialogMessageButton::Left => dialogs::alert("You chose Rock..."),
        DialogMessageButton::Center => dialogs::alert("You chose Paper..."),
        DialogMessageButton::Right => dialogs::alert("You chose Scissors..."),
        DialogMessageButton::Back => dialogs::alert("You chose not to play..."),
    }

    dialogs::alert("... but dolphins can't play rock paper scissors anyways :)");

    0
}
