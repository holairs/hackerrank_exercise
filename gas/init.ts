function tourDeGasolineras(gasolineras: number[][]): number {
  const n = gasolineras.length;

  for (let inicio = 0; inicio < n; inicio++) {
    let gasolina = 0;
    let posible = true;

    for (let i = 0; i < n; i++) {
      const actual = (inicio + i) % n; // Gasolinera actual

      gasolina += gasolineras[actual][0]; // Ganancia de gasolina
      gasolina -= gasolineras[actual][1]; // Consumo de gasolina

      if (gasolina < 0) {
        posible = false;
        break; // No es posible completar el recorrido
      }
    }

    if (posible) {
      console.log(gasolineras[inicio]);
      return inicio; // Encontramos la gasolinera de inicio
    }
  }

  return -1; // No se encontró ninguna gasolinera de inicio válida
}

// Ejemplo de uso
const gasolineras = [
  [10, 5],
  [5, 10],
  [15, 10]
];

const inicio = tourDeGasolineras(gasolineras);

if (inicio !== -1) {
  console.log("Se puede iniciar el recorrido en la gasolinera:", inicio);
} else {
  console.log("No se puede completar el recorrido desde ninguna gasolinera.");
}
