import type { Exchange } from '@urql/svelte';
import { cacheExchange, type UpdatesConfig } from '@urql/exchange-graphcache';

export function getCacheExchange(): Exchange {
  const updatesConfig: UpdatesConfig = {
    Mutation: {
      createTask(_result, _args, cache) {
        cache
          .inspectFields('Query')
          .filter((field) => field.fieldName === 'tasks')
          .forEach((field) => {
            cache.invalidate('Query', field.fieldKey);
          });
      },
      updateTask(_result, _args, cache) {
        cache
          .inspectFields('Query')
          .filter((field) => field.fieldName === 'tasks')
          .forEach((field) => {
            cache.invalidate('Query', field.fieldKey);
          });
      },
      deleteTask(_result, _args, cache) {
        cache
          .inspectFields('Query')
          .filter((field) => field.fieldName === 'tasks')
          .forEach((field) => {
            cache.invalidate('Query', field.fieldKey);
          });
      }
    }
  };
  return cacheExchange({ updates: updatesConfig });
}
