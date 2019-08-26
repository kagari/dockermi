extern crate termion;

use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use termion::clear;
use termion::cursor;
use std::io::{Write, stdout, stdin};
use termion::screen::AlternateScreen;
use std::process::Command;

#[derive(Debug)]
struct DockerImage {
    display: String,
    image_id: String,
    delete_flug: bool,
}

impl DockerImage {
    fn new(display: &str, image_id: &str) -> DockerImage {
        return DockerImage{ display:display.to_owned(),
                            image_id: image_id.to_owned(),
                            delete_flug : false}
    }
}

struct Cursor {
    row: usize,
    column: usize,
}

fn docker_images_display<W: Write>(stdout: &mut AlternateScreen<W>, display_cursor: &Cursor, tag: &str, images: &Vec<DockerImage>) {
    write!(stdout, "{}", clear::All); // 画面をクリア
    write!(stdout, "{}", cursor::Goto(1, 1));
    write!(stdout, "    {}\r\n", tag);
    for image in images {
        write!(stdout, "[{}] {}\r\n", if image.delete_flug { "x" } else { " " }, image.display);
    }
    write!(stdout, "{}", cursor::Goto(display_cursor.column as u16 + 1, display_cursor.row as u16 + 1)); // カーソルを移動
}

fn main() {
    // dockerのimagesを見れるようにする。
    let output = Command::new("docker")
                            .arg("images")
                            .output()
                            .expect("docker is not found");
    
    let images_str = String::from_utf8(output.stdout).unwrap(); // $ docker images のアウトプットを取得

    let images_vec_str: Vec<&str> = images_str.split("\n").collect(); // 個別のimageの情報に分割
    let images_vec = images_vec_str.iter().map(|x: &&str| -> Vec<&str> { x.split(" ").filter(|x| x != &"").collect::<Vec<&str>>() } ).collect::<Vec<Vec<&str>>>();

    let mut images_iter = images_vec.iter();
    images_iter.next(); // headerを取り除く処理

    let mut vec = Vec::new();
    for (index, image_vec) in images_iter.enumerate() {
        if image_vec.iter().count() == 0 {
            break;
        }

        let image_id = image_vec.iter().nth(2).unwrap();

        let mut docker_image = DockerImage::new(images_vec_str.iter().nth(index+1).unwrap(), image_id);
        vec.push(docker_image)
    }

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut display_cursor = Cursor{ column: 0, row: 0};

    // docker imagesを表示
    docker_images_display(&mut stdout, &display_cursor, images_vec_str[0], &vec);

    for c in stdin.events() {
        // docker imagesを表示
        docker_images_display(&mut stdout, &display_cursor, images_vec_str[0], &vec);

        match c.unwrap() {
            Event::Key(Key::Char('\n')) => {
                println!("hoge");
                return;
            }
            Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => { return; }
            Event::Key(Key::Char('j')) => {
                if display_cursor.row < images_vec_str.iter().count() {
                    display_cursor.row += 1;
                } else { }}
            Event::Key(Key::Char('k')) => {
                if display_cursor.row > 0 {
                    display_cursor.row -= 1;
                } else { }}
            Event::Key(Key::Char('x')) => {
                vec[display_cursor.row - 1].delete_flug = true
            }
            _ => {}
        }
        write!(stdout, "{}", cursor::Goto(display_cursor.column as u16 + 1, display_cursor.row as u16 + 1));
        stdout.flush().unwrap();
    }
}