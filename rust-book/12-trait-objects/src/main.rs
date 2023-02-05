fn main() {
    let button = Button {
        label: String::from("OK"),
        width: 120,
        height: 30
    };

    let select_box = SelectBox {
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No")
        ],
        width: 200,
        height: 40
    };

    let screen = Screen {
        components: vec![
            Box::new(button),
            Box::new(select_box)
        ]
    };

    screen.run();
}

trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {} x {}, {}", self.width, self.height, self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box: {} x {}, {:?}", self.width, self.height, self.options);
    }
}
