use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut path_map = vec![Vec::<usize>::new(); n];
    for (u, v) in uv {
        path_map[u].push(v);
        path_map[v].push(u);
    }

    let mut group_and_color_from_vertex = vec![Option::<(usize, usize)>::None; n];
    let mut next_group = 0_usize;
    for i in 0..n {
        if group_and_color_from_vertex[i].is_some() {
            continue;
        }
        let mut stack = vec![(i, (next_group, 0_usize))];
        next_group += 1;
        while let Some((j, (group, color))) = stack.pop() {
            if let Some((cur_group, cur_color)) = group_and_color_from_vertex[j] {
                if cur_group != group || cur_color != color {
                    println!("0");
                    return;
                }
                continue;
            }
            group_and_color_from_vertex[j] = Some((group, color));
            for k in path_map[j].iter() {
                stack.push((*k, (group, 1 - color)));
            }
        }
    }

    let mut count_from_group_and_color = HashMap::<(usize, usize), usize>::new();
    for i in 0..n {
        if let Some(group_and_color) = group_and_color_from_vertex[i] {
            count_from_group_and_color.insert(
                group_and_color,
                *count_from_group_and_color
                    .get(&group_and_color)
                    .unwrap_or(&0)
                    + 1,
            );
        }
    }

    let mut result = 0_usize;
    for i in 0..n {
        if let Some(group_and_color) = group_and_color_from_vertex[i] {
            result += n - count_from_group_and_color.get(&group_and_color).unwrap();
        } else {
            result += n - 1;
        }
    }
    result /= 2;
    result -= m;

    println!("{}", result);
}
