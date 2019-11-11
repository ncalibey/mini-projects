import { FetchTodosAction, DeleteTodoAction } from './todos';

export enum ActionTypes {
  FetchTodos,
  DeleteTodo,
}

export type Action = FetchTodosAction | DeleteTodoAction;
