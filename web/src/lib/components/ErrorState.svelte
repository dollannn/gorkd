<script lang="ts">
	import { AlertCircle, WifiOff, Clock, Search, Server, RefreshCw } from 'lucide-svelte'
	import Button from './Button.svelte'

	type ErrorType = 'network' | 'not_found' | 'rate_limited' | 'search_failed' | 'llm_failed' | 'generic'

	interface Props {
		type?: ErrorType
		title?: string
		message?: string
		onRetry?: () => void
		onNewQuery?: () => void
		retryLabel?: string
	}

	const {
		type = 'generic',
		title,
		message,
		onRetry,
		onNewQuery,
		retryLabel = 'Try again',
	}: Props = $props()

	const errorDefaults: Record<ErrorType, { defaultTitle: string; defaultMessage: string }> = {
		network: {
			defaultTitle: "Couldn't connect to server",
			defaultMessage: 'Please check your internet connection and try again.',
		},
		not_found: {
			defaultTitle: 'Research not found',
			defaultMessage: "This research job doesn't exist or may have expired.",
		},
		rate_limited: {
			defaultTitle: 'Too many requests',
			defaultMessage: 'Please wait a moment before trying again.',
		},
		search_failed: {
			defaultTitle: 'Search providers unavailable',
			defaultMessage: 'Unable to search for sources. Please try again.',
		},
		llm_failed: {
			defaultTitle: "Couldn't generate answer",
			defaultMessage: 'The AI service is temporarily unavailable.',
		},
		generic: {
			defaultTitle: 'Something went wrong',
			defaultMessage: 'An unexpected error occurred. Please try again.',
		},
	}

	const config = $derived(errorDefaults[type])
	const displayTitle = $derived(title ?? config.defaultTitle)
	const displayMessage = $derived(message ?? config.defaultMessage)
</script>

<div
	class="flex flex-col items-center gap-4 rounded-lg border p-8 text-center"
	style="background-color: var(--color-bg); border-color: var(--color-border);"
	role="alert"
	aria-live="polite"
>
	<div
		class="flex h-12 w-12 items-center justify-center rounded-full"
		style="background-color: color-mix(in srgb, var(--color-error) 10%, transparent);"
	>
		{#if type === 'network'}
			<WifiOff class="h-6 w-6" style="color: var(--color-error);" />
		{:else if type === 'not_found'}
			<Search class="h-6 w-6" style="color: var(--color-error);" />
		{:else if type === 'rate_limited'}
			<Clock class="h-6 w-6" style="color: var(--color-error);" />
		{:else if type === 'search_failed'}
			<Search class="h-6 w-6" style="color: var(--color-error);" />
		{:else if type === 'llm_failed'}
			<Server class="h-6 w-6" style="color: var(--color-error);" />
		{:else}
			<AlertCircle class="h-6 w-6" style="color: var(--color-error);" />
		{/if}
	</div>

	<div class="space-y-2">
		<h2 class="text-lg font-semibold" style="color: var(--color-text);">
			{displayTitle}
		</h2>
		<p class="max-w-sm text-sm" style="color: var(--color-text-muted);">
			{displayMessage}
		</p>
	</div>

	<div class="flex gap-3">
		{#if onNewQuery && type === 'not_found'}
			<Button variant="primary" onclick={onNewQuery}>
				New research
			</Button>
		{/if}
		{#if onRetry && type !== 'not_found'}
			<Button variant="primary" onclick={onRetry}>
				<RefreshCw class="h-4 w-4" />
				{retryLabel}
			</Button>
		{/if}
		{#if onNewQuery && type !== 'not_found'}
			<Button variant="secondary" onclick={onNewQuery}>
				New research
			</Button>
		{/if}
	</div>
</div>
