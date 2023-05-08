mod hq9p;

use crate::hq9p::HQ9PInterpreter;


fn main() {

    let mut code = String::default();

    std::io::stdin().read_line(&mut code).unwrap();

    let hq9psm = HQ9PInterpreter::new(code);

    hq9psm.execute();

}
