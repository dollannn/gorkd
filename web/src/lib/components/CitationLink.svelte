<script lang="ts">
	interface Props {
		number: number
		sourceId: string
		sourceTitle: string
		sourceDomain: string
		onClickCitation?: (sourceId: string) => void
	}

	const { number, sourceId, sourceTitle, sourceDomain, onClickCitation }: Props = $props()

	let showTooltip = $state(false)
	let tooltipPosition = $state<'top' | 'bottom'>('top')

	function handleClick(event: MouseEvent) {
		event.preventDefault()
		onClickCitation?.(sourceId)
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault()
			onClickCitation?.(sourceId)
		}
	}

	function handleMouseEnter(event: MouseEvent) {
		const target = event.currentTarget as HTMLElement
		const rect = target.getBoundingClientRect()
		const spaceAbove = rect.top
		const spaceBelow = window.innerHeight - rect.bottom

		tooltipPosition = spaceAbove > spaceBelow ? 'top' : 'bottom'
		showTooltip = true
	}

	function handleMouseLeave() {
		showTooltip = false
	}

	function handleFocus() {
		showTooltip = true
	}

	function handleBlur() {
		showTooltip = false
	}
</script>

<span class="citation-wrapper relative inline">
	<button
		type="button"
		class="citation-link mx-0.5 inline-flex cursor-pointer items-center justify-center rounded px-1 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-1"
		style="
			background-color: var(--color-accent);
			color: white;
			--tw-ring-color: var(--color-accent);
		"
		onclick={handleClick}
		onkeydown={handleKeydown}
		onmouseenter={handleMouseEnter}
		onmouseleave={handleMouseLeave}
		onfocus={handleFocus}
		onblur={handleBlur}
		aria-label="Citation {number}: {sourceTitle}"
		aria-describedby="tooltip-{sourceId}"
		data-source-id={sourceId}
	>
		[{number}]
	</button>

	{#if showTooltip}
		<div
			id="tooltip-{sourceId}"
			role="tooltip"
			class="citation-tooltip pointer-events-none absolute z-50 w-64 rounded-lg border p-2 shadow-lg"
			class:bottom-full={tooltipPosition === 'top'}
			class:top-full={tooltipPosition === 'bottom'}
			class:mb-2={tooltipPosition === 'top'}
			class:mt-2={tooltipPosition === 'bottom'}
			style="
				left: 50%;
				transform: translateX(-50%);
				background-color: var(--color-bg);
				border-color: var(--color-border);
			"
		>
			<p class="line-clamp-2 text-sm font-medium" style="color: var(--color-text);">
				{sourceTitle}
			</p>
			<p class="mt-1 text-xs" style="color: var(--color-text-muted);">
				{sourceDomain}
			</p>
			<div
				class="tooltip-arrow absolute h-2 w-2 rotate-45 border"
				class:bottom-[-5px]={tooltipPosition === 'top'}
				class:top-[-5px]={tooltipPosition === 'bottom'}
				class:border-t-0={tooltipPosition === 'top'}
				class:border-l-0={tooltipPosition === 'top'}
				class:border-b-0={tooltipPosition === 'bottom'}
				class:border-r-0={tooltipPosition === 'bottom'}
				style="
					left: 50%;
					transform: translateX(-50%) rotate(45deg);
					background-color: var(--color-bg);
					border-color: var(--color-border);
				"
			></div>
		</div>
	{/if}
</span>

<style>
	.citation-link:hover {
		filter: brightness(1.1);
	}

	.citation-tooltip {
		animation: tooltip-fade-in 0.15s ease-out;
	}

	@keyframes tooltip-fade-in {
		from {
			opacity: 0;
			transform: translateX(-50%) translateY(4px);
		}
		to {
			opacity: 1;
			transform: translateX(-50%) translateY(0);
		}
	}
</style>
