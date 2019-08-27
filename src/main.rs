extern crate termion;

use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::{IntoRawMode};
use termion::clear;
use termion::cursor;
use termion::screen::AlternateScreen;

use std::io::{Write, BufWriter, stdout, stdin};
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

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

fn docker_images_display<W: Write>(screen: &mut AlternateScreen<W>, display_cursor: &Cursor, tag: &str, images: &Vec<DockerImage>) {
    let mut output = String::new();
    output += &format!("    {}\r\n", tag);
    for image in images {
        output += &format!("[{}] {}\r\n", if image.delete_flug { "x" } else { " " }, image.display);
    }
    write!(screen, "{}", clear::All).unwrap(); // 画面をクリア
    write!(screen, "{}", cursor::Goto(1, 1)).unwrap();
    write!(screen, "{}", output).unwrap(); // docker imagesを描画
    write!(screen, "{}", cursor::Goto(display_cursor.column as u16 + 1, display_cursor.row as u16 + 1)).unwrap(); // カーソルを移動
    screen.flush().unwrap();
}

fn main() {
    // dockerのimagesを見れるようにする。
    let output = Command::new("docker")
                            .arg("images")
                            .output()
                            .expect("docker is not found");
    
    let images_str = String::from_utf8(output.stdout).unwrap(); // $ docker images のアウトプットを取得

    let images_vec_str: Vec<&str> = images_str.split("\n").collect(); // 個別のimageの情報に分割
    let images_vec = images_vec_str.iter()
                            .map(|x: &&str| -> Vec<&str> { x.split(" ").filter(|x| x != &"").collect::<Vec<&str>>() })
                            .collect::<Vec<Vec<&str>>>();

    let mut vec = Vec::new();
    for (index, image_vec) in images_vec[1..].iter().enumerate() {
        if image_vec.len() == 0 {
            break;
        }

        let image_id = image_vec[2];

        let mut docker_image = DockerImage::new(images_vec_str[index+1], image_id);
        vec.push(docker_image)
    }

    let stdin = stdin();
    let mut screen = AlternateScreen::from(BufWriter::new(stdout()).into_raw_mode().unwrap());
    // let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut display_cursor = Cursor{ column: 0, row: 0};

    // docker imagesを表示
    docker_images_display(&mut screen, &display_cursor, images_vec_str[0], &vec);

    let (tx, rx) = channel();

    // 入力は別スレッドで受け取りmpscに流す
    thread::spawn(move || {
        for c in stdin.events() {
            if let Ok(evt) = c {
                tx.send(evt).unwrap();
            }
        }
    });

    loop {
        // 16ミリ秒でタイムアウト
        if let Ok(evt) = rx.recv_timeout(Duration::from_millis(16)) {
            match evt {
                Event::Key(Key::Char('\n')) => {
                    let rm_images: Vec<&str> = vec.iter()
                                                .filter(|x| x.delete_flug)
                                                .map(|x| x.image_id.as_str())
                                                .collect();
                    if rm_images.len() == 0 {
                        return;
                    } else {
                        let output = Command::new("docker")
                                        .arg("rmi")
                                        .args(&rm_images)
                                        .output()
                                        .expect("");
                        return;
                    }
                }
                Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => { return; }
                Event::Key(Key::Char('j')) => {
                    if display_cursor.row < vec.len() + 1 {
                        display_cursor.row += 1;
                    } else { }}
                Event::Key(Key::Char('k')) => {
                    if display_cursor.row > 0 {
                        display_cursor.row -= 1;
                    } else { }}
                Event::Key(Key::Char('x')) => {
                    if 0 < display_cursor.row && display_cursor.row < vec.len() +1 {
                        let delete_flug = vec[display_cursor.row - 1].delete_flug;
                        if delete_flug {
                            vec[display_cursor.row - 1].delete_flug = false;
                        } else {
                            vec[display_cursor.row - 1].delete_flug = true;
                        }
                    }
                }
                _ => {}
            }
            // docker imagesを表示
            docker_images_display(&mut screen, &display_cursor, images_vec_str[0], &vec);
        }
        // 描画処理とか
        write!(screen, "{}", cursor::Goto(display_cursor.column as u16 + 1, display_cursor.row as u16 + 1)).unwrap();
        screen.flush().unwrap();
    }
}