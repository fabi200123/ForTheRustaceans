fn main() {
    let rect = Rectangle {
        width: 25,
        length: 40,
    };

    let tri = Triangle {
        base: 5.0,
        height: 5.0,
    };

    let sq = Rectangle::square(6);

    let triangle_area = tri.area();
    println!("Area of the triangle is {triangle_area}");

    let rectangle_area = rect.area();
    println!("Area of the rectangle is {rectangle_area}");

    let square_area = sq.area();
    println!("Area of the square is {square_area}");
}

struct Rectangle {
    width: u64,
    length: u64,
}

impl Rectangle{
    fn area(&self) -> u64 {
        self.width * self.length
    }

    fn square(size: u64) -> Self {
        Self {
            width: size,
            length: size,
        }
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Triangle{
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}
