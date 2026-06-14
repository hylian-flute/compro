use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uvw: [(Usize1, Usize1, usize); n - 1],
    };

    let mut map = vec![Vec::<(usize, usize)>::new(); n];
    for (u, v, w) in uvw {
        map[u].push((v, w));
        map[v].push((u, w));
    }

    let mut colors = vec![Option::<usize>::None; n];
    let mut stack = vec![(0_usize, 0_usize)];
    while let Some((vertex, color)) = stack.pop() {
        if colors[vertex].is_none() {
            colors[vertex] = Some(color);
            for (new_vertex, w) in map[vertex].iter() {
                stack.push((*new_vertex, (color + w) % 2));
            }
        }
    }

    for color in colors {
        println!("{}", color.unwrap());
    }
}
