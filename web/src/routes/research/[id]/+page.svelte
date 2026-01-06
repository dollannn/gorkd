<script lang="ts">
	import { onMount, onDestroy } from 'svelte'
	import { goto } from '$app/navigation'
	import { resolveRoute } from '$app/paths'
	import { AlertCircle, ArrowLeft } from 'lucide-svelte'
	import {
		Card,
		Button,
		Spinner,
		ProgressStages,
		SourcePreview,
		StreamingIndicator,
	} from '$lib/components'
	import {
		researchStore,
		type PipelineStage,
	} from '$lib/stores/research.svelte'

	interface Props {
		data: { jobId: string }
	}

	const { data }: Props = $props()

	onMount(() => {
		researchStore.loadJob(data.jobId)
	})

	onDestroy(() => {
		researchStore.disconnectStream()
	})

	function handleBack() {
		researchStore.reset()
		goto(resolveRoute('/'))
	}

	function handleRetryConnection() {
		researchStore.connectStream(data.jobId)
	}

	const isLoading = $derived(researchStore.state === 'submitting')
	const isStreaming = $derived(researchStore.state === 'streaming')
	const isCompleted = $derived(researchStore.state === 'completed')
	const isError = $derived(researchStore.state === 'error')

	const currentStage = $derived<PipelineStage>(
		researchStore.streamingProgress?.stage ?? 'pending'
	)
	const progressMessage = $derived(researchStore.streamingProgress?.message)
	const progressValue = $derived(researchStore.streamingProgress?.progress)

	const showConnectionIndicator = $derived(
		isStreaming && researchStore.connectionState !== 'disconnected'
	)

	const recentSources = $derived(
		researchStore.streamingSources.slice(-5).reverse()
	)

	const allSources = $derived(
		isCompleted
			? (researchStore.job?.sources ?? [])
			: researchStore.streamingSources
	)
</script>

<svelte:head>
	<title>{researchStore.job?.query ?? 'Research'} - gorkd</title>
</svelte:head>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<button
			type="button"
			onclick={handleBack}
			class="inline-flex items-center gap-2 text-sm transition-colors hover:opacity-70"
			style="color: var(--color-text-muted);"
		>
			<ArrowLeft class="h-4 w-4" />
			New research
		</button>

		{#if showConnectionIndicator}
			<StreamingIndicator
				state={researchStore.connectionState}
				onRetry={handleRetryConnection}
			/>
		{/if}
	</div>

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
			<div class="space-y-6">
				<h2 class="text-lg font-semibold" style="color: var(--color-text);">
					{researchStore.job?.query}
				</h2>

				<ProgressStages
					{currentStage}
					message={progressMessage}
					progress={progressValue}
				/>

				{#if isStreaming && researchStore.streamingAnswer}
					<div
						class="rounded-lg border-l-4 p-4"
						style="background-color: var(--color-bg-subtle); border-color: var(--color-accent);"
					>
						<p
							class="text-sm font-medium"
							style="color: var(--color-text-muted);"
						>
							Preview
						</p>
						<p class="mt-1" style="color: var(--color-text);">
							{researchStore.streamingAnswer.summary}
						</p>
					</div>
				{/if}

				{#if isCompleted && researchStore.job?.answer}
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

		{#if isStreaming && recentSources.length > 0}
			<Card title="Sources found">
				<div class="space-y-2">
					{#each recentSources as source, index (source.id)}
						<SourcePreview
							id={source.id}
							url={source.url}
							title={source.title}
							relevance={source.relevance}
							isNew={index === 0}
						/>
					{/each}
				</div>
				{#if researchStore.streamingSources.length > 5}
					<p
						class="mt-3 text-center text-sm"
						style="color: var(--color-text-muted);"
					>
						+{researchStore.streamingSources.length - 5} more sources
					</p>
				{/if}
			</Card>
		{/if}

		{#if isCompleted && allSources.length > 0}
			<Card title="Sources">
				<ul class="space-y-3">
					{#each allSources as source (source.id)}
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
								{'domain' in source
									? source.domain
									: new URL(source.url).hostname}
							</span>
						</li>
					{/each}
				</ul>
			</Card>
		{/if}
	{/if}
</div>
