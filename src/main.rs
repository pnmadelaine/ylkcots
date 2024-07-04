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
    let mut next = Vec::new();
    next.push(0);
    for v in 0..n {
        let mut new_next = Vec::new();
        for i in next {
            if res[i].is_none() {
                res[i] = Some(v);
                new_next.push(a[i]);
                if i > 0 {
                    new_next.push(i - 1);
                }
                if i < n - 1 {
                    new_next.push(i + 1);
                }
            }
        }
        next = new_next;
    }
    for m in res {
        print!("{} ", m.unwrap());
    }
    print!("\n");
}
