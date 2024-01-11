/* eslint-disable */
import * as types from './graphql';
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

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
    "\n        mutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {\n          updateTask(input: { id: $id, status: $status }) {\n            id\n            status\n          }\n        }\n      ": types.UpdateTaskStatusDocument,
    "\n        mutation UpdateTaskTitle($id: UUID!, $title: String) {\n          updateTask(input: { id: $id, title: $title }) {\n            id\n            title\n          }\n        }\n      ": types.UpdateTaskTitleDocument,
    "\n        mutation UpdateTaskPoint($id: UUID!, $point: Int) {\n          updateTask(input: { id: $id, point: $point }) {\n            id\n            point\n          }\n        }\n      ": types.UpdateTaskPointDocument,
    "\n        mutation UpdateTaskPlannedOn($id: UUID!, $plannedOn: NaiveDate) {\n          updateTask(input: { id: $id, plannedOn: $plannedOn }) {\n            id\n            point\n          }\n        }\n      ": types.UpdateTaskPlannedOnDocument,
    "\n        mutation UpdateTaskIterations($id: UUID!, $iterations: [UUID!]) {\n          updateTask(input: { id: $id, iterations: $iterations }) {\n            id\n            iterations {\n              id\n              name\n            }\n          }\n        }\n      ": types.UpdateTaskIterationsDocument,
    "\n        mutation DeleteTask($id: UUID!) {\n          deleteTask(id: $id)\n        }\n      ": types.DeleteTaskDocument,
    "\n  mutation CreateTask($input: CreateTaskInput!) {\n    createTask(input: $input) {\n      id\n      title\n      status\n      point\n    }\n  }\n": types.CreateTaskDocument,
    "\n      query allIterations {\n        iterations {\n          id\n          name\n        }\n      }\n    ": types.AllIterationsDocument,
    "\n        mutation createIteration($name: String) {\n          createIteration(input: { name: $name }) {\n            id\n            name\n          }\n        }\n      ": types.CreateIterationDocument,
    "\n      query allTasksInIteration($id: UUID!) {\n        iteration(id: $id) {\n          id\n          name\n          tasks {\n            id\n            title\n            status\n            point\n            iterations {\n              id\n              name\n            }\n          }\n        }\n      }\n    ": types.AllTasksInIterationDocument,
    "\n      query allTasks($filter: TaskFilter) {\n        tasks(filter: $filter) {\n          id\n          title\n          status\n          point\n          plannedOn\n          iterations {\n            id\n            name\n          }\n        }\n      }\n    ": types.AllTasksDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n        mutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {\n          updateTask(input: { id: $id, status: $status }) {\n            id\n            status\n          }\n        }\n      "): (typeof documents)["\n        mutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {\n          updateTask(input: { id: $id, status: $status }) {\n            id\n            status\n          }\n        }\n      "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n        mutation UpdateTaskTitle($id: UUID!, $title: String) {\n          updateTask(input: { id: $id, title: $title }) {\n            id\n            title\n          }\n        }\n      "): (typeof documents)["\n        mutation UpdateTaskTitle($id: UUID!, $title: String) {\n          updateTask(input: { id: $id, title: $title }) {\n            id\n            title\n          }\n        }\n      "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n        mutation UpdateTaskPoint($id: UUID!, $point: Int) {\n          updateTask(input: { id: $id, point: $point }) {\n            id\n            point\n          }\n        }\n      "): (typeof documents)["\n        mutation UpdateTaskPoint($id: UUID!, $point: Int) {\n          updateTask(input: { id: $id, point: $point }) {\n            id\n            point\n          }\n        }\n      "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n        mutation UpdateTaskPlannedOn($id: UUID!, $plannedOn: NaiveDate) {\n          updateTask(input: { id: $id, plannedOn: $plannedOn }) {\n            id\n            point\n          }\n        }\n      "): (typeof documents)["\n        mutation UpdateTaskPlannedOn($id: UUID!, $plannedOn: NaiveDate) {\n          updateTask(input: { id: $id, plannedOn: $plannedOn }) {\n            id\n            point\n          }\n        }\n      "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n        mutation UpdateTaskIterations($id: UUID!, $iterations: [UUID!]) {\n          updateTask(input: { id: $id, iterations: $iterations }) {\n            id\n            iterations {\n              id\n              name\n            }\n          }\n        }\n      "): (typeof documents)["\n        mutation UpdateTaskIterations($id: UUID!, $iterations: [UUID!]) {\n          updateTask(input: { id: $id, iterations: $iterations }) {\n            id\n            iterations {\n              id\n              name\n            }\n          }\n        }\n      "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n        mutation DeleteTask($id: UUID!) {\n          deleteTask(id: $id)\n        }\n      "): (typeof documents)["\n        mutation DeleteTask($id: UUID!) {\n          deleteTask(id: $id)\n        }\n      "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateTask($input: CreateTaskInput!) {\n    createTask(input: $input) {\n      id\n      title\n      status\n      point\n    }\n  }\n"): (typeof documents)["\n  mutation CreateTask($input: CreateTaskInput!) {\n    createTask(input: $input) {\n      id\n      title\n      status\n      point\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n      query allIterations {\n        iterations {\n          id\n          name\n        }\n      }\n    "): (typeof documents)["\n      query allIterations {\n        iterations {\n          id\n          name\n        }\n      }\n    "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n        mutation createIteration($name: String) {\n          createIteration(input: { name: $name }) {\n            id\n            name\n          }\n        }\n      "): (typeof documents)["\n        mutation createIteration($name: String) {\n          createIteration(input: { name: $name }) {\n            id\n            name\n          }\n        }\n      "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n      query allTasksInIteration($id: UUID!) {\n        iteration(id: $id) {\n          id\n          name\n          tasks {\n            id\n            title\n            status\n            point\n            iterations {\n              id\n              name\n            }\n          }\n        }\n      }\n    "): (typeof documents)["\n      query allTasksInIteration($id: UUID!) {\n        iteration(id: $id) {\n          id\n          name\n          tasks {\n            id\n            title\n            status\n            point\n            iterations {\n              id\n              name\n            }\n          }\n        }\n      }\n    "];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n      query allTasks($filter: TaskFilter) {\n        tasks(filter: $filter) {\n          id\n          title\n          status\n          point\n          plannedOn\n          iterations {\n            id\n            name\n          }\n        }\n      }\n    "): (typeof documents)["\n      query allTasks($filter: TaskFilter) {\n        tasks(filter: $filter) {\n          id\n          title\n          status\n          point\n          plannedOn\n          iterations {\n            id\n            name\n          }\n        }\n      }\n    "];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;