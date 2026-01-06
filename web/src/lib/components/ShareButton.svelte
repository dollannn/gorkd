<script lang="ts">
	import { Share2, Check } from 'lucide-svelte'
	import Button from './Button.svelte'

	interface Props {
		url?: string
	}

	const { url }: Props = $props()

	let copied = $state(false)
	let copyTimeout: ReturnType<typeof setTimeout> | null = null

	async function handleShare() {
		const shareUrl = url ?? window.location.href

		try {
			await navigator.clipboard.writeText(shareUrl)
			copied = true

			if (copyTimeout) {
				clearTimeout(copyTimeout)
			}

			copyTimeout = setTimeout(() => {
				copied = false
			}, 2000)
		} catch {
			const textArea = document.createElement('textarea')
			textArea.value = shareUrl
			textArea.style.position = 'fixed'
			textArea.style.left = '-9999px'
			document.body.appendChild(textArea)
			textArea.select()
			document.execCommand('copy')
			document.body.removeChild(textArea)
			copied = true

			if (copyTimeout) {
				clearTimeout(copyTimeout)
			}

			copyTimeout = setTimeout(() => {
				copied = false
			}, 2000)
		}
	}
</script>

<Button variant="secondary" size="sm" onclick={handleShare}>
	{#if copied}
		<Check class="h-4 w-4" style="color: var(--color-success);" />
		Copied!
	{:else}
		<Share2 class="h-4 w-4" />
		Share
	{/if}
</Button>
