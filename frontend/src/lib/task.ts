import { graphql } from '$src/gql';

export const CREATE_TASK = graphql(`
  mutation CreateTask($input: CreateTaskInput!) {
    createTask(input: $input) {
      id
      title
      status
      point
    }
  }
`);
