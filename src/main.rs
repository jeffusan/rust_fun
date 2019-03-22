fn main() {
    println!("Hello, world!");
}

mod stack {

    pub struct Stack {

        item_vec: Vec<i8>,

        max_vec: Vec<i8>
    }

    impl Stack {

        pub fn new() -> Stack {
            Stack {
                item_vec:  Vec::new(),
                max_vec:  Vec::new()
            }
        }

        pub fn push(&mut self, item: i8) {
            self.item_vec.push(item);

            println!("{} item vec len", self.item_vec.len());

            if self.max() > item {
                self.max_vec.push(self.max())
            } else {
                self.max_vec.push(item)
            }
        }

        pub fn pop(&mut self) -> Option<i8> {
            self.max_vec.pop();

            self.item_vec.pop()
        }


        pub fn max(&self) -> i8 {
            match self.max_vec.last() {
                Some(x) => {
                    println!("{} max", x);
                    x.to_owned()
                }
                None => 0,
            }
        }
    }

    #[test]
    fn push_test() {
        let mut s = Stack::new();

        s.push(1);

        assert_eq!(1, s.pop().unwrap_or(0))
    }

    #[test]
    fn max_test() {
        let mut s = Stack::new();

        s.push(1);
        s.push(5);
        s.push(3);

        assert_eq!(5, s.max())
    }
}
