<script lang="ts">
	import { ExternalLink, CheckCircle } from 'lucide-svelte'
	import type { Source } from '$lib/api/types'

	interface Props {
		source: Source
		citationNumber?: number
		isHighlighted?: boolean
	}

	const { source, citationNumber, isHighlighted = false }: Props = $props()

	function getFaviconUrl(domain: string): string {
		return `https://www.google.com/s2/favicons?domain=${domain}&sz=32`
	}

	function getInitial(domain: string): string {
		return domain.charAt(0).toUpperCase()
	}

	function getColorForDomain(domain: string): string {
		const colors = [
			'#3b82f6',
			'#8b5cf6',
			'#ec4899',
			'#f97316',
			'#22c55e',
			'#06b6d4',
			'#6366f1',
		]
		const hash = domain.split('').reduce((a, b) => a + b.charCodeAt(0), 0)
		return colors[hash % colors.length]
	}

	function formatDate(dateStr: string): string {
		try {
			const date = new Date(dateStr)
			return date.toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'short',
				day: 'numeric',
			})
		} catch {
			return dateStr
		}
	}

	const relevancePercent = $derived(Math.round((source.relevance_score ?? 0) * 100))
	const bgColor = $derived(getColorForDomain(source.domain))

	let faviconError = $state(false)

	function handleFaviconError() {
		faviconError = true
	}
</script>

<a
	href={source.url}
	target="_blank"
	rel="noopener noreferrer"
	id="source-{source.id}"
	class="source-card group block rounded-lg border p-4 transition-all hover:shadow-md"
	class:ring-2={isHighlighted}
	class:ring-offset-2={isHighlighted}
	style="
		background-color: var(--color-bg);
		border-color: var(--color-border);
		{isHighlighted ? '--tw-ring-color: var(--color-accent);' : ''}
	"
	data-source-id={source.id}
>
	<div class="flex items-start gap-3">
		<div class="flex-shrink-0">
			{#if !faviconError}
				<img
					src={getFaviconUrl(source.domain)}
					alt=""
					class="h-8 w-8 rounded"
					onerror={handleFaviconError}
				/>
			{:else}
				<div
					class="flex h-8 w-8 items-center justify-center rounded text-sm font-semibold text-white"
					style="background-color: {bgColor};"
				>
					{getInitial(source.domain)}
				</div>
			{/if}
		</div>

		<div class="min-w-0 flex-1">
			<div class="flex items-start justify-between gap-2">
				<div class="min-w-0 flex-1">
					<div class="flex items-center gap-2">
						{#if citationNumber !== undefined}
							<span
								class="flex-shrink-0 rounded px-1.5 py-0.5 text-xs font-semibold"
								style="background-color: var(--color-accent); color: white;"
							>
								[{citationNumber}]
							</span>
						{/if}
						<span class="truncate text-sm" style="color: var(--color-text-muted);">
							{source.domain}
						</span>
						{#if source.published_at}
							<span class="text-xs" style="color: var(--color-text-muted);">
								· {formatDate(source.published_at)}
							</span>
						{/if}
					</div>
					<h4
						class="mt-1 line-clamp-2 font-medium leading-snug group-hover:underline"
						style="color: var(--color-text);"
					>
						{source.title}
					</h4>
				</div>
				<ExternalLink
					class="h-4 w-4 flex-shrink-0 opacity-0 transition-opacity group-hover:opacity-100"
					style="color: var(--color-text-muted);"
				/>
			</div>

			{#if source.relevance_score !== undefined}
				<div class="mt-3 flex items-center gap-2">
					<div
						class="h-1.5 flex-1 overflow-hidden rounded-full"
						style="background-color: var(--color-bg-muted);"
					>
						<div
							class="h-full rounded-full transition-all"
							style="
								width: {relevancePercent}%;
								background-color: {relevancePercent >= 80 ? 'var(--color-success)' : relevancePercent >= 50 ? 'var(--color-warning)' : 'var(--color-error)'};
							"
						></div>
					</div>
					<span class="flex-shrink-0 text-xs tabular-nums" style="color: var(--color-text-muted);">
						{relevancePercent}% relevance
					</span>
				</div>
			{/if}

			{#if source.content_preview}
				<p
					class="mt-2 line-clamp-2 text-sm leading-relaxed"
					style="color: var(--color-text-muted);"
				>
					{source.content_preview}
				</p>
			{/if}

			{#if source.used_in_citations}
				<div class="mt-2 flex items-center gap-1">
					<CheckCircle class="h-3.5 w-3.5" style="color: var(--color-success);" />
					<span class="text-xs font-medium" style="color: var(--color-success);">
						Used in answer
					</span>
				</div>
			{/if}
		</div>
	</div>
</a>

<style>
	.source-card:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 2px;
	}
</style>
