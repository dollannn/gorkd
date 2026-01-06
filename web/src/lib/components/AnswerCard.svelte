<script lang="ts">
	import { ChevronDown, ChevronUp } from 'lucide-svelte'
	import ConfidenceBadge from './ConfidenceBadge.svelte'
	import CitationLink from './CitationLink.svelte'
	import type { Confidence, Source, Citation } from '$lib/api/types'

	interface Props {
		summary: string
		detail?: string
		confidence: Confidence
		citations?: Citation[]
		sources?: Source[]
		onCitationClick?: (sourceId: string) => void
	}

	const { summary, detail, confidence, citations = [], sources = [], onCitationClick }: Props = $props()

	let isExpanded = $state(false)
	const hasDetail = $derived(detail && detail.trim().length > 0)

	function toggleExpanded() {
		isExpanded = !isExpanded
	}

	type ParsedPart =
		| { type: 'text'; content: string; key: string }
		| { type: 'citation'; number: number; sourceId: string; title: string; domain: string; key: string }

	function parseCitations(text: string): ParsedPart[] {
		const parts: ParsedPart[] = []
		const regex = /\[(\d+)\]/g
		let lastIndex = 0
		let match
		let partIndex = 0

		while ((match = regex.exec(text)) !== null) {
			if (match.index > lastIndex) {
				parts.push({
					type: 'text',
					content: text.slice(lastIndex, match.index),
					key: `text-${partIndex++}`,
				})
			}

			const citationNumber = parseInt(match[1], 10)
			const citation = citations[citationNumber - 1]
			if (citation) {
				const source = sources.find((s) => s.id === citation.source_id)
				if (source) {
					parts.push({
						type: 'citation',
						number: citationNumber,
						sourceId: citation.source_id,
						title: source.title,
						domain: source.domain,
						key: `cite-${citationNumber}-${partIndex++}`,
					})
				} else {
					parts.push({ type: 'text', content: match[0], key: `text-${partIndex++}` })
				}
			} else {
				parts.push({ type: 'text', content: match[0], key: `text-${partIndex++}` })
			}

			lastIndex = regex.lastIndex
		}

		if (lastIndex < text.length) {
			parts.push({
				type: 'text',
				content: text.slice(lastIndex),
				key: `text-${partIndex++}`,
			})
		}

		return parts
	}

	const summaryParts = $derived(parseCitations(summary))
	const detailParts = $derived(detail ? parseCitations(detail) : [])
</script>

<div
	class="rounded-lg border"
	style="background-color: var(--color-bg); border-color: var(--color-border);"
>
	<div class="p-4">
		<div class="mb-3 flex items-center justify-between">
			<h3 class="text-sm font-semibold uppercase tracking-wide" style="color: var(--color-text-muted);">
				Answer
			</h3>
			<ConfidenceBadge level={confidence} />
		</div>

		<div class="space-y-3">
			<p class="text-base leading-relaxed" style="color: var(--color-text);">
				{#each summaryParts as part (part.key)}
					{#if part.type === 'text'}
						{part.content}
					{:else}
						<CitationLink
							number={part.number}
							sourceId={part.sourceId}
							sourceTitle={part.title}
							sourceDomain={part.domain}
							onClickCitation={onCitationClick}
						/>
					{/if}
				{/each}
			</p>

			{#if hasDetail}
				{#if isExpanded}
					<div
						class="prose prose-sm max-w-none border-t pt-3"
						style="color: var(--color-text); border-color: var(--color-border);"
					>
						{#each detailParts as part (part.key)}
							{#if part.type === 'text'}
								{part.content}
							{:else}
								<CitationLink
									number={part.number}
									sourceId={part.sourceId}
									sourceTitle={part.title}
									sourceDomain={part.domain}
									onClickCitation={onCitationClick}
								/>
							{/if}
						{/each}
					</div>
				{/if}

				<button
					type="button"
					onclick={toggleExpanded}
					class="inline-flex items-center gap-1 text-sm font-medium transition-colors hover:opacity-80"
					style="color: var(--color-accent);"
					aria-expanded={isExpanded}
				>
					{#if isExpanded}
						<ChevronUp class="h-4 w-4" />
						Read less
					{:else}
						<ChevronDown class="h-4 w-4" />
						Read more
					{/if}
				</button>
			{/if}
		</div>
	</div>
</div>
