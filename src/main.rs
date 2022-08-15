use clap::{ArgEnum, Parser};
use std::io;
use unicode_width::UnicodeWidthStr;

// お顔
const BONDREWD_FACES: [&'static str; 3] = ["（｜）", "(| )", "( |)"];

#[derive(Clone, Copy, ArgEnum)]
enum Direction {
    Up,
    Left,
    Right,
}

/// Sir Bondrewd speaks
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Text to display
    #[clap(value_parser)]
    text: Option<String>,

    /// Direction of Text
    #[clap(short, long, arg_enum, value_parser, default_value_t = Direction::Right)]
    direction: Direction,

    /// Sir Bondrewd's Left Hand
    #[clap(short, long, value_parser, value_name = "TEXT")]
    lefthand: Option<String>,

    /// Sir Bondrewd's Right Hand
    #[clap(short, long, value_parser, value_name = "TEXT")]
    righthand: Option<String>,
}

fn main() {
    let args = Args::parse();
    let bondrewd = create_face(&args);

    let text = match args.text {
        Some(v) => v,
        None => {
            let mut buf = String::new();

            while let Ok(byte) = io::stdin().read_line(&mut buf) {
                if byte == 0 {
                    break;
                }
            }

            buf.trim().to_string()
        }
    };

    match args.direction {
        Direction::Up => print_up(&bondrewd, &text),
        Direction::Left => println!("{} > {}", text, bondrewd),
        Direction::Right => print_right(&bondrewd, &text),
    };
}

fn create_face(args: &Args) -> String {
    let face = BONDREWD_FACES[args.direction as usize];

    let lefthand = match &args.lefthand {
        Some(hand) => hand.to_string(),
        None => "".to_string(),
    };

    let righthand = match &args.righthand {
        Some(hand) => hand.to_string(),
        None => "".to_string(),
    };

    format!("{}{}{}", lefthand, face, righthand)
}

fn print_up(face: &String, text: &String) {
    let lines: Vec<&str> = text.split('\n').collect();
    let max_width = match lines.iter().map(|e| UnicodeWidthStr::width(*e)).max() {
        Some(n) => n,
        None => unreachable!(),
    };

    let indent = " ".repeat(UnicodeWidthStr::width(face.as_str()) / 2);

    println!("{}  {}", indent, "⎽".repeat(max_width));

    for (i, line) in lines.iter().enumerate() {
        let bracket = if lines.len() == 1 {
            ['<', '>']
        } else if i == 0 {
            ['/', '\\']
        } else if i == lines.len() - 1 {
            ['\\', '/']
        } else {
            ['|', '|']
        };

        let spaces = " ".repeat(max_width - UnicodeWidthStr::width(*line));

        println!(
            "{}{} {}{} {}",
            indent, bracket[0], *line, spaces, bracket[1]
        );
    }

    println!("{}  v{}", indent, "⎺".repeat(max_width - 1));
    println!(" {}", face);
}

fn print_right(face: &String, text: &String) {
    let face_width = UnicodeWidthStr::width(face.as_str()) + 3;
    let lines: Vec<&str> = text.split('\n').collect();

    for (i, line) in lines.iter().enumerate() {
        if i == lines.len() - 1 {
            println!("{} < {}", face, line);
            return;
        }

        println!("{}{}", " ".repeat(face_width), line);
    }
}
