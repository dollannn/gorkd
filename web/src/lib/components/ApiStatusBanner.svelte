<script lang="ts">
	import { AlertTriangle, RefreshCw } from 'lucide-svelte'

	interface Props {
		onRetry?: () => void
		isRetrying?: boolean
	}

	let { onRetry, isRetrying = false }: Props = $props()
</script>

<div
	class="flex items-center gap-3 rounded-lg border px-4 py-3"
	style="
		background-color: color-mix(in srgb, var(--color-warning) 10%, transparent);
		border-color: var(--color-warning);
	"
	role="alert"
>
	<AlertTriangle
		class="h-5 w-5 shrink-0"
		style="color: var(--color-warning);"
	/>

	<div class="flex-1">
		<p class="text-sm font-medium" style="color: var(--color-text);">
			Unable to connect to the research API
		</p>
		<p class="text-sm" style="color: var(--color-text-muted);">
			The service may be temporarily unavailable. Please try again later.
		</p>
	</div>

	{#if onRetry}
		<button
			onclick={onRetry}
			disabled={isRetrying}
			class="flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm font-medium transition-colors hover:opacity-80 disabled:opacity-50"
			style="
				background-color: var(--color-warning);
				color: white;
			"
		>
			<span class={isRetrying ? 'animate-spin' : ''}>
				<RefreshCw class="h-4 w-4" />
			</span>
			{isRetrying ? 'Checking...' : 'Retry'}
		</button>
	{/if}
</div>
