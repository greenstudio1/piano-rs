extern crate rustbox;
extern crate rodio;

use std::default::Default;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

use rustbox::{Color, RustBox};
use rustbox::Key;

/*
█▒
*/

fn print_whitekeys(rustbox: &RustBox) {
    for y in 0..16 {
        // last border is lonely
        rustbox.print(156, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
        for x in 0..52 {
            let k = x*3;
            rustbox.print(k, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
            rustbox.print(k+1, y, rustbox::RB_BOLD, Color::White, Color::Black, "██");
        }
    }
}


fn print_blackkeys(rustbox: &RustBox) {
    for y in 0..9 {
        // first black key is lonely
        rustbox.print(3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

        for x in 0..7 {
            let g1k1 = x*21 + 9;
            let g1k2 = g1k1 + 3;
            rustbox.print(g1k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.print(g1k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

            let g2k1 = g1k2 + 6;
            let g2k2 = g2k1 + 3;
            let g2k3 = g2k2 + 3;
            rustbox.print(g2k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.print(g2k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.print(g2k3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
        }

    }
}


fn play_note(note: &str, endpoint: &rodio::Endpoint) -> Result<rodio::Sink,
                                                        rodio::decoder::DecoderError> {
    let file_path = format!("assets/{}.ogg", note);
    let file = std::fs::File::open(file_path).unwrap();
    rodio::play_once(endpoint, BufReader::new(file))
}


fn make_mark(x: usize, y: usize, rustbox: &RustBox) {
    rustbox.print(x, y, rustbox::RB_BOLD, Color::Black, Color::White, "▒▒");
    thread::spawn(move || {
        let delay = Duration::from_millis(2000);
        thread::sleep(delay);
        rustbox.print(x, y, rustbox::RB_BOLD, Color::White, Color::Black, "██");
    });
}


fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let endpoint = rodio::get_default_endpoint().unwrap();

    print_whitekeys(&rustbox);
    print_blackkeys(&rustbox);

    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                // println!("{:?}", key);
                match key {
                    Key::Char('q') => {
                        play_note("a0", &endpoint).unwrap().detach();
                        make_mark(1, 15, &rustbox);
                    }
                    Key::Char('w') => {
                        play_note("b0", &endpoint).unwrap().detach();
                        make_mark(1, 15, &rustbox);
                    }
                    Key::Esc => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

