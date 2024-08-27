import { gql } from '@/graphql/generated/gql.ts';

export const LIST_TASKS = gql(`
  query ListTasks($viewType: ViewType!, $epoch: InputEpoch) {
    tasks(filter: {
      viewFilter: {
        type: $viewType,
        epoch: $epoch,
      }
    }) {
      id
      title
      cost
      isCompleted
      scheduledOn {
        type
        date
      }
    }
  }
`);
export const CREATE_TASK = gql(`
  mutation CreateTask($title: String!, $cost: Int, $scheduledOn: InputEpoch) {
    createTask(input: {
      title: $title
      cost: $cost
      scheduledOn: $scheduledOn
    }) {
      id
      title
      cost
      isCompleted
    }
  }
`);
export const UPDATE_TASK = gql(`
  mutation UpdateTask($input: UpdateTaskInput!) {
    updateTask(input: $input) {
      id
      title
      cost
      isCompleted
    }
  }
`);
export const DELETE_TASK = gql(`
  mutation DeleteTask($id: UUID!) {
    deleteTask(id: $id)
  }
`);
