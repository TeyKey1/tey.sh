<script lang="ts">
  import { page } from "$app/stores";

  // Components
  import ErrorMessage from "$lib/ErrorMessage.svelte";
</script>

<ErrorMessage message="Oh noes something went wrong :[" status={$page.status} />
