#![allow(unused)]
struct Person {
    name: String,
    age: u8,
    favorite_fruit: String,
}

impl AsJson for Person {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
            self.name, self.age, self.favorite_fruit
        )
    }
}

struct Dog {
    name: String,
    color: String,
    likes_petting: bool,
}

impl AsJson for Dog {
    fn as_json(&self) -> String {
        format!(
            r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
            self.name, self.color, self.likes_petting
        )
    }
}
trait AsJson {
    fn as_json(&self) -> String;
}

fn send_data_as_json<T: AsJson>(value: &T) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

fn main() {
    println!("Uso de Trait");
    let laura = Person {
        name: String::from("Laura"),
        age: 31,
        favorite_fruit: String::from("apples"),
    };

    let fido = Dog {
        name: String::from("Fido"),
        color: String::from("Black"),
        likes_petting: true,
    };

    send_data_as_json(&laura);
    send_data_as_json(&fido);
}
