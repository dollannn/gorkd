<script lang="ts">
	import Skeleton from './Skeleton.svelte'
</script>

<div
	class="rounded-lg border p-4"
	style="background-color: var(--color-bg); border-color: var(--color-border);"
	role="status"
	aria-label="Loading answer"
>
	<div class="mb-3 flex items-center justify-between">
		<Skeleton class="h-4 w-16" />
		<Skeleton class="h-6 w-20" rounded="full" />
	</div>

	<div class="space-y-3">
		<Skeleton class="h-6 w-3/4" />
		<div class="space-y-2">
			<Skeleton class="h-4 w-full" />
			<Skeleton class="h-4 w-full" />
			<Skeleton class="h-4 w-2/3" />
		</div>
	</div>

	<div class="mt-4">
		<Skeleton class="h-4 w-24" />
	</div>
</div>
