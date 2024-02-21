import type { Exchange } from '@urql/svelte';
import {
  cacheExchange,
  type Cache,
  type OptimisticMutationConfig,
  type UpdatesConfig
} from '@urql/exchange-graphcache';
import schema from '$src/generated-introspection.json';
import { graphql } from '$src/gql';

function allTasksCache(cache: Cache) {
  return cache.inspectFields('QueryRoot').filter((field) => field.fieldName === 'tasks');
}

function invalidateAllTasks(cache: Cache) {
  console.debug('Invalidating all tasks');
  for (let field of allTasksCache(cache)) {
    cache.invalidate('QueryRoot', field.fieldKey);
  }
}

export function getCacheExchange(): Exchange {
  const updatesConfig: UpdatesConfig = {
    MutationRoot: {
      createTask(_result, _args, cache) {
        invalidateAllTasks(cache);
      },
      updateTask(parent, args, cache) {
        if (args.input?.plannedOn !== undefined) {
          invalidateAllTasks(cache);
        }
      },
      deleteTask(_result, _args, cache) {
        invalidateAllTasks(cache);
      }
    }
  };
  // TODO: Add optimistic updates.
  const optimisticConfig: OptimisticMutationConfig = {};
  return cacheExchange({
    updates: updatesConfig,
    schema
  });
}
