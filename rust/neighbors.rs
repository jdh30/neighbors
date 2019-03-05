use std::collections::HashSet;
use std::hash::BuildHasherDefault;

use std::default::Default;
use std::hash::Hasher;

pub struct FnvHasher(u64);

impl Default for FnvHasher {
    #[inline]
    fn default() -> FnvHasher {
        FnvHasher(0xcbf29ce484222325)
    }
}

impl Hasher for FnvHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        panic!("Cannot hash arbitrary bytes.");
        let FnvHasher(mut hash) = *self;

        for byte in bytes.iter() {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        *self = FnvHasher(hash);
    }

    fn write_i32(&mut self, value: i32) {
        let FnvHasher(mut hash) = *self;

        hash <<= 10;
        hash ^= value as u64;

        *self = FnvHasher(hash);
    }

}

type Set = HashSet<(i32, i32), BuildHasherDefault<FnvHasher>>;

fn Empty() -> Set {
    let hasher = BuildHasherDefault::<FnvHasher>::default();
    HashSet::with_hasher(hasher)
}

fn iterNeighbors<F>(mut f: F, (i, j): (i32, i32)) -> ()
    where F: FnMut((i32, i32)) -> ()
{
    f((i-1, j));
    f((i+1, j));
    f((i, j-1));
    f((i, j+1));
}

fn nthLoop(n: i32, s1: Set, s2: Set) -> Set {
    if n == 0 {
        return s1;
    } else {
        let mut s0 = Empty();
        for &p in &s1 {
            let add = |p| {
                if !(s1.contains(&p) || s2.contains(&p)) {
                    s0.insert(p);
                }
            };
            iterNeighbors(add, p);
        }
        drop(s2);
        return nthLoop(n-1, s0, s1);
    }
}

fn nth(n: i32, p: (i32, i32)) -> Set {
    let mut s1 = Empty();
    s1.insert(p);
    let s2 = Empty();
    nthLoop(n, s1, s2)
}

fn main() {
    let s = nth(6000, (0, 0));
    println!("{}", s.len());
}
