<script lang="ts">
	import { Button } from '$lib/components'

	type ConnectionState =
		| 'disconnected'
		| 'connecting'
		| 'connected'
		| 'reconnecting'
		| 'failed'

	interface Props {
		state: ConnectionState
		onRetry?: () => void
	}

	const { state, onRetry }: Props = $props()

	const statusConfig: Record<
		ConnectionState,
		{ label: string; dotColor: string; animate: boolean }
	> = {
		disconnected: {
			label: 'Disconnected',
			dotColor: 'var(--color-text-muted)',
			animate: false,
		},
		connecting: {
			label: 'Connecting...',
			dotColor: '#eab308',
			animate: true,
		},
		connected: {
			label: 'Live',
			dotColor: 'var(--color-success)',
			animate: false,
		},
		reconnecting: {
			label: 'Reconnecting...',
			dotColor: '#eab308',
			animate: true,
		},
		failed: {
			label: 'Connection lost',
			dotColor: 'var(--color-error)',
			animate: false,
		},
	}

	const config = $derived(statusConfig[state])
</script>

<div class="inline-flex items-center gap-2">
	<div class="flex items-center gap-1.5">
		<div
			class="h-2 w-2 rounded-full"
			class:animate-pulse={config.animate}
			style="background-color: {config.dotColor};"
		></div>
		<span class="text-xs font-medium" style="color: var(--color-text-muted);">
			{config.label}
		</span>
	</div>

	{#if state === 'failed' && onRetry}
		<Button variant="secondary" size="sm" onclick={onRetry}>Retry</Button>
	{/if}
</div>
