<script lang="ts">
	import {
	Button,
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
	import { Client, cacheExchange, fetchExchange, mutationStore, queryStore, setContextClient } from '@urql/svelte';
	import { graphql } from '$src/gql';
	import { uuidToBase64 } from '$lib';
	import { PlusSolid } from 'flowbite-svelte-icons';

	const client = new Client({
		url: '/graphql',
		exchanges: [cacheExchange, fetchExchange]
	});
	setContextClient(client);

	const allIterationsStore = queryStore({
		client,
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
		return `/iteration/${uuidToBase64(iterationId)}`;
	}

	$: {
		console.log($allIterationsStore.data);
	}

	let iterationModalIsActive = false;
  let newIterationName = null;

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

<div class="w-full min-h-screen flex justify-center bg-gray-200">
	<Sidebar class="absolute left-0">
		<SidebarWrapper>
			<SidebarGroup>
				<SidebarItem label="All Tasks" href="/tasks" />
				<SidebarDropdownWrapper label="Iterations">
					{#each $allIterationsStore.data.iterations as iteration}
						<SidebarDropdownItem label={iteration.name} href={iterationUrl(iteration.id)} />
					{/each}
					<SidebarDropdownItem
						label="New Iteration"
						on:click={() => (iterationModalIsActive = true)}
					/>
				</SidebarDropdownWrapper>
			</SidebarGroup>
		</SidebarWrapper>
	</Sidebar>
	<slot />
</div>

<Modal bind:open={iterationModalIsActive} size="xs" autoclose={false}>
	<form class="flex flex-col space-y-6" action="#">
		<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Create a new iteration</h3>
		<Label class="space-y-2">
			<span>Name</span>
			<Input type="text" name="name" placeholder="New Iteration" bind:value={newIterationName}/>
		</Label>
		<Button type="submit" class="w-full" color="red" on:click={createIteration}>Create</Button>
	</form>
</Modal>
