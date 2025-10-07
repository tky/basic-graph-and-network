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
}
