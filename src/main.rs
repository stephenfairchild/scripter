fn main() {
    println!("Hello scripter!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(2 + 2, 4);
    }
}
