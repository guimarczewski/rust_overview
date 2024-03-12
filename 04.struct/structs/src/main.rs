struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("guimarcz@gmail.com"),
        username: String::from("guimarcz"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("guilhermem");

    let user2 = build_user(
        String::from("geo@gmail.com"),
        String::from("geo")
    );

    let user3 = User {
        email: String::from("abc@gmail.com"),
        username: String::from("abc"),
        ..user1
    };

    println!("{}", user3.username);

    /* calcular a area */
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    
    /* calcular a area2 - usando tuples */
    let rect = (30, 50);

    println!(
        "The area1 of the rectangle is {} square pixels.",
        area2(rect)
    );

    /* calcular a area3 - usando structs */
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area2 of the rectangle is {} square pixels.",
        area3(&rect)
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/* funcao para calcular a area */
fn area(width: u32, height: u32) -> u32 {
    width * height
}

/* funcao para calcular a area2 - nova versao usando tuples */
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

/* funcao para calcular a area3 - nova versao usando structs */
struct  Rectangle {
    width: u32,
    height: u32
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}