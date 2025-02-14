function palindromeIndex(s: string): number {

  function isPalindrome(str: string): boolean {
    return str === str.split("").reverse().join("");
  }

  if (isPalindrome(s)) return -1;

  let left = 0;
  let right = s.length - 1;

  while (left < right) {
    if (s[left] !== s[right]) {

      if (isPalindrome(s.slice(0, left) + s.slice(left + 1))) {
        return left;
      }

      if (isPalindrome(s.slice(0, right) + s.slice(right + 1))) {
        return right;
      }
      return -1;
    }
    left++;
    right--;
  }

  return -1;
}

// Prueba con el ejemplo "bcbc"
let s = "aaab";
let result = palindromeIndex(s);
console.log(result); // Output esperado: 1
