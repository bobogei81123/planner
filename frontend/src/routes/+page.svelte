<script lang="ts">
	import { goto } from '$app/navigation';
	import { login, tokenStore } from '$lib/auth';

	$: {
		if ($tokenStore === null) {
			login('meteor');
		}
	}

</script>
