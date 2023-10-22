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
    "\n\t\t\t\tmutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {\n\t\t\t\t\tupdateTask(input: { id: $id, status: $status }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tstatus\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t": types.UpdateTaskStatusDocument,
    "\n\t\t\t\tmutation UpdateTaskTitle($id: UUID!, $title: String) {\n\t\t\t\t\tupdateTask(input: { id: $id, title: $title }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\ttitle\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t": types.UpdateTaskTitleDocument,
    "\n\t\t\t\tmutation UpdateTaskPoint($id: UUID!, $point: Int) {\n\t\t\t\t\tupdateTask(input: { id: $id, point: $point }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tpoint\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t": types.UpdateTaskPointDocument,
    "\n\t\t\t\tmutation UpdateTaskIterations($id: UUID!, $iterations: [UUID!]) {\n\t\t\t\t\tupdateTask(input: { id: $id, iterations: $iterations }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\titerations {\n\t\t\t\t\t\t\tid\n\t\t\t\t\t\t\tname\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t": types.UpdateTaskIterationsDocument,
    "\n\t\t\t\tmutation DeleteTask($id: UUID!) {\n\t\t\t\t\tdeleteTask(id: $id)\n\t\t\t\t}\n\t\t\t": types.DeleteTaskDocument,
    "\n\t\t\tquery allIterations {\n\t\t\t\titerations {\n\t\t\t\t\tid\n\t\t\t\t\tname\n\t\t\t\t}\n\t\t\t}\n\t\t": types.AllIterationsDocument,
    "\n  mutation CreateTask($title: String!, $iteration: UUID) {\n    createTask(input: { title: $title, iteration: $iteration }) {\n      id\n      title\n      status\n      point\n    }\n  }\n": types.CreateTaskDocument,
    "\n\t\t\t\tmutation createIteration($name: String) {\n\t\t\t\t\tcreateIteration(input: { name: $name }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tname\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t": types.CreateIterationDocument,
    "\n\t\t\tquery allTasksInIteration($id: UUID!) {\n\t\t\t\titeration(id: $id) {\n\t\t\t\t\tid\n\t\t\t\t\tname\n\t\t\t\t\ttasks {\n\t\t\t\t\t\tid\n\t\t\t\t\t\ttitle\n\t\t\t\t\t\tstatus\n\t\t\t\t\t\tpoint\n            iterations {\n              id\n              name\n            }\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t": types.AllTasksInIterationDocument,
    "\n\t\t\tquery allTasks {\n\t\t\t\ttasks {\n\t\t\t\t\tid\n\t\t\t\t\ttitle\n\t\t\t\t\tstatus\n\t\t\t\t\tpoint\n          iterations {\n            id\n            name\n          }\n\t\t\t\t}\n\t\t\t}\n\t\t": types.AllTasksDocument,
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
export function graphql(source: "\n\t\t\t\tmutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {\n\t\t\t\t\tupdateTask(input: { id: $id, status: $status }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tstatus\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"): (typeof documents)["\n\t\t\t\tmutation UpdateTaskStatus($id: UUID!, $status: TaskStatus!) {\n\t\t\t\t\tupdateTask(input: { id: $id, status: $status }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tstatus\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\t\t\t\tmutation UpdateTaskTitle($id: UUID!, $title: String) {\n\t\t\t\t\tupdateTask(input: { id: $id, title: $title }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\ttitle\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"): (typeof documents)["\n\t\t\t\tmutation UpdateTaskTitle($id: UUID!, $title: String) {\n\t\t\t\t\tupdateTask(input: { id: $id, title: $title }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\ttitle\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\t\t\t\tmutation UpdateTaskPoint($id: UUID!, $point: Int) {\n\t\t\t\t\tupdateTask(input: { id: $id, point: $point }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tpoint\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"): (typeof documents)["\n\t\t\t\tmutation UpdateTaskPoint($id: UUID!, $point: Int) {\n\t\t\t\t\tupdateTask(input: { id: $id, point: $point }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tpoint\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\t\t\t\tmutation UpdateTaskIterations($id: UUID!, $iterations: [UUID!]) {\n\t\t\t\t\tupdateTask(input: { id: $id, iterations: $iterations }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\titerations {\n\t\t\t\t\t\t\tid\n\t\t\t\t\t\t\tname\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"): (typeof documents)["\n\t\t\t\tmutation UpdateTaskIterations($id: UUID!, $iterations: [UUID!]) {\n\t\t\t\t\tupdateTask(input: { id: $id, iterations: $iterations }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\titerations {\n\t\t\t\t\t\t\tid\n\t\t\t\t\t\t\tname\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\t\t\t\tmutation DeleteTask($id: UUID!) {\n\t\t\t\t\tdeleteTask(id: $id)\n\t\t\t\t}\n\t\t\t"): (typeof documents)["\n\t\t\t\tmutation DeleteTask($id: UUID!) {\n\t\t\t\t\tdeleteTask(id: $id)\n\t\t\t\t}\n\t\t\t"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\t\t\tquery allIterations {\n\t\t\t\titerations {\n\t\t\t\t\tid\n\t\t\t\t\tname\n\t\t\t\t}\n\t\t\t}\n\t\t"): (typeof documents)["\n\t\t\tquery allIterations {\n\t\t\t\titerations {\n\t\t\t\t\tid\n\t\t\t\t\tname\n\t\t\t\t}\n\t\t\t}\n\t\t"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateTask($title: String!, $iteration: UUID) {\n    createTask(input: { title: $title, iteration: $iteration }) {\n      id\n      title\n      status\n      point\n    }\n  }\n"): (typeof documents)["\n  mutation CreateTask($title: String!, $iteration: UUID) {\n    createTask(input: { title: $title, iteration: $iteration }) {\n      id\n      title\n      status\n      point\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\t\t\t\tmutation createIteration($name: String) {\n\t\t\t\t\tcreateIteration(input: { name: $name }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tname\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"): (typeof documents)["\n\t\t\t\tmutation createIteration($name: String) {\n\t\t\t\t\tcreateIteration(input: { name: $name }) {\n\t\t\t\t\t\tid\n\t\t\t\t\t\tname\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\t\t\tquery allTasksInIteration($id: UUID!) {\n\t\t\t\titeration(id: $id) {\n\t\t\t\t\tid\n\t\t\t\t\tname\n\t\t\t\t\ttasks {\n\t\t\t\t\t\tid\n\t\t\t\t\t\ttitle\n\t\t\t\t\t\tstatus\n\t\t\t\t\t\tpoint\n            iterations {\n              id\n              name\n            }\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t"): (typeof documents)["\n\t\t\tquery allTasksInIteration($id: UUID!) {\n\t\t\t\titeration(id: $id) {\n\t\t\t\t\tid\n\t\t\t\t\tname\n\t\t\t\t\ttasks {\n\t\t\t\t\t\tid\n\t\t\t\t\t\ttitle\n\t\t\t\t\t\tstatus\n\t\t\t\t\t\tpoint\n            iterations {\n              id\n              name\n            }\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\t\t\tquery allTasks {\n\t\t\t\ttasks {\n\t\t\t\t\tid\n\t\t\t\t\ttitle\n\t\t\t\t\tstatus\n\t\t\t\t\tpoint\n          iterations {\n            id\n            name\n          }\n\t\t\t\t}\n\t\t\t}\n\t\t"): (typeof documents)["\n\t\t\tquery allTasks {\n\t\t\t\ttasks {\n\t\t\t\t\tid\n\t\t\t\t\ttitle\n\t\t\t\t\tstatus\n\t\t\t\t\tpoint\n          iterations {\n            id\n            name\n          }\n\t\t\t\t}\n\t\t\t}\n\t\t"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;