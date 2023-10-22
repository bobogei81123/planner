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
	import {
		Client,
		cacheExchange,
		fetchExchange,
		getContextClient,
		mutationStore,
		queryStore,
		setContextClient
	} from '@urql/svelte';
	import { graphql } from '$src/gql';
	import { uuidToBase64 } from '$lib';
	import { login, tokenStore } from '$lib/auth';
	import { PlusSolid } from 'flowbite-svelte-icons';
	import { authExchange } from '@urql/exchange-auth';
	import { goto } from '$app/navigation';
	import { get } from 'svelte/store';

	const _authExchange = authExchange(async (utils) => {
		return {
			addAuthToOperation(operation) {
				const token = get(tokenStore);
				if (token === null) {
					return operation;
				}
				return utils.appendHeaders(operation, {
					Authorization: `Bearer ${token}`
				});
			},
			didAuthError(error) {
				console.log('error');
				return error.graphQLErrors.some((e) => e.extensions?.code === 'FORBIDDEN');
			},
			async refreshAuth() {
				console.log('refresh');
				await goto('/login');
			}
		};
	});
	const client = new Client({
		url: '/graphql',
		exchanges: [cacheExchange, _authExchange, fetchExchange]
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

<div class="w-full min-h-screen flex justify-center bg-gray-200">
	<Sidebar class="absolute left-0">
		<SidebarWrapper>
			<SidebarGroup>
				<SidebarItem label="All Tasks" href="/tasks" />
				<SidebarDropdownWrapper label="Iterations">
					{#if $allIterationsStore.data != null && $allIterationsStore.data != undefined}
						{#each $allIterationsStore.data.iterations as iteration}
							<SidebarDropdownItem label={iteration.name} href={iterationUrl(iteration.id)} />
						{/each}
						<SidebarDropdownItem
							label="New Iteration"
							on:click={() => (iterationModalIsActive = true)}
						/>
					{/if}
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
			<Input type="text" name="name" placeholder="New Iteration" bind:value={newIterationName} />
		</Label>
		<Button type="submit" class="w-full" color="red" on:click={createIteration}>Create</Button>
	</form>
</Modal>
