// extern crate argparse;

// use argparse::{ArgumentParser, StoreTrue, Store};
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

fn main() {
    // まずは引数を取らずに、dockerのimageを見れるようにする。
    let output = Command::new("docker")
                            .arg("images")
                            .output()
                            .expect("docker is not found");
    
    let images_str = String::from_utf8(output.stdout).unwrap(); // $ docker images のアウトプットを取得
    // println!("output: \n{}", images_str);

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
    for i in vec.iter() {
        println!("[{}] {}", if i.delete_flug { "x" } else { " " }, i.display);
    }
}