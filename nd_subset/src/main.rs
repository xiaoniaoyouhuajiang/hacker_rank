
fn nonDivisibleSubset(k: i32, s: &[i32]) -> i32 {
    
    1
}

fn bron_kerbosch<const N: usize>(
    r: &[usize], // 当前团
    p: &[usize], // 候选团
    x: &[usize], // 已访问团
    graph: &[Vec<usize>; N], // 图的邻接表
    size: usize, // 当前团的大小
    max_size: &mut usize, // 最大团的大小
    best_cliques: &mut Vec<Vec<usize>>, // 最大团的集合
) {
    if p.is_empty() && x.is_empty() {
        if size > *max_size {
            *max_size = size;
            *best_cliques = r.to_vec();
        }
        return;
    }

    for &v in p {
        let mut new_p = Vec::new();
        let mut new_x = Vec::new();
        for &u in p {
            if graph[v][u] {
                new_p.push(u);
            }
        }
        for &u in x {
            if graph[v][u] {
                new_x.push(u);
            }
        }

        // 递归搜索
        bron_kerbosch::<N>(
            r.iter().copied().collect::<Vec<usize>>().as_slice(),
            &new_p,
            &new_x,
            graph,
            size + 1,
            max_size,
            best_cliques,
        );

        // 回溯
        p.retain(|&u| u != v);
        x.push(v);
    }
}


fn main() {
    println!("Hello, world!");
}
