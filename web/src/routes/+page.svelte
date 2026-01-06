<script lang="ts">
	import { onMount } from 'svelte'
	import { Card, ErrorState } from '$lib/components'
	import QueryInput from '$lib/components/QueryInput.svelte'
	import ExampleQueries from '$lib/components/ExampleQueries.svelte'
	import ApiStatusBanner from '$lib/components/ApiStatusBanner.svelte'
	import { researchStore } from '$lib/stores/research.svelte'
	import { healthStore } from '$lib/stores/health.svelte'

	let query = $state('')

	const isSubmitting = $derived(researchStore.state === 'submitting')
	const apiUnavailable = $derived(!healthStore.isAvailable && healthStore.status !== 'unknown' && healthStore.status !== 'checking')
	const isCheckingHealth = $derived(healthStore.status === 'checking')

	onMount(() => {
		healthStore.startPolling()
		return () => healthStore.stopPolling()
	})

	function handleSubmit(queryText: string) {
		researchStore.submit(queryText)
	}

	function handleExampleSelect(example: string) {
		query = example
	}

	function getErrorType(code: string | undefined) {
		switch (code) {
			case 'NETWORK_ERROR':
			case 'TIMEOUT':
				return 'network' as const
			case 'RATE_LIMITED':
				return 'rate_limited' as const
			default:
				return 'generic' as const
		}
	}
</script>

<svelte:head>
	<title>gorkd - Research anything</title>
</svelte:head>

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
			{apiUnavailable}
			onsubmit={handleSubmit}
		/>

		{#if apiUnavailable}
			<div class="mt-4">
				<ApiStatusBanner
					onRetry={() => healthStore.check()}
					isRetrying={isCheckingHealth}
				/>
			</div>
		{/if}
	</Card>

	<ExampleQueries onselect={handleExampleSelect} />

	{#if researchStore.state === 'error' && researchStore.error}
		<ErrorState
			type={getErrorType(researchStore.error.code)}
			message={researchStore.error.message}
			onRetry={() => researchStore.retry()}
		/>
	{/if}
</div>
