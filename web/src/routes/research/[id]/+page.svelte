<script lang="ts">
	import { onMount } from 'svelte'
	import { goto } from '$app/navigation'
	import { resolveRoute } from '$app/paths'
	import { AlertCircle, ArrowLeft, Loader2 } from 'lucide-svelte'
	import { Card, Button, Spinner } from '$lib/components'
	import { researchStore } from '$lib/stores/research.svelte'

	interface Props {
		data: { jobId: string }
	}

	const { data }: Props = $props()

	onMount(() => {
		researchStore.loadJob(data.jobId)
	})

	function handleBack() {
		researchStore.reset()
		goto(resolveRoute('/'))
	}

	const isLoading = $derived(researchStore.state === 'submitting')
	const isStreaming = $derived(researchStore.state === 'streaming')
	const isCompleted = $derived(researchStore.state === 'completed')
	const isError = $derived(researchStore.state === 'error')
</script>

<svelte:head>
	<title>{researchStore.job?.query ?? 'Research'} - gorkd</title>
</svelte:head>

<div class="space-y-6">
	<button
		type="button"
		onclick={handleBack}
		class="inline-flex items-center gap-2 text-sm transition-colors hover:opacity-70"
		style="color: var(--color-text-muted);"
	>
		<ArrowLeft class="h-4 w-4" />
		New research
	</button>

	{#if isLoading}
		<Card>
			<div class="flex flex-col items-center gap-4 py-12">
				<Spinner size="lg" />
				<p style="color: var(--color-text-muted);">Loading research...</p>
			</div>
		</Card>
	{:else if isError && researchStore.error}
		<Card>
			<div class="flex flex-col items-center gap-4 py-8 text-center">
				<AlertCircle class="h-12 w-12" style="color: var(--color-error);" />
				<div class="space-y-2">
					<h2 class="text-lg font-semibold" style="color: var(--color-text);">
						{researchStore.error.code === 'JOB_NOT_FOUND'
							? 'Research not found'
							: 'Something went wrong'}
					</h2>
					<p style="color: var(--color-text-muted);">
						{researchStore.error.message}
					</p>
				</div>
				<div class="flex gap-3">
					<Button variant="secondary" onclick={handleBack}>
						<ArrowLeft class="h-4 w-4" />
						Go back
					</Button>
					{#if researchStore.error.code !== 'JOB_NOT_FOUND'}
						<Button variant="primary" onclick={() => researchStore.retry()}
							>Try again</Button
						>
					{/if}
				</div>
			</div>
		</Card>
	{:else if isStreaming || isCompleted}
		<Card>
			<div class="space-y-4">
				<h2 class="text-lg font-semibold" style="color: var(--color-text);">
					{researchStore.job?.query}
				</h2>

				{#if isStreaming}
					<div class="flex items-center gap-3 py-8">
						<Loader2
							class="h-5 w-5 animate-spin"
							style="color: var(--color-accent);"
						/>
						<span style="color: var(--color-text-muted);">
							{researchStore.job?.progress?.message ?? 'Researching...'}
						</span>
					</div>
				{:else if isCompleted && researchStore.job?.answer}
					<div class="space-y-4">
						<div
							class="rounded-lg p-4"
							style="background-color: var(--color-bg-subtle); color: var(--color-text);"
						>
							<p class="font-medium">{researchStore.job.answer.summary}</p>
						</div>

						{#if researchStore.job.answer.detail}
							<div class="prose max-w-none" style="color: var(--color-text);">
								<p>{researchStore.job.answer.detail}</p>
							</div>
						{/if}

						<div
							class="flex items-center gap-2 text-sm"
							style="color: var(--color-text-muted);"
						>
							<span
								class="rounded-full px-2 py-0.5"
								class:bg-green-100={researchStore.job.answer.confidence ===
									'high'}
								class:bg-yellow-100={researchStore.job.answer.confidence ===
									'medium'}
								class:bg-orange-100={researchStore.job.answer.confidence ===
									'low'}
								class:bg-red-100={researchStore.job.answer.confidence ===
									'insufficient'}
								class:text-green-800={researchStore.job.answer.confidence ===
									'high'}
								class:text-yellow-800={researchStore.job.answer.confidence ===
									'medium'}
								class:text-orange-800={researchStore.job.answer.confidence ===
									'low'}
								class:text-red-800={researchStore.job.answer.confidence ===
									'insufficient'}
							>
								{researchStore.job.answer.confidence} confidence
							</span>
							{#if researchStore.job.metadata?.duration_ms}
								<span>
									Completed in {(
										researchStore.job.metadata.duration_ms / 1000
									).toFixed(1)}s
								</span>
							{/if}
						</div>
					</div>
				{/if}
			</div>
		</Card>

		{#if isCompleted && researchStore.job?.sources?.length}
			<Card title="Sources">
				<ul class="space-y-3">
					{#each researchStore.job.sources as source (source.id)}
						{@const externalUrl = source.url}
						<li class="flex flex-col gap-1">
							<a
								href={externalUrl}
								target="_blank"
								rel="noopener noreferrer"
								class="font-medium hover:underline"
								style="color: var(--color-accent);"
							>
								{source.title}
							</a>
							<span class="text-sm" style="color: var(--color-text-muted);">
								{source.domain}
							</span>
						</li>
					{/each}
				</ul>
			</Card>
		{/if}
	{/if}
</div>
