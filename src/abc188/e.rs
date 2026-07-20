use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        xy: [(Usize1, Usize1); m],
    };

    let mut next_town_list_from_town = vec![Vec::<usize>::new(); n];
    for (x, y) in xy {
        next_town_list_from_town[x].push(y);
    }

    let mut lowest_price_on_route_from_town = vec![i32::MAX; n];
    for town_idx in 0..n {
        for next_town_idx in next_town_list_from_town[town_idx].iter() {
            lowest_price_on_route_from_town[*next_town_idx] = lowest_price_on_route_from_town
                [*next_town_idx]
                .min(lowest_price_on_route_from_town[town_idx].min(a[town_idx]));
        }
    }

    let mut result = i32::MIN;
    for town_idx in 0..n {
        result = result.max(a[town_idx] - lowest_price_on_route_from_town[town_idx]);
    }

    println!("{}", result);
}
