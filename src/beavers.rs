use idpf::*;
use idpf::RingElm;
use idpf::prg::*;
use std::ops::Index;
use std::error::Error;
pub struct Beaver{
    pub a: Vec<RingElm>,
    pub n: usize
}

impl Beaver{
    pub fn gen(n: usize) ->Self{
        let N: usize = (1<<n) - 1;
        let mut rv = Vec::<RingElm>::new();
        let zero = RingElm::from(0u32);
        for _ in 0.. N{
            rv.push(zero.clone());
        }

        let seed = PrgSeed::random();
        let mut stream = FixedKeyPrgStream::new();
        stream.set_key(&seed.key);
        for i in 0..=n-1{
            let loc = (1<<i) - 1; // convert the loc to index by minus 1

            let rd_bits = stream.next_bits(NUMERIC_LEN);
            rv[loc] = RingElm::from( bits_to_u32(&rd_bits[..NUMERIC_LEN]));
            //rv[loc] = RingElm::from((i+2) as u32);
        }// initialize the n atom-s which is in the 2^i-1 location
        //println!("{:?}", rv);
        // assign the other locations
        for i in 0..N{
            let mut j = i+1; //j is the location

            /*Decomposite j to a binary stream, and if the bit is equal to 1, then operate rv */
            let mut bits = Vec::<usize>::new();
            while j != 0{
                bits.push(j%2);
                j = j / 2;
            }
            //println!("bits={:?}", bits);
            let mut e = RingElm::from(1u32);
            let mut bit_loc = bits.len();
            while !bits.is_empty(){
                if bits.pop().unwrap() == 1{
                    e.mul(&rv[(1<<(bit_loc-1)) - 1]);
                }
                bit_loc -= 1;
            }
            rv[i] = e;
        } 
        //println!("{:?}", rv);
        Beaver { a: rv, n: n}
    }

    pub fn split(&self) -> (Beaver, Beaver){
        let len = self.a.len();
        let mut rv1 = Vec::<RingElm>::new();
        let mut rv2 = Vec::<RingElm>::new();
        let seed = PrgSeed::random();
        let mut stream = FixedKeyPrgStream::new();
        stream.set_key(&seed.key);
        for i in 0..=len-1{
            let rd_bits = stream.next_bits(NUMERIC_LEN);
            let e1 = RingElm::from( bits_to_u32(&rd_bits[..NUMERIC_LEN]));
            let mut  e = self.a[i].clone();
            e.sub(&e1);
            rv1.push(e1);
            rv2.push(e);
        }

        let bv1 = Beaver{a: rv1, n: len};
        let bv2 = Beaver{a: rv2, n: len};
        (bv1, bv2)
    }

    pub fn extendfrom(v: Vec<RingElm>)->Self{
        let n = v.len();
        let N: usize = (1<<n) - 1;
        let mut rv = Vec::<RingElm>::new();
        let zero = RingElm::from(0u32);
        for _ in 0.. N{
            rv.push(zero.clone());
        }
        for i in 0..=n-1{
            let loc = (1<<i) - 1; // convert the loc to index by minus 1
            rv[loc] = v[i].clone();
            //rv[loc] = RingElm::from((i+1) as u32);
        }// initialize the n atom-s which is in the 2^i-1 location

        let mut e = rv[0].clone();
        e.mul(&rv[1]);
        rv[2] = e;

        for i in 2..n{
            let loc = (1<<i) - 1; //the i-th atom item.
            let e = rv[loc].clone(); 
        
            for j in 1..=loc{
                rv[loc+j] = rv[j-1].clone(); 
                rv[loc+j].mul(&e);
            }
        }  
        println!("{:?}", rv);
        Beaver { a: rv, n: n }
    }

}

impl Index<usize> for Beaver {
    type Output = RingElm;

    fn index(&self, index: usize) -> & RingElm {
        let loc = 1<<index;
        &self.a[loc-1]
    }
}

//delta is extend from the opened ring values, and b is the beaver tuples shared by the two parites in offline phase
pub fn Muls(delta: &Beaver, b: &Beaver, is_server: bool)->Result<RingElm, Box<dyn Error>>{
    if delta.a.len() != b.a.len() && delta.n == b.n{
        return Err("the two beaver tuples don't match".into());
    }
    //println!("delta={:?}, b={:?}", delta.a, b.a);
    let n = delta.n;
    let N = (1<<n) - 1; //0x0..01..1(n 1-s)
    //println!("n={}, N={}", n, N);
    let mut r: RingElm = RingElm::from(0);
    if is_server{
        r.add(&delta.a[N-1]);
    }
    r.add(&b.a[N-1]);
    for i in 1..N{
        let mut unit = delta.a[i-1].clone();
        let index = (!i) & N;
        //println!("i={}, j={}", i, index);
        unit.mul(&b.a[index-1]);
        r.add(&unit);
    }

    Ok(r)
}