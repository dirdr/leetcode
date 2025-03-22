from typing import List, Tuple, Set

class Solution:
    def countCompleteComponents(self, n: int, edges: List[List[int]]) -> int:
        adj = [[] for i in range(n)]
        for edge in edges:
            adj[edge[0]].append(edge[1])
            adj[edge[1]].append(edge[0])
        visited = set()

        def dfs(current: int, adj: List[List[int]], visited: Set[int]) -> Tuple[int, int]:
            visited.add(current)
            nodes = 1
            edges = len(adj[current])
            for neighbor in adj[current]:
                if neighbor not in visited:
                    n, e = dfs(neighbor, adj, visited)
                    nodes += n
                    edges += e
            return nodes, edges

        answer = 0
        for i in range(n):
            if i not in visited:
                nodes, edges = dfs(i, adj, visited)
                if edges == nodes * (nodes - 1):
                    answer += 1
        return answer
