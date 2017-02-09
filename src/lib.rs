pub mod greeting {
    pub fn print_greet() {
        println!("{{greeting}}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
