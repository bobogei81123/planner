export function checkNonNull<T>(x: T) : NonNullable<T> {
  if (x === null || x === undefined) {
    throw new Error(`the variable is {x === null ? 'null' : 'undefined'}`)
  }
  return x
}

export function orElse<T>(x: T, f: NonNullable<T>): NonNullable<T> {
  return x ?? f
}
