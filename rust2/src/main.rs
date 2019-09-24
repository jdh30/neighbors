use fnv::FnvHashSet;

fn main() {
    let mut s1:FnvHashSet<(i32, i32)> = FnvHashSet::with_capacity_and_hasher(8000, Default::default());
    let mut s2:FnvHashSet<(i32, i32)> = FnvHashSet::with_capacity_and_hasher(8000, Default::default());
    let mut s0:FnvHashSet<(i32, i32)> = FnvHashSet::with_capacity_and_hasher(8000, Default::default());
    s1.insert((0, 0));
    for _ in 0..2000 {
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
    println!("{}", s1.len());
}
