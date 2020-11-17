#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn area(r: Rectangle) -> f32 {
    let Point { x: x1, y: y1 } = r.top_left;
    let Point { x: x2, y: y2 } = r.bottom_right;

    (x2 - x1) * (y2 - y1)
}

fn square(p: Point, width: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: p.x,
            y: p.y + width,
        },
        bottom_right: Point {
            x: p.x + width,
            y: p.y,
        },
    }
}

pub fn run() {
    println!("p6_struct >>>>>>>>");
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destruct the point to variable top_edge and left_edge
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destruct a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("area: {}", area(_rectangle));

    let lower_left = Point { x: 0.0, y: 0.0 };
    let _square = square(lower_left, 5.0);
    println!("square: {:?}", _square);
    println!("\n");
}
