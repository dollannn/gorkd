<script lang="ts">
	import { onMount, onDestroy } from 'svelte'
	import {
		Card,
		ProgressStages,
		SourcePreview,
		StreamingIndicator,
		ThemeToggle,
		AnswerCard,
		LimitationsList,
		SourcesPanel,
		MetadataBar,
		ShareButton,
		NewQueryButton,
		ErrorState,
		AnswerSkeleton,
		SourceSkeleton,
	} from '$lib/components'
	import {
		researchStore,
		type PipelineStage,
	} from '$lib/stores/research.svelte'

	interface Props {
		data: { jobId: string }
	}

	const { data }: Props = $props()

	let highlightedSourceId = $state<string | null>(null)

	onMount(() => {
		researchStore.loadJob(data.jobId)
	})

	onDestroy(() => {
		researchStore.disconnectStream()
	})

	function handleReset() {
		researchStore.reset()
	}

	function handleRetryConnection() {
		researchStore.connectStream(data.jobId)
	}

	function handleCitationClick(sourceId: string) {
		highlightedSourceId = sourceId
		const sourceElement = document.getElementById(`source-${sourceId}`)
		if (sourceElement) {
			sourceElement.scrollIntoView({ behavior: 'smooth', block: 'center' })
			setTimeout(() => {
				highlightedSourceId = null
			}, 2000)
		}
	}

	function getErrorType(code: string | undefined) {
		switch (code) {
			case 'NETWORK_ERROR':
			case 'TIMEOUT':
				return 'network' as const
			case 'JOB_NOT_FOUND':
				return 'not_found' as const
			case 'RATE_LIMITED':
				return 'rate_limited' as const
			case 'SEARCH_FAILED':
				return 'search_failed' as const
			case 'LLM_FAILED':
				return 'llm_failed' as const
			default:
				return 'generic' as const
		}
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
</script>

<svelte:head>
	<title>{researchStore.job?.query ?? 'Research'} - gorkd</title>
</svelte:head>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<NewQueryButton onReset={handleReset} />

		<div class="flex items-center gap-2">
			{#if showConnectionIndicator}
				<StreamingIndicator
					state={researchStore.connectionState}
					onRetry={handleRetryConnection}
				/>
			{/if}
			{#if isCompleted}
				<ShareButton />
			{/if}
			<ThemeToggle />
		</div>
	</div>

	{#if researchStore.job?.query}
		<div
			class="rounded-lg border p-4"
			style="background-color: var(--color-bg-subtle); border-color: var(--color-border);"
		>
			<p class="text-sm font-medium" style="color: var(--color-text-muted);">Query</p>
			<p class="mt-1 text-lg font-semibold" style="color: var(--color-text);">
				{researchStore.job.query}
			</p>
		</div>
	{/if}

	{#if isLoading}
		<div class="space-y-6">
			<AnswerSkeleton />
			<SourceSkeleton count={3} />
		</div>
	{:else if isError && researchStore.error}
		<ErrorState
			type={getErrorType(researchStore.error.code)}
			message={researchStore.error.message}
			onRetry={() => researchStore.retry()}
			onNewQuery={handleReset}
		/>
	{:else if isStreaming}
		<Card>
			<div class="space-y-6">
				<ProgressStages
					{currentStage}
					message={progressMessage}
					progress={progressValue}
				/>

				{#if researchStore.streamingAnswer}
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
			</div>
		</Card>

		{#if recentSources.length > 0}
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
	{:else if isCompleted && researchStore.job?.answer}
		<AnswerCard
			summary={researchStore.job.answer.summary}
			detail={researchStore.job.answer.detail}
			confidence={researchStore.job.answer.confidence}
			citations={researchStore.job.citations ?? []}
			sources={researchStore.job.sources ?? []}
			onCitationClick={handleCitationClick}
		/>

		{#if researchStore.job.answer.limitations && researchStore.job.answer.limitations.length > 0}
			<LimitationsList limitations={researchStore.job.answer.limitations} />
		{/if}

		{#if researchStore.job.sources && researchStore.job.sources.length > 0}
			<SourcesPanel
				sources={researchStore.job.sources}
				citations={researchStore.job.citations ?? []}
				{highlightedSourceId}
			/>
		{/if}

		<MetadataBar
			durationMs={researchStore.job.metadata?.duration_ms}
			sourcesConsidered={researchStore.job.metadata?.sources_considered}
			cached={researchStore.job.metadata?.cached}
		/>
	{/if}
</div>
