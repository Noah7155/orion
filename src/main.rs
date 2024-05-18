use termion::{ 
    raw::{
        IntoRawMode, 
        RawTerminal
    }, 
    cursor, 
    color,
    clear
    
};

use std::{ 
    io::{
        Write, 
        stdout, 
        Stdout,
        stdin, 
        Stdin,
        Read
    },
    process::exit,
};

struct Cursorpos {
    x: u16,
    y: u16
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = stdin();
    let mut pos = Cursorpos { x: 1, y: 1 };

    write!(stdout, "{}", clear::All).expect("RawModeError");
    loop {
        render(&mut stdout);
        let keyin = getkeypress(&mut stdin);
        write!(stdout, "{}{}{} ('{}')", cursor::Goto(1, 1), clear::All, keyin, keyin as char).expect("StdoutError");
        processkeypress(&mut pos, keyin);
        write!(stdout, "{}e", cursor::Goto(pos.x, pos.y)).expect("CursorError");
    }
}

fn update(stdout: &mut RawTerminal<Stdout>) {
    stdout.flush().unwrap();
}

fn render(stdout: &mut RawTerminal<Stdout>) {
    print!("{}{}{}{}{}{}{}{}",
        cursor::Hide,
        cursor::Goto(0, getborders().1 - 1),
        color::Bg(color::Cyan),
        color::Fg(color::Black),
        color::Bg(color::Yellow),
        rightpad(" NORMAL ".to_string(), " ".to_string()),
        color::Bg(color::Reset),
        color::Fg(color::Reset),
        );
    update(stdout);
}

fn getborders() -> (u16, u16) {
    termion::terminal_size().unwrap()
}

fn getkeypress(stdin: &mut Stdin, ) -> u8 {
    for c in stdin.bytes() {
        match c {
            _ => return c.expect("KeyError"),
        }
    }
    return 0;
}

fn rightpad(mut str: String, chr: String) -> String {
    for _i in 0..(<u16 as Into<usize>>::into(getborders().0) - str.len()) {
        str = str + &chr;
    }
    return str
}

fn processkeypress(pos: &mut Cursorpos, key: u8, ) {
    match key {
        b'h' => pos.x = pos.x - 1,
        b'j' => pos.y = pos.y + 1,
        b'k' => pos.y = pos.y - 1,
        b'l' => pos.x = pos.x + 1,
        17 => exit(0),
        _ => {}
    }
    if pos.x < 1 { pos.x = 1 };
    if pos.y < 1 { pos.y = 1 };
    if pos.x > getborders().0 { pos.x = getborders().0 };
    if pos.y > getborders().1 { pos.y = getborders().1 };
}
