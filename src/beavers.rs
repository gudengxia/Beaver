use idpf::*;
use idpf::RingElm;
use idpf::prg::*;
pub struct Beaver{
    pub a: Vec<RingElm>
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
            //rv[loc] = RingElm::from((i+1) as u32);
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
            let mut r = RingElm::from(1u32);
            let mut bit_loc = bits.len();
            while !bits.is_empty(){
                if bits.pop().unwrap() == 1{
                    r.mul(&rv[(1<<(bit_loc-1)) - 1]);
                }
                bit_loc -= 1;
            }
            rv[i] = r;
        } 
        //println!("{:?}", rv);
        Beaver { a: rv }
    }

    fn split(&self) -> (Beaver, Beaver){
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

        let bv1 = Beaver{a: rv1};
        let bv2 = Beaver{a: rv2};
        (bv1, bv2)
    }
}