
fn download_data(url : &str, callback : impl FnOnce(&str)){

    println!("Downloading from {}", url);

    std::thread::sleep(
        std::time::Duration::from_secs(1)
    );

    let data = format!("Some data from {}", url);

    callback(&data);

}

fn main() {

    let print_data = |data : &str| println!("{}", data);

    download_data("www.google.com", print_data);


}
