<script lang="ts">
	import { Clock, FileText, Zap } from 'lucide-svelte'

	interface Props {
		durationMs?: number
		sourcesConsidered?: number
		cached?: boolean
	}

	const { durationMs, sourcesConsidered, cached = false }: Props = $props()

	const formattedDuration = $derived(() => {
		if (durationMs === undefined) return null
		const seconds = durationMs / 1000
		return seconds < 1 ? `${durationMs}ms` : `${seconds.toFixed(1)}s`
	})
</script>

<div
	class="flex flex-wrap items-center gap-x-4 gap-y-2 text-sm"
	style="color: var(--color-text-muted);"
>
	{#if formattedDuration()}
		<span class="inline-flex items-center gap-1.5">
			<Clock class="h-4 w-4" aria-hidden="true" />
			<span>Completed in {formattedDuration()}</span>
		</span>
	{/if}

	{#if sourcesConsidered !== undefined}
		<span class="inline-flex items-center gap-1.5">
			<FileText class="h-4 w-4" aria-hidden="true" />
			<span>{sourcesConsidered} sources considered</span>
		</span>
	{/if}

	{#if cached}
		<span
			class="inline-flex items-center gap-1.5 rounded-full px-2 py-0.5"
			style="background-color: var(--color-bg-muted);"
		>
			<Zap class="h-3.5 w-3.5" style="color: var(--color-warning);" aria-hidden="true" />
			<span class="text-xs font-medium">Cached</span>
		</span>
	{/if}
</div>
