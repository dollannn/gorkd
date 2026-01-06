<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg'
	}

	const { size = 'md' }: Props = $props()

	const sizeClasses = {
		sm: 'text-lg',
		md: 'text-2xl',
		lg: 'text-4xl',
	}
</script>

<span
	class="font-bold tracking-tight {sizeClasses[size]}"
	style="color: var(--color-text);"
>
	gorkd
</span>
