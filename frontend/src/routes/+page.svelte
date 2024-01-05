<script lang="ts">
  import { login, tokenStore } from '$lib/auth';

  $: {
    if ($tokenStore === null) {
      login('meteor');
    }
  }
</script>
