mod beavers;
use beavers::*;

fn main() {
    let beaver = Beaver::gen(4);
}


mod test{
    use crate::beavers::*;

    #[test]
    fn beavers_gen_works(){
        let beaver = Beaver::gen(2);
    }
}