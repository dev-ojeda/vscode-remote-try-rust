#![allow(unused)]

struct User { 
    name: String,
    first_name: String,
    last_name: String,
    user: String, 
    pass: String, 
}
trait TodoUser {

    fn default() -> Self;

    fn new(name: String, first_name: String, last_name: String, user: String, pass: String) -> Self;

    fn set_name(&mut self, name: String);

    fn set_first_name(&mut self, first_name: String);

    fn set_last_name(&mut self, last_name: String);

    fn set_user(&mut self, user: String);

    fn set_pass(&mut self, pass: String);

    fn name(&self) -> &str;

    fn first_name(&self) -> &str;

    fn last_name(&self) -> &str;

    fn user(&self) -> &str;

    fn pass(&self) -> &str;

    fn name_mut(&mut self) -> &mut String;

    fn first_name_mut(&mut self) -> &mut String;

    fn last_name_mut(&mut self) -> &mut String;

    fn user_mut(&mut self) -> &mut String;

    fn pass_mut(&mut self) -> &mut String;
}
impl TodoUser for User {

    fn default() -> User {
        User{name: String::new() ,first_name: String::new(), last_name: String::new(), user: String::new(),pass: String::new()}
    }

    fn new(name: String, first_name: String, last_name: String, user: String, pass: String) -> Self { Self { name, first_name, last_name, user, pass } }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    fn set_user(&mut self, user: String) {
        self.user = user;
    }

    fn set_pass(&mut self, pass: String) {
        self.pass = pass;
    }

    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn first_name(&self) -> &str {
        self.first_name.as_ref()
    }

    fn last_name(&self) -> &str {
        self.last_name.as_ref()
    }

    fn user(&self) -> &str {
        self.user.as_ref()
    }

    fn pass(&self) -> &str {
        self.pass.as_ref()
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn first_name_mut(&mut self) -> &mut String {
        &mut self.first_name
    }

    fn last_name_mut(&mut self) -> &mut String {
        &mut self.last_name
    }

    fn user_mut(&mut self) -> &mut String {
        &mut self.user
    }

    fn pass_mut(&mut self) -> &mut String {
        &mut self.pass
    }
}

fn main(){
    let todo_user = User::default();
}