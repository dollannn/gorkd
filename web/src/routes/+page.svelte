<script lang="ts">
	import { Card } from '$lib/components'
	import QueryInput from '$lib/components/QueryInput.svelte'
	import ExampleQueries from '$lib/components/ExampleQueries.svelte'
	import { researchStore } from '$lib/stores/research.svelte'

	let query = $state('')

	const isSubmitting = $derived(researchStore.state === 'submitting')

	function handleSubmit(queryText: string) {
		researchStore.submit(queryText)
	}

	function handleExampleSelect(example: string) {
		query = example
	}
</script>

<div class="space-y-8">
	<section class="text-center">
		<h1 class="mb-2 text-3xl font-bold" style="color: var(--color-text);">
			Research anything
		</h1>
		<p style="color: var(--color-text-muted);">
			Ask a question, get a cited answer with sources.
		</p>
	</section>

	<Card>
		<QueryInput
			bind:value={query}
			loading={isSubmitting}
			onsubmit={handleSubmit}
		/>
	</Card>

	<ExampleQueries onselect={handleExampleSelect} />

	{#if researchStore.state === 'error' && researchStore.error}
		<Card>
			<div class="flex flex-col items-center gap-4 py-4 text-center">
				<p style="color: var(--color-error);">
					{researchStore.error.message}
				</p>
				<button
					type="button"
					onclick={() => researchStore.retry()}
					class="rounded-md px-4 py-2 text-sm font-medium transition-colors"
					style="background-color: var(--color-accent); color: white;"
				>
					Try again
				</button>
			</div>
		</Card>
	{/if}
</div>
