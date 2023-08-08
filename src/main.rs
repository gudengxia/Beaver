mod beavers;
use beavers::*;
use idpf::RingElm;
use idpf::*;
fn main() {
    
}


mod test{
    use crate::beavers::*;
    use idpf::*;
    use idpf::RingElm;

    #[test]
    fn beavers_gen_works(){
        let beaver = Beaver::gen(2);
    }
   
    fn beaver_split_works()
    {
        let n = 4;
        let b = Beaver::gen(n);
        let (b1, b2) = b.split();

        let mut r1 = b1.a.clone();
        let r2 = b2.a.clone();

        for i in 0..r1.len(){
            r1[i].add(&r2[i]);
        }

        for i in 0..r1.len(){
            assert_eq!(r1[i], b[i])
        }
    }


    #[test]
    fn Muls_works(){
        let n: usize = 6;
        let mut v1= Vec::<RingElm>::new();
        for i in 1..=n{
            v1.push(RingElm::from(i as u32));
        }

        let mut v2= Vec::<RingElm>::new();
        for i in 1..=n{
            v2.push(RingElm::from((i+1) as u32));
        }

        let beaver = Beaver::gen(n);

        let (b1, b2) = beaver.split();

        let mut d1 = v1.clone();
        let mut d2 = v2.clone();

        for i in 0..n{
            d1[i].sub(&b1[i]);
            d2[i].sub(&b2[i]);
        }

        let mut d = d1.clone();
        for i in 0..n{
            d[i].add(&d2[i]);
        }

         //let delta = Beaver::extendfrom(d);

        //let r1 = Muls(&delta, &b1, true).unwrap();
        //let r2 = Muls(&delta, &b2, false).unwrap();
        let r1 = product(&d, &b1, true).unwrap();
        let r2 = product(&d, &b2, false).unwrap();

        let mut result = r1.clone();
        result.add(&r2);

        let mut v_mul = RingElm::from(1);
        for i in 0..v1.len(){
            let mut unit = v1[i].clone();
            unit.add(&v2[i]);
            v_mul.mul(&unit);
        } 
        assert_eq!(v_mul, result);
    }
}
