/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    fn bfs_from(&self, start: usize,visit_order:&mut Vec<usize>) {

        // println!("开始访问{}",start);
        if !visit_order.contains(&start) {
            visit_order.push(start);
        }

        let len = visit_order.len();
        // 访问孩子们
        let neighbors =  &self.adj[start];
        // println!("发现 {} 的孩子们 {:?}",start,neighbors);
        // // 全部访问
        // visit_order.extend(neighbors);
        for n in neighbors {
            // self.bfs_from(n, visit_order);
            
            if !visit_order.contains(n){
                visit_order.push(*n);
            }
        }
        //有新的孩子
        // if visit_order.len() > len {
            // println!("发现新 {} 的孩子们 {:?}",start,neighbors);
        for i in len..visit_order.len(){
            // println!("开始新 {} 的孩子们 {:?}",start,visit_order[i]);
            self.bfs_from(visit_order[i], visit_order);
        }
        // }
		//TODO
    }

    fn bfs_with_rec(&self, start: usize) -> Vec<usize> {

        let mut visit_order = vec![];
        self.bfs_from(start, &mut visit_order);
        visit_order
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO

        let mut visit_order = vec![];
        //核心，用个缓存队列
        let mut queue = std::collections::VecDeque::new();
        let mut visited = vec![false; self.adj.len()];

        queue.push_back(start);
        visited[start] = true;

        while let Some(node) = queue.pop_front() {
            visit_order.push(node);
            //处理孩子
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    queue.push_back(neighbor);
                    visited[neighbor]=true;
                }
            }
        }
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

