<script lang="ts">
	import { Sparkles } from 'lucide-svelte'

	interface Props {
		onselect?: (query: string) => void
	}

	const { onselect }: Props = $props()

	const examples = [
		'What caused the 2024 CrowdStrike outage?',
		'Is coffee good or bad for health?',
		'How does mRNA vaccine technology work?',
		'What are the latest developments in nuclear fusion?',
	]
</script>

<div class="space-y-3">
	<div class="flex items-center gap-2" style="color: var(--color-text-muted);">
		<Sparkles class="h-4 w-4" />
		<span class="text-sm font-medium">Try an example</span>
	</div>

	<div class="flex flex-wrap gap-2">
		{#each examples as example (example)}
			<button
				type="button"
				onclick={() => onselect?.(example)}
				class="rounded-full border px-3 py-1.5 text-sm transition-colors hover:border-transparent"
				style="
					background-color: var(--color-bg-subtle);
					border-color: var(--color-border);
					color: var(--color-text);
				"
				style:--hover-bg="var(--color-accent)"
			>
				{example}
			</button>
		{/each}
	</div>
</div>

<style>
	button:hover {
		background-color: var(--color-accent);
		color: white;
	}
</style>
