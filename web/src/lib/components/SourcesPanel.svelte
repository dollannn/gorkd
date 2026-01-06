<script lang="ts">
	import { untrack } from 'svelte'
	import { ChevronDown, ChevronUp } from 'lucide-svelte'
	import SourceCard from './SourceCard.svelte'
	import type { Source, Citation } from '$lib/api/types'

	interface Props {
		sources: Source[]
		citations?: Citation[]
		highlightedSourceId?: string | null
		initialExpanded?: boolean
	}

	const { sources, citations = [], highlightedSourceId = null, initialExpanded = true }: Props = $props()

	let isExpanded = $state(untrack(() => initialExpanded))

	function toggleExpanded() {
		isExpanded = !isExpanded
	}

	function getCitationNumber(sourceId: string): number | undefined {
		const index = citations.findIndex((c) => c.source_id === sourceId)
		return index >= 0 ? index + 1 : undefined
	}

	function isCited(sourceId: string): boolean {
		return citations.some((c) => c.source_id === sourceId)
	}

	const sortedSources = $derived(() => {
		const cited: Source[] = []
		const uncited: Source[] = []

		sources.forEach((source) => {
			if (isCited(source.id)) {
				cited.push(source)
			} else {
				uncited.push(source)
			}
		})

		cited.sort((a, b) => {
			const numA = getCitationNumber(a.id) ?? Infinity
			const numB = getCitationNumber(b.id) ?? Infinity
			return numA - numB
		})

		uncited.sort((a, b) => (b.relevance_score ?? 0) - (a.relevance_score ?? 0))

		return [...cited, ...uncited]
	})
</script>

<div
	class="rounded-lg border"
	style="background-color: var(--color-bg); border-color: var(--color-border);"
>
	<button
		type="button"
		onclick={toggleExpanded}
		class="flex w-full items-center justify-between p-4 text-left transition-colors hover:opacity-80"
		aria-expanded={isExpanded}
		aria-controls="sources-content"
	>
		<h3 class="flex items-center gap-2 text-sm font-semibold uppercase tracking-wide" style="color: var(--color-text-muted);">
			Sources
			<span
				class="rounded-full px-2 py-0.5 text-xs font-medium"
				style="background-color: var(--color-bg-muted); color: var(--color-text);"
			>
				{sources.length}
			</span>
		</h3>
		{#if isExpanded}
			<ChevronUp class="h-5 w-5" style="color: var(--color-text-muted);" />
		{:else}
			<ChevronDown class="h-5 w-5" style="color: var(--color-text-muted);" />
		{/if}
	</button>

	{#if isExpanded}
		<div
			id="sources-content"
			class="space-y-3 border-t px-4 pb-4 pt-3"
			style="border-color: var(--color-border);"
		>
			{#each sortedSources() as source (source.id)}
				<SourceCard
					{source}
					citationNumber={getCitationNumber(source.id)}
					isHighlighted={highlightedSourceId === source.id}
				/>
			{/each}

			{#if sources.length === 0}
				<p class="py-4 text-center text-sm" style="color: var(--color-text-muted);">
					No sources found
				</p>
			{/if}
		</div>
	{/if}
</div>
