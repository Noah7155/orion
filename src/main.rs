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
        Read,
        BufReader,
        BufRead,
        Result,
        Lines
    },
    process::exit,
    env,
    fs::File,
    path::Path
};

struct Editor {
    cx: u16,
    cy: u16,
    scrollstate: u16,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = stdin();
    let mut editor = Editor { cx: 1, cy: 1, scrollstate: 0 };
    let mut buffer = read_lines(&args[1]).expect("FileReadError").flatten();

    write!(stdout, "{}", clear::All).expect("RawModeError");


    loop {
        for line in &mut buffer {
            print!("{}\r\n", line);
        }
        render(&mut stdout, &editor);

        let keyin = getkeypress(&mut stdin);
        processkeypress(&mut editor, keyin);

        if keyin == 17 { 
            write!(stdout, "{}", cursor::Show).expect("CursorRestoreError"); 
            stdout.suspend_raw_mode().expect("SuspendRawModeError"); 
            exit(0);
        }

    }
}

fn update(stdout: &mut RawTerminal<Stdout>) {
    stdout.flush().unwrap();
}

fn render(stdout: &mut RawTerminal<Stdout>, editor: &Editor) {
    print!("{}{}{}{}{}{}{}{}",
        cursor::Goto(1, 1),
        cursor::Goto(0, getborders().1 - 1),
        color::Bg(color::Cyan),
        color::Fg(color::Black),
        rightpad(" NORMAL ".to_string(), " ".to_string()),
        color::Bg(color::Reset),
        color::Fg(color::Reset),
        cursor::Goto(editor.cx, editor.cy)
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

fn processkeypress(editor: &mut Editor, key: u8) {
    match key {
        b'h' => editor.cx = editor.cx - 1,
        b'j' => editor.cy = editor.cy + 1,
        b'k' => editor.cy = editor.cy - 1,
        b'l' => editor.cx = editor.cx + 1,
        _ => {}
    }
    if editor.cx < 1 { editor.cx = 1 };
    if editor.cy < 1 { editor.cy = 1 };
    if editor.cx > getborders().0 { editor.cx = getborders().0 };
    if editor.cy > getborders().1 { editor.cy = getborders().1 };
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
