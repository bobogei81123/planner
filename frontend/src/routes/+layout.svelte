<script lang="ts">
  import {
    Input,
    Label,
    Modal,
    Sidebar,
    SidebarDropdownItem,
    SidebarDropdownWrapper,
    SidebarGroup,
    SidebarItem,
    SidebarWrapper
  } from 'flowbite-svelte';
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
</script>

<div class="w-full min-h-screen flex justify-center">
  <ModeWatcher />
  <!-- <Sidebar class="absolute left-0"> -->
  <!--   <SidebarWrapper> -->
  <!--     <SidebarGroup> -->
  <!--       <SidebarItem label="All Tasks" href="/tasks" /> -->
  <!--       <SidebarDropdownWrapper label="Iterations"> -->
  <!--         {#if $allIterationsStore.data != null && $allIterationsStore.data != undefined} -->
  <!--           {#each $allIterationsStore.data.iterations as iteration} -->
  <!--             <SidebarDropdownItem label={iteration.name} href={iterationUrl(iteration.id)} /> -->
  <!--           {/each} -->
  <!--           <SidebarDropdownItem -->
  <!--             label="New Iteration" -->
  <!--             on:click={() => (iterationModalIsActive = true)} -->
  <!--           /> -->
  <!--         {/if} -->
  <!--       </SidebarDropdownWrapper> -->
  <!--     </SidebarGroup> -->
  <!--   </SidebarWrapper> -->
  <!-- </Sidebar> -->
  <slot />
</div>

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

<!-- <Modal bind:open={iterationModalIsActive} size="xs" autoclose={false}> -->
<!--   <form class="flex flex-col space-y-6" action="#"> -->
<!--     <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Create a new iteration</h3> -->
<!--     <Label class="space-y-2"> -->
<!--       <span>Name</span> -->
<!--       <Input type="text" name="name" placeholder="New Iteration" bind:value={newIterationName} /> -->
<!--     </Label> -->
<!--     <Button type="submit" class="w-full" color="red" on:click={createIteration}>Create</Button> -->
<!--   </form> -->
<!-- </Modal> -->
