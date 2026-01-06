<script lang="ts">
	import { CheckCircle, AlertCircle, AlertTriangle, XCircle } from 'lucide-svelte'
	import type { Confidence } from '$lib/api/types'

	interface Props {
		level: Confidence
		showLabel?: boolean
	}

	const { level, showLabel = true }: Props = $props()

	const config = {
		high: {
			icon: CheckCircle,
			label: 'High confidence',
			bgClass: 'bg-green-100 dark:bg-green-900/30',
			textClass: 'text-green-800 dark:text-green-300',
		},
		medium: {
			icon: AlertCircle,
			label: 'Medium confidence',
			bgClass: 'bg-yellow-100 dark:bg-yellow-900/30',
			textClass: 'text-yellow-800 dark:text-yellow-300',
		},
		low: {
			icon: AlertTriangle,
			label: 'Low confidence',
			bgClass: 'bg-orange-100 dark:bg-orange-900/30',
			textClass: 'text-orange-800 dark:text-orange-300',
		},
		insufficient: {
			icon: XCircle,
			label: 'Insufficient evidence',
			bgClass: 'bg-red-100 dark:bg-red-900/30',
			textClass: 'text-red-800 dark:text-red-300',
		},
	}

	const current = $derived(config[level])
</script>

<span
	class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-sm font-medium {current.bgClass} {current.textClass}"
	role="status"
	aria-label="{current.label}"
>
	<current.icon class="h-4 w-4" aria-hidden="true" />
	{#if showLabel}
		<span>{current.label}</span>
	{/if}
</span>
