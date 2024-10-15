/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  query ListTasks($viewType: ViewType!, $epoch: InputEpoch) {\n    tasks(filter: {\n      viewFilter: {\n        type: $viewType,\n        epoch: $epoch,\n      }\n    }) {\n      id\n      title\n      cost\n      isCompleted\n      scheduledOn {\n        type\n        date\n      }\n    }\n  }\n": types.ListTasksDocument,
    "\n  mutation CreateTask($title: String!, $cost: Int, $scheduledOn: InputEpoch, $recurringSpec: InputRecurringSpec) {\n    createTask(input: {\n      title: $title\n      cost: $cost\n      scheduledOn: $scheduledOn\n      recurringSpec: $recurringSpec\n    }) {\n      id\n      title\n      cost\n      isCompleted\n    }\n  }\n": types.CreateTaskDocument,
    "\n  mutation UpdateTask($input: UpdateTaskInput!) {\n    updateTask(input: $input) {\n      id\n      title\n      cost\n      isCompleted\n    }\n  }\n": types.UpdateTaskDocument,
    "\n  mutation DeleteTask($id: UUID!) {\n    deleteTask(id: $id)\n  }\n": types.DeleteTaskDocument,
};

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = gql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function gql(source: string): unknown;

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  query ListTasks($viewType: ViewType!, $epoch: InputEpoch) {\n    tasks(filter: {\n      viewFilter: {\n        type: $viewType,\n        epoch: $epoch,\n      }\n    }) {\n      id\n      title\n      cost\n      isCompleted\n      scheduledOn {\n        type\n        date\n      }\n    }\n  }\n"): (typeof documents)["\n  query ListTasks($viewType: ViewType!, $epoch: InputEpoch) {\n    tasks(filter: {\n      viewFilter: {\n        type: $viewType,\n        epoch: $epoch,\n      }\n    }) {\n      id\n      title\n      cost\n      isCompleted\n      scheduledOn {\n        type\n        date\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  mutation CreateTask($title: String!, $cost: Int, $scheduledOn: InputEpoch, $recurringSpec: InputRecurringSpec) {\n    createTask(input: {\n      title: $title\n      cost: $cost\n      scheduledOn: $scheduledOn\n      recurringSpec: $recurringSpec\n    }) {\n      id\n      title\n      cost\n      isCompleted\n    }\n  }\n"): (typeof documents)["\n  mutation CreateTask($title: String!, $cost: Int, $scheduledOn: InputEpoch, $recurringSpec: InputRecurringSpec) {\n    createTask(input: {\n      title: $title\n      cost: $cost\n      scheduledOn: $scheduledOn\n      recurringSpec: $recurringSpec\n    }) {\n      id\n      title\n      cost\n      isCompleted\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  mutation UpdateTask($input: UpdateTaskInput!) {\n    updateTask(input: $input) {\n      id\n      title\n      cost\n      isCompleted\n    }\n  }\n"): (typeof documents)["\n  mutation UpdateTask($input: UpdateTaskInput!) {\n    updateTask(input: $input) {\n      id\n      title\n      cost\n      isCompleted\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  mutation DeleteTask($id: UUID!) {\n    deleteTask(id: $id)\n  }\n"): (typeof documents)["\n  mutation DeleteTask($id: UUID!) {\n    deleteTask(id: $id)\n  }\n"];

export function gql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;