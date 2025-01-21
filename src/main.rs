struct Point (i32, i32, i32);

fn main() {
    let corners: [Point; 8] = [
        Point( 1, -1, -5),
        Point( 1, -1, -3),
        Point( 1,  1, -5),
        Point( 1,  1, -3),
        Point(-1, -1, -5),
        Point(-1, -1, -3),
        Point(-1,  1, -5),
        Point(-1,  1, -3),
    ];

    for point in corners {
        println!("Proyected corner: x: {}, y: {})", point.0 as f64 / -point.2 as f64, point.1 as f64 / -point.2 as f64);
    }
}
