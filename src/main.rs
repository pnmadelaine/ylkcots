use std::collections::HashSet;

fn main() {
    let mut lines = std::io::stdin().lines();
    let _ = lines.next();
    let a: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap() - 1)
        .collect();
    let n = a.len();
    let mut res = vec![None; n];
    let mut set = HashSet::new();
    set.insert(0);
    for v in 0..n {
        let mut new_set = HashSet::new();
        for i in set {
            if res[i].is_none() {
                res[i] = Some(v);
                new_set.insert(a[i]);
                if i > 0 {
                    new_set.insert(i - 1);
                }
                if i < n - 1 {
                    new_set.insert(i + 1);
                }
            }
        }
        set = new_set;
    }
    for m in res {
        print!("{} ", m.unwrap());
    }
    print!("\n");
}
