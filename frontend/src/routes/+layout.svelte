<script>
	import '../app.css';
	import { Client, cacheExchange, fetchExchange, setContextClient } from '@urql/svelte';

	const client = new Client({
		url: '/graphql',
		exchanges: [cacheExchange, fetchExchange]
	});
	setContextClient(client);
</script>

<slot />
