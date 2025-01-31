const toJadenCase = (value: string) => {
  const split = value.trim().split(" ")
  let converted_words: string[] = []

  for (let i = 0; i < split.length; i++) {
    let newWord = upercaseFirst(split[i])
    converted_words.push(newWord)
  }

  const result = converted_words.join(" ")
  return result
}

const upercaseFirst = (word: string): string => {
  if (word.length === 0) return word;
  return word.charAt(0).toUpperCase() + word.slice(1);
}


console.log(toJadenCase("How can mirrors be real if our eyes aren't real"))


//_______
//ultra solución


const toJadenCaseBetter = (value: string) => {
    return value.replace(/(?<=\s|^)./g, (m) => m.toUpperCase())
}

console.log(toJadenCaseBetter("How can mirrors be real if our eyes aren't real"))

