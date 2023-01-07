fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let korean = "안녕, 세상아!";
    let japanese = "ハロー・ワールド";

    let regions = [southern_germany, korean, japanese];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}