<script lang="ts">
	import { Moon, Sun } from 'lucide-svelte'
	import { themeStore } from '$lib/stores/theme.svelte'
</script>

<button
	type="button"
	onclick={() => themeStore.toggle()}
	class="inline-flex h-9 w-9 items-center justify-center rounded-md transition-colors hover:bg-(--color-bg-subtle) focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-(--color-accent)"
	aria-label={themeStore.value === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
>
	{#if themeStore.value === 'dark'}
		<Sun class="h-5 w-5" style="color: var(--color-text-muted);" />
	{:else}
		<Moon class="h-5 w-5" style="color: var(--color-text-muted);" />
	{/if}
</button>
