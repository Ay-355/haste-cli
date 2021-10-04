mod app;

fn main() {
    let matches = app::build_app().get_matches();
    println!("{:?}", matches);
}
