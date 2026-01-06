<script lang="ts">
	import { Check } from 'lucide-svelte'

	type Stage =
		| 'pending'
		| 'planning'
		| 'searching'
		| 'fetching'
		| 'synthesizing'
		| 'completed'

	interface Props {
		currentStage: Stage
		message?: string
		progress?: number
	}

	const { currentStage, message, progress }: Props = $props()

	const stages: { key: Stage; label: string }[] = [
		{ key: 'planning', label: 'Planning' },
		{ key: 'searching', label: 'Searching' },
		{ key: 'synthesizing', label: 'Synthesizing' },
		{ key: 'completed', label: 'Complete' },
	]

	const stageOrder: Record<Stage, number> = {
		pending: 0,
		planning: 1,
		searching: 2,
		fetching: 2,
		synthesizing: 3,
		completed: 4,
	}

	function getStageState(
		stage: Stage,
		current: Stage
	): 'pending' | 'active' | 'complete' {
		const currentOrder = stageOrder[current]
		const stageOrderNum = stageOrder[stage]

		if (stageOrderNum < currentOrder) return 'complete'
		if (stageOrderNum === currentOrder) return 'active'
		return 'pending'
	}
</script>

<div class="space-y-4">
	<div class="flex items-center justify-between">
		{#each stages as stage, index (stage.key)}
			{@const state = getStageState(stage.key, currentStage)}
			<div class="flex flex-1 items-center">
				<div class="flex flex-col items-center gap-1">
					<div
						class="flex h-8 w-8 items-center justify-center rounded-full border-2 transition-all duration-300"
						class:animate-pulse={state === 'active'}
						style={state === 'complete'
							? 'background-color: var(--color-success); border-color: var(--color-success);'
							: state === 'active'
								? 'background-color: var(--color-accent); border-color: var(--color-accent);'
								: 'background-color: transparent; border-color: var(--color-border);'}
					>
						{#if state === 'complete'}
							<Check class="h-4 w-4 text-white" />
						{:else if state === 'active'}
							<div class="h-2 w-2 rounded-full bg-white"></div>
						{/if}
					</div>
					<span
						class="text-xs font-medium"
						style={state === 'active'
							? 'color: var(--color-accent);'
							: state === 'complete'
								? 'color: var(--color-success);'
								: 'color: var(--color-text-muted);'}
					>
						{stage.label}
					</span>
				</div>

				{#if index < stages.length - 1}
					<div
						class="mx-2 h-0.5 flex-1 transition-colors duration-300"
						style={stageOrder[stage.key] < stageOrder[currentStage]
							? 'background-color: var(--color-success);'
							: 'background-color: var(--color-border);'}
					></div>
				{/if}
			</div>
		{/each}
	</div>

	{#if message && currentStage !== 'completed'}
		<div class="text-center">
			<p class="text-sm" style="color: var(--color-text-muted);">
				{message}
			</p>
			{#if progress !== undefined}
				<div
					class="mx-auto mt-2 h-1.5 w-48 overflow-hidden rounded-full"
					style="background-color: var(--color-border);"
				>
					<div
						class="h-full rounded-full transition-all duration-300"
						style="background-color: var(--color-accent); width: {Math.round(
							progress * 100
						)}%;"
					></div>
				</div>
			{/if}
		</div>
	{/if}
</div>
