<script lang="ts">
	import type { Snippet } from 'svelte'

	interface Props {
		title?: string
		children: Snippet
	}

	const { title, children }: Props = $props()
</script>

<div
	class="rounded-lg border p-4"
	style="background-color: var(--color-bg); border-color: var(--color-border);"
>
	{#if title}
		<h3 class="mb-3 text-lg font-semibold" style="color: var(--color-text);">
			{title}
		</h3>
	{/if}
	{@render children()}
</div>
