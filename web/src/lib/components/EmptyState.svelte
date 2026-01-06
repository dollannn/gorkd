<script lang="ts">
	import { Inbox, Search, FileQuestion, FolderOpen } from 'lucide-svelte'
	import type { Snippet } from 'svelte'
	import Button from './Button.svelte'

	type EmptyType = 'no_results' | 'no_sources' | 'no_data' | 'generic'

	interface Props {
		type?: EmptyType
		title?: string
		message?: string
		action?: Snippet
		onAction?: () => void
		actionLabel?: string
	}

	const {
		type = 'generic',
		title,
		message,
		action,
		onAction,
		actionLabel,
	}: Props = $props()

	const emptyDefaults: Record<EmptyType, { defaultTitle: string; defaultMessage: string }> = {
		no_results: {
			defaultTitle: 'No results found',
			defaultMessage: 'Try adjusting your search query or try a different question.',
		},
		no_sources: {
			defaultTitle: 'No sources available',
			defaultMessage: 'We couldn\'t find any relevant sources for this query.',
		},
		no_data: {
			defaultTitle: 'Nothing here yet',
			defaultMessage: 'Start by asking a question to see results.',
		},
		generic: {
			defaultTitle: 'Nothing to show',
			defaultMessage: 'There\'s no content to display at the moment.',
		},
	}

	const config = $derived(emptyDefaults[type])
	const displayTitle = $derived(title ?? config.defaultTitle)
	const displayMessage = $derived(message ?? config.defaultMessage)
</script>

<div
	class="flex flex-col items-center gap-4 rounded-lg border border-dashed p-8 text-center"
	style="background-color: var(--color-bg-subtle); border-color: var(--color-border);"
>
	<div
		class="flex h-12 w-12 items-center justify-center rounded-full"
		style="background-color: var(--color-bg-muted);"
	>
		{#if type === 'no_results'}
			<Search class="h-6 w-6" style="color: var(--color-text-muted);" />
		{:else if type === 'no_sources'}
			<FileQuestion class="h-6 w-6" style="color: var(--color-text-muted);" />
		{:else if type === 'no_data'}
			<FolderOpen class="h-6 w-6" style="color: var(--color-text-muted);" />
		{:else}
			<Inbox class="h-6 w-6" style="color: var(--color-text-muted);" />
		{/if}
	</div>

	<div class="space-y-2">
		<h3 class="text-lg font-semibold" style="color: var(--color-text);">
			{displayTitle}
		</h3>
		<p class="max-w-sm text-sm" style="color: var(--color-text-muted);">
			{displayMessage}
		</p>
	</div>

	{#if action}
		{@render action()}
	{:else if onAction && actionLabel}
		<Button variant="secondary" onclick={onAction}>
			{actionLabel}
		</Button>
	{/if}
</div>
