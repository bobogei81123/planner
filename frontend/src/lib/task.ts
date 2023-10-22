import { graphql } from '$src/gql';
import type { CreateTaskMutation, CreateTaskMutationVariables } from '$src/gql/graphql';
import type { TypedDocumentNode, VariablesOf } from '@graphql-typed-document-node/core';
import { getContextClient, mutationStore, queryStore, type OperationResultStore } from '@urql/svelte';


export const CREATE_TASK = graphql(`
  mutation CreateTask($title: String!, $iteration: UUID) {
    createTask(input: { title: $title, iteration: $iteration }) {
      id
      title
      status
      point
    }
  }
`);

// function createTask(title: string, planned_for?: string):
//   OperationResultStore<CreateTaskMutation, CreateTaskMutationVariables> {
//   return mutationStore({
//     client: getContextClient(),
//     query: graphql(`
// 				mutation CreateTask($title: String!, $planned_for: UUID) {
// 					createTask(input: { title: $title, planned_for: $planned_for }) {
// 						id
// 						title
// 						status
// 						point
// 					}
// 				}
// 			`),
//     variables: { title, planned_for }
//   });
// }
