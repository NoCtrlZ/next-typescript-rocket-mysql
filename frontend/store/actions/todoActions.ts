import { GetTodosSuccess, Todo } from "./interface";
import { dummy } from "../../utils/sample-data"

//types
export const GET_TODOS_SUCCESS = "GET_TODOS_SUCCESS";
export const GET_TODOS_FAILURE = "GET_TODOS_FAILURE";

//action creators
export const getTodosSuccess = (todos: Todo[]) : GetTodosSuccess => ({
    todos,
    type: GET_TODOS_SUCCESS
})

//redux thunk for todos

export const getTodos = () => {
    const response = dummy;

    return (dispatch: (arg0: GetTodosSuccess) => void) => {
        if(response.status === 200) {
            dispatch(getTodosSuccess(response.todos))
        }
    }
}
