/*

There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.

    For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.

Return true if you can finish all courses. Otherwise, return false.
*/



use std::collections::HashSet;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        
        let mut graph = vec![vec![]; num_courses as usize];

        for pre in prerequisites.iter() {
            graph[pre[0] as usize].push(pre[1]);
        }

        let mut set =  HashSet::new();
        let mut vis = vec![false; num_courses as usize];

        !dfs_util(&graph, &mut vis, &mut set)

    }


   
}

 pub fn dfs_util(graph: &Vec<Vec<i32>>, vis: &mut Vec<bool>, in_process: &mut HashSet<i32>) -> bool {
        let mut result = false;
        for i in 0..graph.len() {
            if(!vis[i]) {
                result = result | dfs(i as i32, graph, vis, in_process);
            }
        }
        return result;
    }

    fn dfs(x: i32, graph: &Vec<Vec<i32>>, vis: &mut Vec<bool>, in_process: &mut HashSet<i32>) -> bool {

        if in_process.contains(&x) {
            return true;
        }

        if vis[x as usize] {
            return false;
        }


        in_process.insert(x);
       
        
        let mut result = false;
        for i in 0..graph[x as usize].len() {
            result = result | dfs(graph[x as usize][i], graph, vis, in_process);
        }

        in_process.remove(&x);
         vis[x as usize] = true;
        
        return result;
    }
