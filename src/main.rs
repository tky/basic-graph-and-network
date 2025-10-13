use std::vec;

/// 有向グラフの辺りスト
/// 各辺の始点と終点をそれぞれ別の配列で管理する
/// 使用する領域は2mである
/// ただし配列のindexを1から初めている
struct EdgeList {
    tail: vec::Vec<usize>,
    head: vec::Vec<usize>,
}

struct DirectedGraph {
    edge_first: Vec<usize>,
    edge_next: Vec<usize>,
    rev_edge_first: Vec<usize>,
    rev_edge_next: Vec<usize>,
}

/// pre_label[v] 頂点 v を最初に訪問した順番
/// post_label[v] 頂点 v の探索が完了した順番
/// 頂点vから出る辺の全てがを調べ尽くした順番のこと
struct DfsTime {
    pre_label: Vec<usize>,
    post_label: Vec<usize>,
}

/// edge_firstの添字は頂点の番号
/// edge_first[v]
/// 頂点vを始点とする最初の辺の番号
///
/// edge_firstの添字は辺の番号
/// 同じ始点を持つ次の辺の番号(なければ0)
///
/// edge_first[1]
///   ↓
/// [辺2] ->[辺5] -> 0
/// この状態で新しい辺a = 7が頂点1から出るとすると
/// step1
/// 新しく辺7を
/// edge_nextに追加して辺2を指すようにする
/// edge_first[1]
///           ↓
/// [辺7] ->[辺2] ->[辺5] -> 0
/// step2
/// edge_first[1]を辺7に更新する
/// edge_first[1]
///     ↓
/// [辺7] ->[辺2] ->[辺5] -> 0
///
/// v = tail[a]
/// edge_next[a] = edge_first[v] // 新しく追加した辺が今までの先頭の辺を指すようにする
/// edge_first[v] = a  // 新しく追加した辺を先頭にする
///
/// edge_firstとedge_nextは辺IDを鎖で繋ぐイメージ
/// n: 頂点数
/// m: 辺数
fn dicomp_incidence_list_construct(graph: &EdgeList, n: usize, m: usize) -> DirectedGraph {
    let mut edge_first = vec![0; n + 1];
    let mut edge_next = vec![0; m + 1];
    let mut rev_edge_first = vec![0; n + 1];
    let mut rev_edge_next = vec![0; m + 1];

    // aは辺の番号
    for a in (1..=m).rev() {
        {
            let v = graph.tail[a];
            // edge_next[a]: 新しく追加する辺
            // edge_first[v]: 今までの頂点vを始点とする最初の辺のポインタ
            // edge_next[a] = edge_first[v] // 新しく追加した辺が今までの先頭の辺を指すようにする
            edge_next[a] = edge_first[v];
            edge_first[v] = a;
        }
        {
            let v = graph.head[a];
            rev_edge_next[a] = rev_edge_first[v];
            rev_edge_first[v] = a;
        }
    }
    DirectedGraph {
        edge_first,
        edge_next,
        rev_edge_first,
        rev_edge_next,
    }
}

/// 深さ優先探索
/// 頂点vを始点として探索を行う
/// n: 頂点数
/// v: 探索の始点
fn dfs(edge: &EdgeList, graph: &DirectedGraph, n: usize, v: usize) -> DfsTime {
    let mut pre_label = vec![0; n + 1];
    let mut post_label = vec![0; n + 1];

    // 先行順のラベル
    let mut k: usize = 1;
    // 後行順のラベル
    let mut j: usize = 1;

    /// v: 探索の始点
    fn go(
        v: usize,
        pre_label: &mut Vec<usize>,
        post_label: &mut Vec<usize>,
        edge: &EdgeList,
        graph: &DirectedGraph,
        k: &mut usize,
        j: &mut usize,
    ) {
        pre_label[v] = *k;
        *k += 1;

        // aはvを始点とする辺のリストの先頭の辺
        let mut a = graph.edge_first[v];
        while a != 0 {
            // aの終点
            let w = edge.head[a];
            if pre_label[w] == 0 {
                go(w, pre_label, post_label, edge, graph, k, j);
            }
            a = graph.edge_next[a];
        }
        post_label[v] = *j;
        *j += 1;
    }

    go(
        v,
        &mut pre_label,
        &mut post_label,
        edge,
        graph,
        &mut k,
        &mut j,
    );

    DfsTime {
        pre_label,
        post_label,
    }
}

