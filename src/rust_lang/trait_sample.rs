// 定义一个 trait，表示可以计算面积的类型
trait Area {
    fn area(&self) -> f64;
}
// 定义一个结构体，表示二维平面上的矩形
#[derive(Debug)]
struct Rectangle<'a, T: Area> {
    width: T,
    height: T,
    name: &'a str,
}
// 实现 Area trait for Rectangle 结构体
impl<'a, T: Area> Area for Rectangle<'a, T> {
    fn area(&self) -> f64 {
        self.width.area() * self.height.area()
    }
}

// 定义一个结构体，表示圆形
#[derive(Debug)]
struct Circle {
    radius: f64,
}
// 实现 Area trait for Circle 结构体
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义一个函数，用于计算多个具有 Area trait 的类型的总面积
fn total_area(shapes: &Vec<&dyn Area>) -> f64 {
    let mut total = 0.0;
    for shape in shapes {
        total += shape.area();
    }
    total
}

pub fn do_area() {
    let rectangle = Rectangle {
        width: Circle { radius: 2.0 },
        height: Circle { radius: 3.0 },
        name: "Rectangle",
    };

    let circle = Circle { radius: 4.0 };

    let shapes: Vec<&dyn Area> = vec![&rectangle, &circle];

    println!("Total area: {}", total_area(&shapes));
}