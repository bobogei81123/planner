<script lang="ts">
  import { goto } from '$app/navigation';
  import { login } from '$src/lib/auth';

  async function loginAndReturnHome() {
    await login('meteor');
    await goto('/tasks');
  }

  loginAndReturnHome();
</script>

<div>
  <p>login in...</p>
</div>
