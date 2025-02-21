function bfs(n: number, m: number, edges: number[][], s: number): number[] {
  const graph: number[][] = Array.from({ length: n + 1 }, () => []);
  for (const [u, v] of edges) {
    graph[u].push(v);
    graph[v].push(u);
  }

  const dist: number[] = Array(n + 1).fill(-1);
  dist[s] = 0;

  const q: number[] = [s];
  while (q.length > 0) {
    const current = q.shift()!;
    for (const n of graph[current]) {
      if (dist[n] === -1) {
        dist[n] = dist[current] + 6; // each edge = 6
        q.push(n);
      }
    }
  }

  // skipping the start node....
  const result: number[] = [];
  for (let i = 1; i <= n; i++) {
    if (i !== s) {
      result.push(dist[i]);
    }
  }
  return result;
}
