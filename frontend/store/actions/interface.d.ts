//todos object
export interface Todo {
    id: string;
    title: string;
}

export interface GetTodosSuccess {
    todos: Todo[];
    type: GET_TODOS_SUCCESS;
}