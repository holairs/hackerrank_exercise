/*
 * Complete the getMoneySpent function below.
 */
function getMoneySpent(keyboards, drives, b) {
  let max_spent = -1;

  for (let i = 0; i < keyboards.length; i++) {
    for (let j = 0; j < drives.length; j++) {
      const current_spent = keyboards[i] + drives[j];
      console.log(current_spent, b)
      if (current_spent <= b) {
        max_spent = Math.max(max_spent, current_spent);
      }
    }
  }

  return max_spent;
}


const b = 60;
const keyboards = [40, 50, 60];
const drives = [5, 8, 12];
console.log(getMoneySpent(keyboards, drives, b));

