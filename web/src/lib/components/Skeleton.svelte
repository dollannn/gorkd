<script lang="ts">
	interface Props {
		class?: string
		width?: string
		height?: string
		rounded?: 'none' | 'sm' | 'md' | 'lg' | 'full'
	}

	const {
		class: className = '',
		width,
		height,
		rounded = 'md',
	}: Props = $props()

	const roundedClasses = {
		none: '',
		sm: 'rounded-sm',
		md: 'rounded',
		lg: 'rounded-lg',
		full: 'rounded-full',
	}
</script>

<div
	class="animate-pulse {roundedClasses[rounded]} {className}"
	style="
		background-color: var(--color-bg-muted);
		{width ? `width: ${width};` : ''}
		{height ? `height: ${height};` : ''}
	"
	aria-hidden="true"
></div>

<style>
	@keyframes pulse {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.animate-pulse {
		animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}
</style>
