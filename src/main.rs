struct Point (i32, i32, i32);

fn main() {
    let corners: [Point; 8] = [
        Point( 0, 0,  0),
        Point(12, 0,  0),
        Point(12, 8,  0),
        Point( 0, 8,  0),
        Point( 0, 0, 10),
        Point(12, 0, 10),
        Point(12, 8, 10),
        Point( 0, 8, 10),
    ];

}