fn main() {
    // 配列のindexを1から始めるため、先頭にダミーで0を入れておく
    let graph = EdgeList {
        tail: vec![0, 1, 1, 6, 6, 4, 5, 3, 2, 4],
        head: vec![0, 2, 5, 2, 5, 1, 4, 6, 3, 3],
    };
    let directed_graph = dicomp_incidence_list_construct(&graph, 6, 9);

    // 結果を表示
    println!("edge_first: {:?}", directed_graph.edge_first);
    println!("edge_next: {:?}", directed_graph.edge_next);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dicomp_incidence_list_construct() {
        let graph = EdgeList {
            tail: vec![0, 1, 1, 6, 6, 4, 5, 3, 2, 4],
            head: vec![0, 2, 5, 2, 5, 1, 4, 6, 3, 3],
        };
        let directed_graph = dicomp_incidence_list_construct(&graph, 6, 9);

        assert_eq!(directed_graph.edge_first, vec![0, 1, 8, 7, 5, 6, 3]);
        assert_eq!(directed_graph.edge_next, vec![0, 2, 0, 4, 0, 9, 0, 0, 0, 0]);
        assert_eq!(directed_graph.rev_edge_first, vec![0, 5, 1, 8, 6, 2, 7]);
        assert_eq!(
            directed_graph.rev_edge_next,
            vec![0, 3, 4, 0, 0, 0, 0, 0, 9, 0]
        );
    }
    #[test]
    fn self_loop() {
        let g = EdgeList {
            tail: vec![0, 2],
            head: vec![0, 2],
        };
        let dg = dicomp_incidence_list_construct(&g, 3, 1);
        assert_eq!(dg.edge_first, vec![0, 0, 1, 0]);
        assert_eq!(dg.edge_next, vec![0, 0]);
        assert_eq!(dg.rev_edge_first, vec![0, 0, 1, 0]);
        assert_eq!(dg.rev_edge_next, vec![0, 0]);
    }

    #[test]
    fn multi_edges_same_pair() {
        let g = EdgeList {
            tail: vec![0, 1, 1],
            head: vec![0, 2, 2],
        };
        let dg = dicomp_incidence_list_construct(&g, 2, 2);
        assert_eq!(dg.edge_first, vec![0, 1, 0]);
        assert_eq!(dg.edge_next, vec![0, 2, 0]);
        assert_eq!(dg.rev_edge_first, vec![0, 0, 1]);
        assert_eq!(dg.rev_edge_next, vec![0, 2, 0]);
    }

    #[test]
    fn preserves_input_order_per_tail() {
        // a=1..4 を (1→4),(1→3),(1→2),(1→5) とする
        let g = EdgeList {
            tail: vec![0, 1, 1, 1, 1],
            head: vec![0, 4, 3, 2, 5],
        };
        let dg = dicomp_incidence_list_construct(&g, 5, 4);
        assert_eq!(dg.edge_first[1], 1);
        assert_eq!(dg.edge_next, vec![0, 2, 3, 4, 0]); // 1→2→3→4
    }

    #[test]
    fn dfs_test() {
        let graph = EdgeList {
            tail: vec![0, 1, 1, 2, 6, 4, 5, 3, 2, 3],
            head: vec![0, 2, 5, 6, 5, 1, 4, 6, 3, 4],
        };
        let directed_graph = dicomp_incidence_list_construct(&graph, 6, 9);
        let result = dfs(&graph, &directed_graph, 6, 1);
        assert_eq!(result.pre_label, vec![0, 1, 2, 6, 5, 4, 3]);
        assert_eq!(result.post_label, vec![0, 6, 5, 4, 1, 2, 3]);
    }
    #[test]
    fn dfs_linear() {
        let graph = EdgeList {
            tail: vec![0, 1, 2],
            head: vec![0, 2, 3],
        };
        let directed_graph = dicomp_incidence_list_construct(&graph, 3, 2);
        let result = dfs(&graph, &directed_graph, 3, 1);

        // 訪問順は 1,2,3
        assert_eq!(result.pre_label, vec![0, 1, 2, 3]);
        // 帰りがけ順は逆順
        assert_eq!(result.post_label, vec![0, 3, 2, 1]);
    }
    /// 分岐を持つ木構造: 1 -> 2, 1 -> 3
    #[test]
    fn dfs_branching() {
        let graph = EdgeList {
            tail: vec![0, 1, 1],
            head: vec![0, 2, 3],
        };
        let directed_graph = dicomp_incidence_list_construct(&graph, 3, 2);
        let result = dfs(&graph, &directed_graph, 3, 1);

        // 先に 2, 次に 3 を探索すると仮定
        assert_eq!(result.pre_label[1], 1);
        assert!(result.pre_label[2] < result.pre_label[3]);
        assert!(result.post_label[1] > result.post_label[2]);
        assert!(result.post_label[1] > result.post_label[3]);
    }

    #[test]
    fn dfs_cycle() {
        let graph = EdgeList {
            tail: vec![0, 1, 2, 3],
            head: vec![0, 2, 3, 1],
        };
        let directed_graph = dicomp_incidence_list_construct(&graph, 3, 3);
        let result = dfs(&graph, &directed_graph, 3, 1);

        // すべて訪問できている（pre_labelが全て非ゼロ）
        assert!(result.pre_label[1..=3].iter().all(|&x| x > 0));
    }
    #[test]
    fn dfs_disconnected() {
        let graph = EdgeList {
            tail: vec![0, 1, 3],
            head: vec![0, 2, 4],
        };
        let directed_graph = dicomp_incidence_list_construct(&graph, 4, 2);
        let result = dfs(&graph, &directed_graph, 4, 1);
        assert_eq!(result.pre_label, vec![0, 1, 2, 0, 0]);
        assert_eq!(result.post_label, vec![0, 2, 1, 0, 0]);
    }
}
