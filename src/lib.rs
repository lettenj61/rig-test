pub mod greeting {

    pub struct {{greeter | Camel}} {
        greet: String,
    }

    impl {{greeter | Camel}} {

        pub fn new() -> {{greeter | Camel}} {
            {{greeter | Camel}} {
                greet: "{{greeting}}".to_string(),
            }
        }

        pub fn print_greet(&self) {
            println!("{}", self.greet);
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
