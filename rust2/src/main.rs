use fnv::FnvHashSet;
type Set = FnvHashSet<(i32, i32)>;

fn nth(n: i32, p0: (i32, i32)) -> Set {
    let mut s1:Set = FnvHashSet::with_capacity_and_hasher(n as usize * 4, Default::default());
    let mut s2:Set = FnvHashSet::with_capacity_and_hasher(n as usize * 4, Default::default());
    let mut s0:Set = FnvHashSet::with_capacity_and_hasher(n as usize * 4, Default::default());
    s1.insert(p0);
    for _ in 0..n {
        for p in &s1 {
            let mut add = |p: (i32, i32)| {
                if !s1.contains(&p) && !s2.contains(&p) {
                    s0.insert(p);
                }
            };
            add((p.0 - 1, p.1));
            add((p.0 + 1, p.1));
            add((p.0, p.1 - 1));
            add((p.0, p.1 + 1));
        }
        s2.clear();
        let temp1 = s1;
        let temp2 = s2;
        s1 = s0;
        s2 = temp1;
        s0 = temp2;
    }
    s1
}

fn main() {
    let s1 = nth(2000, (0, 0));
    println!("{}", s1.len());
}
