<script lang="ts">
	import type { ComponentType, SvelteComponent } from 'svelte'

	interface IconProps {
		size?: number | string
		class?: string
	}

	interface Props {
		icon: ComponentType<SvelteComponent<IconProps>>
		size?: 'sm' | 'md' | 'lg'
		class?: string
	}

	const { icon: IconComponent, size = 'md', class: className = '' }: Props = $props()

	const sizeMap = {
		sm: 16,
		md: 20,
		lg: 24
	}
</script>

<IconComponent size={sizeMap[size]} class={className} />
