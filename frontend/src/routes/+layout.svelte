<script lang="ts">
  import '../app.css';
  import {
    Client,
    fetchExchange,
    getContextClient,
    mutationStore,
    queryStore,
    setContextClient
  } from '@urql/svelte';
  import { authExchange } from '@urql/exchange-auth';
  import { ModeWatcher } from 'mode-watcher';
  import { devtoolsExchange } from '@urql/devtools';

  import { graphql } from '$src/gql';
  import { getAuthConfig } from '$lib/auth';
  import { getCacheExchange } from '$lib/graph_cache';
  import { Button } from '$lib/components/ui/button';
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
  import { Sun, Moon } from 'lucide-svelte';
  import { setMode, resetMode } from 'mode-watcher';
  import { cn } from '$src/lib/utils';
  import { page } from '$app/stores';

  const client = new Client({
    url: '/graphql',
    exchanges: [
      devtoolsExchange,
      getCacheExchange(),
      authExchange(async (utils) => getAuthConfig(utils)),
      fetchExchange
    ]
  });
  setContextClient(client);

  const allIterationsStore = queryStore({
    client: getContextClient(),
    query: graphql(`
      query allIterations {
        iterations {
          id
          name
        }
      }
    `)
  });

  function iterationUrl(iterationId: string) {
    return `/iteration/${iterationId}`;
  }

  let iterationModalIsActive = false;
  let newIterationName: string | null = null;

  function createIteration() {
    mutationStore({
      client,
      query: graphql(`
        mutation createIteration($name: String) {
          createIteration(input: { name: $name }) {
            id
            name
          }
        }
      `),
      variables: { name: newIterationName }
    });
    newIterationName = null;
    iterationModalIsActive = false;
  }

  const routes = [
    { name: 'Tasks', href: '/tasks' },
    { name: 'Task Schedules', href: '/task-schedules' }
  ];
</script>

<ModeWatcher />
<div class="w-full flex-col">
  <div class="border-b flex justify-between items-center px-6">
    <nav class="h-12 flex items-center space-x-4">
      {#each routes as route}
        {@const isActive = $page.url.pathname.startsWith(route.href)}
        <a
          class={cn(
            'rounded-full px-4 text-muted-foreground transition-colors hover:text-primary',
            isActive ? 'text-primary bg-muted' : ''
          )}
          href={route.href}>{route.name}</a
        >
      {/each}
    </nav>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild let:builder>
        <Button builders={[builder]} variant="outline" size="icon">
          <Sun
            class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
          />
          <Moon
            class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
          />
          <span class="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="end">
        <DropdownMenu.Item on:click={() => setMode('light')}>Light</DropdownMenu.Item>
        <DropdownMenu.Item on:click={() => setMode('dark')}>Dark</DropdownMenu.Item>
        <DropdownMenu.Item on:click={() => resetMode()}>System</DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </div>
  <div class="w-full min-h-screen flex justify-center">
    <slot />
  </div>
</div>
