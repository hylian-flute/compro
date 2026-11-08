use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    };

    let mut next_and_label_vec_from_vtx: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for (u, v, w) in uvw {
        if u != n - 1 {
            next_and_label_vec_from_vtx[u].push((v, w));
        }
        if v != n - 1 {
            next_and_label_vec_from_vtx[v].push((u, w));
        }
    }

    let mut vtx_and_path_idx_stack = vec![(0_usize, 0_usize)];
    let mut score = 0_usize;
    let mut visited = [vec![true], vec![false; n - 1]].concat();
    let mut min_score = usize::MAX;
    while let Some(&(vtx, path_idx)) = vtx_and_path_idx_stack.last() {
        if let Some(&(next_vtx, label)) = next_and_label_vec_from_vtx[vtx].get(path_idx) {
            if visited[next_vtx] {
                vtx_and_path_idx_stack.pop();
                vtx_and_path_idx_stack.push((vtx, path_idx + 1));
            } else {
                score = score ^ label;
                visited[next_vtx] = true;
                vtx_and_path_idx_stack.push((next_vtx, 0));
            }
        } else {
            if vtx == n - 1 {
                min_score = min_score.min(score);
            }
            vtx_and_path_idx_stack.pop();
            visited[vtx] = false;
            if let Some(&(prev_vtx, prev_path_idx)) = vtx_and_path_idx_stack.last() {
                let (_, prev_label) = next_and_label_vec_from_vtx[prev_vtx][prev_path_idx];
                score = score ^ prev_label;
                vtx_and_path_idx_stack.pop();
                vtx_and_path_idx_stack.push((prev_vtx, prev_path_idx + 1));
            }
        }
    }

    println!("{}", min_score);
}
