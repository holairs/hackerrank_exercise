export function solution(str: string, ending: string): boolean {
  const response = str.endsWith(ending);
  return response
}

console.log(solution('abcde', 'cdes'))
