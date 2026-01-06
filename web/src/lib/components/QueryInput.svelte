<script lang="ts">
	import { X, Search, CornerDownLeft } from 'lucide-svelte'
	import Button from './Button.svelte'

	interface Props {
		value?: string
		placeholder?: string
		maxLength?: number
		loading?: boolean
		disabled?: boolean
		apiUnavailable?: boolean
		onsubmit?: (query: string) => void
	}

	let {
		value = $bindable(''),
		placeholder = 'Ask a research question...',
		maxLength = 2000,
		loading = false,
		disabled = false,
		apiUnavailable = false,
		onsubmit,
	}: Props = $props()

	let textarea: HTMLTextAreaElement | undefined = $state()

	const charCount = $derived(value.length)
	const isOverLimit = $derived(charCount > maxLength)
	const isEmpty = $derived(value.trim().length === 0)
	const canSubmit = $derived(!isEmpty && !isOverLimit && !loading && !disabled && !apiUnavailable)

	function handleKeydown(event: KeyboardEvent) {
		if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
			event.preventDefault()
			if (canSubmit) {
				onsubmit?.(value.trim())
			}
		}
	}

	function handleSubmit() {
		if (canSubmit) {
			onsubmit?.(value.trim())
		}
	}

	function clear() {
		value = ''
		textarea?.focus()
	}

	$effect(() => {
		textarea?.focus()
	})
</script>

<div class="space-y-3">
	<div class="relative">
		<textarea
			bind:this={textarea}
			bind:value
			{placeholder}
			{disabled}
			rows={3}
			onkeydown={handleKeydown}
			data-search-input
			class="w-full resize-none rounded-lg border p-4 pr-10 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-1"
			class:border-error={isOverLimit}
			style="
				background-color: var(--color-bg);
				border-color: {isOverLimit ? 'var(--color-error)' : 'var(--color-border)'};
				color: var(--color-text);
			"
			style:--tw-ring-color="var(--color-accent)"
		></textarea>

		{#if value}
			<button
				type="button"
				onclick={clear}
				class="absolute right-3 top-3 rounded p-1 transition-colors hover:bg-black/5 dark:hover:bg-white/10"
				style="color: var(--color-text-muted);"
				aria-label="Clear input"
			>
				<X class="h-4 w-4" />
			</button>
		{/if}
	</div>

	<div class="flex items-center justify-between">
		<div class="flex items-center gap-4">
			<span
				class="text-sm"
				style="color: {isOverLimit
					? 'var(--color-error)'
					: 'var(--color-text-muted)'};"
			>
				{charCount.toLocaleString()} / {maxLength.toLocaleString()}
			</span>

			<span
				class="hidden text-sm sm:inline"
				style="color: var(--color-text-muted);"
			>
				<kbd
					class="rounded border px-1.5 py-0.5 text-xs"
					style="background-color: var(--color-bg-subtle); border-color: var(--color-border);"
				>
					{navigator?.platform?.includes('Mac') ? '⌘' : 'Ctrl'}
				</kbd>
				+
				<kbd
					class="rounded border px-1.5 py-0.5 text-xs"
					style="background-color: var(--color-bg-subtle); border-color: var(--color-border);"
				>
					Enter
				</kbd>
				to submit
			</span>
		</div>

		<Button
			variant="primary"
			{loading}
			disabled={!canSubmit}
			onclick={handleSubmit}
		>
			<Search class="h-4 w-4" />
			Research
			<CornerDownLeft class="hidden h-3 w-3 sm:inline" />
		</Button>
	</div>
</div>
