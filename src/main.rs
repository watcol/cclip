extern crate atty;
extern crate copypasta;
extern crate copypasta_ext;
extern crate which;

use std::env::{args, var};
use std::error::Error;
use std::io::{stdin, Read};

use copypasta::{ClipboardContext, ClipboardProvider};
use copypasta_ext::x11_bin::X11BinClipboardContext;

struct Clipboard(Box<dyn ClipboardProvider>);

impl Clipboard {
    fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        if var("XDG_SESSION_TYPE") == Ok("x11".to_string())
            && (which::which("xclip").is_ok() || which::which("xsel").is_ok())
        {
            Ok(Self(Box::new(X11BinClipboardContext::new_with_x11()?)))
        } else {
            Ok(Self(Box::new(ClipboardContext::new()?)))
        }
    }

    fn store<T: Into<String>>(&mut self, text: T) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.0.set_contents(text.into())
    }

    fn load(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        self.0.get_contents()
    }
}

fn inner_main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut clip = Clipboard::new()?;

    match args().nth(1) {
        Some(text) => {
            clip.store(text)?;
        }
        None if atty::isnt(atty::Stream::Stdin) => {
            let mut text = String::new();
            stdin().read_to_string(&mut text)?;
            clip.store(text)?;
        }
        None => {
            println!("{}", clip.load()?);
        }
    }

    Ok(())
}

fn main() {
    inner_main().unwrap_or_else(|e| {
        println!("ERROR: {}", e);
        std::process::exit(1);
    })
}
