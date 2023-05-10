<script>
	import Canvas from '../../lib/Canvas.svelte';
	import { goto } from '$app/navigation';

	function disconnect() {
		goto('/login');
	}
</script>

<main>
	<button on:click={disconnect}>Disconnect</button>
	<Canvas />
</main>

<style>
	button {
		color: red;
	}
</style>
