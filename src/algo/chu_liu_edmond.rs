pub fn chu_liu_edmond(n: usize, edges: Vec<(usize, usize, i32)>, root: usize) -> i32 {
    let mut mins: Vec<(i32, Option<usize>)> = vec![(i32::MAX, None); n];
    for (f, t, w) in edges.iter() {
        if *w < mins[*t].0 {
            mins[*t] = (*w, Some(*f));
        }
    }
    mins[root] = (-1, None);
    for e in &mins {
        print!("({}, {:?}),", e.0, e.1);
    }
    println!();

    let mut group: Vec<usize> = vec![0; n];
    let mut comp: Vec<bool> = vec![false; n];
    let mut cnt = 0;
    let mut used: Vec<bool> = vec![false; n];

    for v in 0..n {
        if used[v] {
            continue;
        }
        let mut chain: Vec<usize> = vec![];
        let mut cur: i32 = v as i32;
        loop {
            chain.push(cur as usize);
            used[cur as usize] = true;
            if let Some(nex) = mins[cur as usize].1 {
                cur = nex as i32;
                if used[cur as usize] {
                    break;
                }
            } else {
                break;
            }
        }
        dbg!(&chain);
        if cur != -1 {
            let mut cycle = false;
            for e in chain {
                group[e] = cnt;
                if e == cur as usize {
                    cycle = true;
                    comp[cnt] = true;
                }
                if !cycle {
                    cnt += 1;
                }
            }
            if cycle {
                cnt += 1;
            }
        } else {
            for e in chain {
                group[e] = cnt;
                cnt += 1;
            }
        }
    }

    dbg!(&cnt);
    if cnt == n {
        dbg!(&mins);
        return mins.iter().map(|e| e.0).sum::<i32>() + 1;
    }

    let mut res = 0;
    for v in 0..n {
        if v != root && comp[group[v]] {
            res += mins[v].0;
        }
    }

    let mut n_edges: Vec<(usize, usize, i32)> = vec![];
    for (f, t, w) in edges.iter() {
        let gf = group[*f];
        let gt = group[*t];
        if gf == gt {
            continue;
        }
        n_edges.push((gf, gt, if comp[gt] { w - mins[*t].0 } else { *w }));
    }
    res + chu_liu_edmond(cnt, n_edges, group[root])
}

#[cfg(test)]
mod test {
    use super::chu_liu_edmond;

    #[test]
    fn test_chu_liu() {
        let edges = &[(0, 1, 1), (2, 0, 1), (1, 2, 1), (0, 3, 1)];
        let edges = &[
            (0, 3, 8),
            (0, 1, 2),
            (0, 2, 10),
            (1, 3, 2),
            (2, 1, 1),
            (2, 4, 1),
            (3, 2, 8),
            (3, 4, 3),
            (4, 5, 1),
            (5, 2, 2),
        ];
        let res = chu_liu_edmond(6, edges.to_vec(), 0);
        dbg!(res);
    }
}
