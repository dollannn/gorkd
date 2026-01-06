<script lang="ts">
	interface Props {
		label?: string
		error?: string
		placeholder?: string
		value?: string
		type?: string
		id?: string
		name?: string
		required?: boolean
		oninput?: (e: Event) => void
	}

	let {
		label,
		error,
		placeholder = '',
		value = $bindable(''),
		type = 'text',
		id,
		name,
		required = false,
		oninput,
	}: Props = $props()

	const generatedId = `input-${Math.random().toString(36).slice(2, 9)}`
	const inputId = $derived(id ?? generatedId)
</script>

<div class="flex flex-col gap-1.5">
	{#if label}
		<label
			for={inputId}
			class="text-sm font-medium"
			style="color: var(--color-text);"
		>
			{label}
			{#if required}
				<span style="color: var(--color-error);">*</span>
			{/if}
		</label>
	{/if}

	<input
		{type}
		id={inputId}
		{name}
		{placeholder}
		{required}
		bind:value
		{oninput}
		class="h-10 w-full rounded-md border px-3 text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-1"
		class:border-error={error}
		style="
			background-color: var(--color-bg);
			border-color: {error ? 'var(--color-error)' : 'var(--color-border)'};
			color: var(--color-text);
		"
		style:--tw-ring-color="var(--color-accent)"
	/>

	{#if error}
		<span class="text-sm" style="color: var(--color-error);">
			{error}
		</span>
	{/if}
</div>
