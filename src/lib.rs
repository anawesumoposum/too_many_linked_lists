use first::List0;
use first::List1;
//use first::bad;
pub mod second;
pub mod third;
pub mod fourth;
pub mod fifth;
pub mod silly;

pub fn main() {
    let list: List0<i32> = List0::Elem(1, Box::new(List0::Elem(2, Box::new(List0::Empty))));
    println!("{:?}", list);

    let list: List1<i32> = List1::ElemthenNotEmpty(1, Box::new(List1::ElemThenEmpty(2)));
    println!("{:?}", list);

    //at this point the book begins doing unit testing
}
