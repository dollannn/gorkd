<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg'
	}

	const { size = 'md' }: Props = $props()

	const sizeClasses = {
		sm: 'h-4 w-4 border-2',
		md: 'h-6 w-6 border-2',
		lg: 'h-8 w-8 border-3'
	}
</script>

<div
	class="animate-spin rounded-full {sizeClasses[size]}"
	style="border-color: var(--color-border); border-top-color: var(--color-accent);"
	role="status"
	aria-label="Loading"
></div>
