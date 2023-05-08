pub struct HQ9PInterpreter {
    code: String
}


impl HQ9PInterpreter {
    pub fn new(code: String) -> HQ9PInterpreter {
        return HQ9PInterpreter {code};
    }

    pub fn execute(self) {
        let mut accumulator = 0;
        for i in self.code.bytes() {
            match i {
                b'H' => {
                    println!("Hello, world!");
                },
                b'Q' => {
                    println!("{}", self.code);
                },
                b'9' => {
                    let mut current = 99;
                    while current >= 1 { 
                        println!("{} bottles of beer\nyou take one down, pass it around,\n{} bottles of beer on the wall.\n",
                        current, current - 1); current -= 1;
                    }

                },
                b'+' => {
                    accumulator += 1; 
                },
                b'A' => {
                    println!("{accumulator}");
                },
                _ => ()
            }
        }        
    }

}