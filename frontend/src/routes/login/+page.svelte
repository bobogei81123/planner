<script lang="ts">
	import { goto } from '$app/navigation';
	import { login, tokenStore } from '$src/lib/auth';
	import {
		Client,
		cacheExchange,
		fetchExchange,
		getContextClient,
		mutationStore,
		queryStore,
		setContextClient
	} from '@urql/svelte';
	import { authExchange } from '@urql/exchange-auth';

	async function loginAndReturnHome() {
		await login('meteor');

		await goto('/tasks');
	}

	loginAndReturnHome();
</script>

<div>
	<p>login in...</p>
</div>
