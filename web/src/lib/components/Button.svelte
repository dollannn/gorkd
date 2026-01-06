<script lang="ts">
	import type { Snippet } from 'svelte'
	import Spinner from './Spinner.svelte'

	interface Props {
		variant?: 'primary' | 'secondary' | 'ghost'
		size?: 'sm' | 'md' | 'lg'
		disabled?: boolean
		loading?: boolean
		type?: 'button' | 'submit'
		onclick?: () => void
		children: Snippet
	}

	const {
		variant = 'primary',
		size = 'md',
		disabled = false,
		loading = false,
		type = 'button',
		onclick,
		children,
	}: Props = $props()

	const baseClasses =
		'inline-flex items-center justify-center gap-2 font-medium rounded-md transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none'

	const variantClasses = {
		primary:
			'bg-(--color-accent) text-white hover:bg-(--color-accent-hover) focus-visible:ring-(--color-accent)',
		secondary:
			'bg-(--color-bg-subtle) text-(--color-text) border border-(--color-border) hover:bg-(--color-bg-muted) focus-visible:ring-(--color-accent)',
		ghost:
			'text-(--color-text-muted) hover:text-(--color-text) hover:bg-(--color-bg-subtle) focus-visible:ring-(--color-accent)',
	}

	const sizeClasses = {
		sm: 'h-8 px-3 text-sm',
		md: 'h-10 px-4 text-sm',
		lg: 'h-12 px-6 text-base',
	}
</script>

<button
	{type}
	class="{baseClasses} {variantClasses[variant]} {sizeClasses[size]}"
	disabled={disabled || loading}
	{onclick}
>
	{#if loading}
		<Spinner size="sm" />
	{/if}
	{@render children()}
</button>
