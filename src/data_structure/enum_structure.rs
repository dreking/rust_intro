#[allow(dead_code)]

pub fn enum_structure() {
    enum COLORS {
        RED,
        GREEN,
        BLUE,
        RGB(u8, u8, u8), // tuple
        CYMK {
            cyan: u8,
            yellow: u8,
            magenta: u8,
            black: u8,
        }, // struct
    }
    // let color = COLORS::RED;
    // let color = COLORS::RGB(0, 0, 0);

    let color = COLORS::CYMK {
        cyan: 0,
        yellow: 128,
        magenta: 0,
        black: 22,
    };
    match color {
        COLORS::RED => println!("The color is red!"),
        COLORS::GREEN => println!("The color is green!"),
        COLORS::BLUE => println!("The color is blue!"),
        COLORS::RGB(0, 0, 0) => println!("The color is black!"),
        COLORS::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        COLORS::CYMK {
            cyan: _,
            yellow: _,
            magenta: _,
            black: 255,
        } => println!("The color is black!"),
        COLORS::CYMK { black: 0, .. } => println!("The color is white!"), // .. means ignore the rest
        _ => println!("Some other color!"),
    }
}
