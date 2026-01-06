<script lang="ts">
	import Skeleton from './Skeleton.svelte'

	interface Props {
		count?: number
	}

	const { count = 1 }: Props = $props()
</script>

<div class="space-y-3" role="status" aria-label="Loading sources">
	{#each Array.from({ length: count }, (_, i) => i) as index (index)}
		<div
			class="rounded-lg border p-4"
			style="background-color: var(--color-bg); border-color: var(--color-border);"
		>
			<div class="flex items-start gap-3">
				<Skeleton class="h-8 w-8 flex-shrink-0" rounded="md" />

				<div class="min-w-0 flex-1">
					<div class="flex items-center gap-2">
						<Skeleton class="h-4 w-24" />
						<Skeleton class="h-3 w-16" />
					</div>
					<Skeleton class="mt-2 h-5 w-3/4" />
					<div class="mt-3 flex items-center gap-2">
						<Skeleton class="h-1.5 flex-1" rounded="full" />
						<Skeleton class="h-3 w-20" />
					</div>
					<div class="mt-2 space-y-1">
						<Skeleton class="h-3 w-full" />
						<Skeleton class="h-3 w-4/5" />
					</div>
				</div>
			</div>
		</div>
	{/each}
</div>
