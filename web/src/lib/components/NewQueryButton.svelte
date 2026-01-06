<script lang="ts">
	import { ArrowLeft } from 'lucide-svelte'
	import { goto } from '$app/navigation'
	import { resolveRoute } from '$app/paths'
	import Button from './Button.svelte'

	interface Props {
		onReset?: () => void
	}

	const { onReset }: Props = $props()

	function handleClick() {
		onReset?.()
		goto(resolveRoute('/'))
	}
</script>

<Button variant="ghost" size="sm" onclick={handleClick}>
	<ArrowLeft class="h-4 w-4" />
	New Research
</Button>
