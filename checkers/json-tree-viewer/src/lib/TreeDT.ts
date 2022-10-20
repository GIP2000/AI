export type Tree<T> = {
    val: T,
    next: Tree<T>[]
}

export type RTTree = {
    h_val: number,
    alpha: number,
    beta: number,
    mv: any,
    is_max: boolean,
    pruned: boolean,
}

