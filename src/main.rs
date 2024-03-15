struct _Square {
    side: f32,
    _line_width: u8,
    _color: String,
}

struct _Rectangle {
    length: f32,
    width: f32,
    _line_width: u8,
    _color: String,
}

trait _Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented for this shape");
        0.0
    }
}

impl _Shape for _Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

impl _Shape for _Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
    fn perimeter(&self) -> f32 {
        2.0 * (self.length + self.width)
    }
}

fn _return_instance_of_trait() -> impl _Shape {
    let square = _Square {
        side: 5.0,
        _line_width: 1,
        _color: "blue".to_string(),
    };
    square
}

fn main() {}
