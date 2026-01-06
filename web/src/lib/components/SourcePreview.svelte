<script lang="ts">
	import { ExternalLink } from 'lucide-svelte'

	interface Props {
		id: string
		url: string
		title: string
		relevance: number
		isNew?: boolean
	}

	const { id, url, title, relevance, isNew = false }: Props = $props()

	function getDomain(urlStr: string): string {
		try {
			return new URL(urlStr).hostname.replace(/^www\./, '')
		} catch {
			return urlStr
		}
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

	const domain = $derived(getDomain(url))
	const initial = $derived(getInitial(domain))
	const bgColor = $derived(getColorForDomain(domain))
	const isHighRelevance = $derived(relevance > 0.8)
</script>

<a
	href={url}
	target="_blank"
	rel="noopener noreferrer"
	class="group flex items-start gap-3 rounded-lg border p-3 transition-all duration-300 hover:shadow-sm"
	class:animate-fade-in={isNew}
	style="background-color: var(--color-bg); border-color: var(--color-border);"
	data-source-id={id}
>
	<div
		class="flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full text-sm font-semibold text-white"
		style="background-color: {bgColor};"
	>
		{initial}
	</div>

	<div class="min-w-0 flex-1">
		<div class="flex items-start gap-2">
			<p
				class="line-clamp-2 flex-1 text-sm font-medium leading-tight group-hover:underline"
				style="color: var(--color-text);"
			>
				{title}
			</p>
			<ExternalLink
				class="h-3.5 w-3.5 flex-shrink-0 opacity-0 transition-opacity group-hover:opacity-100"
				style="color: var(--color-text-muted);"
			/>
		</div>

		<div class="mt-1 flex items-center gap-2">
			<span class="text-xs" style="color: var(--color-text-muted);">
				{domain}
			</span>
			{#if isHighRelevance}
				<span
					class="rounded-full px-1.5 py-0.5 text-xs font-medium"
					style="background-color: var(--color-success); color: white;"
				>
					High relevance
				</span>
			{/if}
		</div>
	</div>
</a>

<style>
	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.animate-fade-in {
		animation: fade-in 0.3s ease-out;
	}
</style>
