struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // calculates the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Enter the height and width of the rectangle: ");
    let mut height = String::new();
    let mut width = String::new();
    std::io::stdin().read_line(&mut height).expect("Failed to read line");
    std::io::stdin().read_line(&mut width).expect("Failed to read line");
    let height: u32 = height.trim().parse().expect("Please type a number!");
    let width: u32 = width.trim().parse().expect("Please type a number!");
    let rect = Rectangle { width, height };
    println!("The area of the rectangle is: {}", rect.area());
}
