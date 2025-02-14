#include <unordered_set>
#include <vector>

int pairs(int k, std::vector<int> arr) {
  std::unordered_set<int> numSet; // 1. Crear un Conjunto Hash
  for (int num : arr) {
    numSet.insert(num); // 2. Poblar el Conjunto Hash
  }

  int count = 0;                // 3. Inicializar contador
  for (int num : arr) {         // 4. Iterar sobre el array
    int target_value = num + k; // 5. Calcular valor objetivo
    if (numSet.count(
            target_value)) { // 6. Verificar existencia en Conjunto Hash
      count++;               // 7. Incrementar contador
    }
  }
  return count; // 8. Retornar contador
}
