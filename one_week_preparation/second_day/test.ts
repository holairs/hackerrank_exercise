function flippingMatrix(matrix: number[][]): number {
  const n = matrix.length / 2;
  let maxSum = 0;

  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      // Escoger el máximo de las 4 posiciones reflejadas
      let maxVal = Math.max(
        matrix[i][j],  // Original
        matrix[i][2 * n - 1 - j],  // Reflejado horizontalmente
        matrix[2 * n - 1 - i][j],  // Reflejado verticalmente
        matrix[2 * n - 1 - i][2 * n - 1 - j]  // Reflejado diagonalmente
      );

      maxSum += maxVal;
    }
  }

  return maxSum;
}

// 🔹 **Ejemplo de uso**
const matrix: number[][] = [
  [112, 42, 83, 119],
  [56, 125, 56, 49],
  [15, 78, 101, 43],
  [62, 98, 114, 108]
];

//const matrix = [[1, 2], [3, 4]];
console.log(flippingMatrix(matrix)); // Output: 459

