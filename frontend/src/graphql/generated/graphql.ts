/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * ISO 8601 calendar date without timezone.
   * Format: %Y-%m-%d
   *
   * # Examples
   *
   * * `1994-11-13`
   * * `2000-02-24`
   */
  NaiveDate: { input: string; output: string; }
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: string; output: string; }
};

export type CreateTaskInput = {
  cost?: InputMaybe<Scalars['Int']['input']>;
  recurringSpec?: InputMaybe<InputRecurringSpec>;
  scheduledOn?: InputMaybe<InputEpoch>;
  title: Scalars['String']['input'];
};

export type Epoch = {
  __typename?: 'Epoch';
  date: Scalars['NaiveDate']['output'];
  type: EpochType;
};

export enum EpochType {
  Date = 'DATE',
  Week = 'WEEK'
}

export type InputEpoch = {
  date: Scalars['NaiveDate']['input'];
  type: EpochType;
};

export type InputRecurringPattern = {
  every: Scalars['Int']['input'];
};

export type InputRecurringSpec = {
  pattern: InputRecurringPattern;
  startDate: Scalars['NaiveDate']['input'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  createTask: Task;
  deleteTask: Scalars['UUID']['output'];
  updateTask: Task;
};


export type MutationRootCreateTaskArgs = {
  input: CreateTaskInput;
};


export type MutationRootDeleteTaskArgs = {
  id: Scalars['UUID']['input'];
};


export type MutationRootUpdateTaskArgs = {
  input: UpdateTaskInput;
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  tasks: Array<Task>;
};


export type QueryRootTasksArgs = {
  filter?: InputMaybe<TaskFilter>;
};

export type RecurringPattern = {
  __typename?: 'RecurringPattern';
  every: Scalars['Int']['output'];
};

export type RecurringSpec = {
  __typename?: 'RecurringSpec';
  pattern: RecurringPattern;
  startDate: Scalars['NaiveDate']['output'];
};

export type Task = {
  __typename?: 'Task';
  cost?: Maybe<Scalars['Int']['output']>;
  id: Scalars['UUID']['output'];
  isCompleted: Scalars['Boolean']['output'];
  recurring?: Maybe<RecurringSpec>;
  scheduledOn?: Maybe<Epoch>;
  title: Scalars['String']['output'];
};

export type TaskFilter = {
  viewFilter?: InputMaybe<ViewFilter>;
};

export type UpdateTaskInput = {
  completeDate?: InputMaybe<Scalars['NaiveDate']['input']>;
  cost?: InputMaybe<Scalars['Int']['input']>;
  id: Scalars['UUID']['input'];
  scheduledOn?: InputMaybe<InputEpoch>;
  title?: InputMaybe<Scalars['String']['input']>;
};

export type ViewFilter = {
  epoch?: InputMaybe<InputEpoch>;
  type: ViewType;
};

export enum ViewType {
  Planned = 'PLANNED',
  Scheduled = 'SCHEDULED'
}

export type ListTasksQueryVariables = Exact<{
  viewType: ViewType;
  epoch?: InputMaybe<InputEpoch>;
}>;


export type ListTasksQuery = { __typename?: 'QueryRoot', tasks: Array<{ __typename?: 'Task', id: string, title: string, cost?: number | null, isCompleted: boolean, scheduledOn?: { __typename?: 'Epoch', type: EpochType, date: string } | null, recurring?: { __typename?: 'RecurringSpec', startDate: string, pattern: { __typename?: 'RecurringPattern', every: number } } | null }> };

export type CreateTaskMutationVariables = Exact<{
  title: Scalars['String']['input'];
  cost?: InputMaybe<Scalars['Int']['input']>;
  scheduledOn?: InputMaybe<InputEpoch>;
  recurringSpec?: InputMaybe<InputRecurringSpec>;
}>;


export type CreateTaskMutation = { __typename?: 'MutationRoot', createTask: { __typename?: 'Task', id: string, title: string, cost?: number | null, isCompleted: boolean } };

export type UpdateTaskMutationVariables = Exact<{
  input: UpdateTaskInput;
}>;


export type UpdateTaskMutation = { __typename?: 'MutationRoot', updateTask: { __typename?: 'Task', id: string, title: string, cost?: number | null, isCompleted: boolean, scheduledOn?: { __typename?: 'Epoch', type: EpochType, date: string } | null } };

export type UpdateTaskCompleteDateMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
  completeDate?: InputMaybe<Scalars['NaiveDate']['input']>;
}>;


