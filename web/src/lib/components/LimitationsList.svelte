<script lang="ts">
	import { AlertTriangle } from 'lucide-svelte'

	interface Props {
		limitations: string[]
	}

	const { limitations }: Props = $props()

	const hasLimitations = $derived(limitations.length > 0)
</script>

{#if hasLimitations}
	<div
		class="rounded-lg border"
		style="background-color: var(--color-bg); border-color: var(--color-border);"
	>
		<div class="p-4">
			<div class="mb-3 flex items-center gap-2">
				<AlertTriangle class="h-4 w-4" style="color: var(--color-warning);" />
				<h3 class="text-sm font-semibold uppercase tracking-wide" style="color: var(--color-text-muted);">
					Limitations
				</h3>
			</div>

			<ul class="space-y-2" role="list">
				{#each limitations as limitation, index (index)}
					<li class="flex items-start gap-2 text-sm" style="color: var(--color-text);">
						<span class="mt-1.5 h-1.5 w-1.5 flex-shrink-0 rounded-full" style="background-color: var(--color-text-muted);"></span>
						<span>{limitation}</span>
					</li>
				{/each}
			</ul>
		</div>
	</div>
{/if}