export type UpdateTaskCompleteDateMutation = { __typename?: 'MutationRoot', updateTask: { __typename?: 'Task', id: string, title: string, cost?: number | null, isCompleted: boolean, scheduledOn?: { __typename?: 'Epoch', type: EpochType, date: string } | null } };

export type DeleteTaskMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
}>;


export type DeleteTaskMutation = { __typename?: 'MutationRoot', deleteTask: string };

export type UpdateTaskEpochMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
  scheduledOn?: InputMaybe<InputEpoch>;
}>;


export type UpdateTaskEpochMutation = { __typename?: 'MutationRoot', updateTask: { __typename?: 'Task', id: string, scheduledOn?: { __typename?: 'Epoch', type: EpochType, date: string } | null } };


export const ListTasksDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"ListTasks"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"viewType"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ViewType"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"epoch"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"InputEpoch"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"tasks"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"filter"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"viewFilter"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"type"},"value":{"kind":"Variable","name":{"kind":"Name","value":"viewType"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"epoch"},"value":{"kind":"Variable","name":{"kind":"Name","value":"epoch"}}}]}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"cost"}},{"kind":"Field","name":{"kind":"Name","value":"isCompleted"}},{"kind":"Field","name":{"kind":"Name","value":"scheduledOn"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"type"}},{"kind":"Field","name":{"kind":"Name","value":"date"}}]}},{"kind":"Field","name":{"kind":"Name","value":"recurring"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"startDate"}},{"kind":"Field","name":{"kind":"Name","value":"pattern"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"every"}}]}}]}}]}}]}}]} as unknown as DocumentNode<ListTasksQuery, ListTasksQueryVariables>;
export const CreateTaskDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"CreateTask"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"title"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"cost"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"scheduledOn"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"InputEpoch"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"recurringSpec"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"InputRecurringSpec"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createTask"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"title"},"value":{"kind":"Variable","name":{"kind":"Name","value":"title"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"cost"},"value":{"kind":"Variable","name":{"kind":"Name","value":"cost"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"scheduledOn"},"value":{"kind":"Variable","name":{"kind":"Name","value":"scheduledOn"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"recurringSpec"},"value":{"kind":"Variable","name":{"kind":"Name","value":"recurringSpec"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"cost"}},{"kind":"Field","name":{"kind":"Name","value":"isCompleted"}}]}}]}}]} as unknown as DocumentNode<CreateTaskMutation, CreateTaskMutationVariables>;
export const UpdateTaskDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateTask"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"input"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UpdateTaskInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateTask"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"Variable","name":{"kind":"Name","value":"input"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"cost"}},{"kind":"Field","name":{"kind":"Name","value":"isCompleted"}},{"kind":"Field","name":{"kind":"Name","value":"scheduledOn"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"type"}},{"kind":"Field","name":{"kind":"Name","value":"date"}}]}}]}}]}}]} as unknown as DocumentNode<UpdateTaskMutation, UpdateTaskMutationVariables>;
export const UpdateTaskCompleteDateDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateTaskCompleteDate"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"completeDate"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"NaiveDate"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateTask"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"completeDate"},"value":{"kind":"Variable","name":{"kind":"Name","value":"completeDate"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"cost"}},{"kind":"Field","name":{"kind":"Name","value":"isCompleted"}},{"kind":"Field","name":{"kind":"Name","value":"scheduledOn"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"type"}},{"kind":"Field","name":{"kind":"Name","value":"date"}}]}}]}}]}}]} as unknown as DocumentNode<UpdateTaskCompleteDateMutation, UpdateTaskCompleteDateMutationVariables>;
export const DeleteTaskDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"DeleteTask"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"deleteTask"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}]}]}}]} as unknown as DocumentNode<DeleteTaskMutation, DeleteTaskMutationVariables>;
export const UpdateTaskEpochDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateTaskEpoch"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"scheduledOn"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"InputEpoch"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateTask"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"input"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"scheduledOn"},"value":{"kind":"Variable","name":{"kind":"Name","value":"scheduledOn"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"scheduledOn"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"type"}},{"kind":"Field","name":{"kind":"Name","value":"date"}}]}}]}}]}}]} as unknown as DocumentNode<UpdateTaskEpochMutation, UpdateTaskEpochMutationVariables>;