/// <reference types="@testing-library/jest-dom" />
import { render, screen, cleanup } from '@testing-library/svelte'
import { afterEach, describe, it, expect, vi } from 'vitest'
import userEvent from '@testing-library/user-event'
import QueryInput from './QueryInput.svelte'

afterEach(() => {
	cleanup()
})

describe('QueryInput', () => {
	it('renders textarea with default placeholder', () => {
		render(QueryInput)
		expect(screen.getByPlaceholderText('Ask a research question...')).toBeInTheDocument()
	})

	it('renders with custom placeholder', () => {
		render(QueryInput, {
			props: {
				placeholder: 'Custom placeholder',
			},
		})
		expect(screen.getByPlaceholderText('Custom placeholder')).toBeInTheDocument()
	})

	it('displays character count', () => {
		render(QueryInput)
		expect(screen.getByText('0 / 2,000')).toBeInTheDocument()
	})

	it('updates character count as user types', async () => {
		const user = userEvent.setup()
		render(QueryInput)

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, 'Hello')

		expect(screen.getByText('5 / 2,000')).toBeInTheDocument()
	})

	it('shows Research button', () => {
		render(QueryInput)
		expect(screen.getByRole('button', { name: /research/i })).toBeInTheDocument()
	})

	it('disables button when textarea is empty', () => {
		render(QueryInput)
		expect(screen.getByRole('button', { name: /research/i })).toBeDisabled()
	})

	it('enables button when textarea has content', async () => {
		const user = userEvent.setup()
		render(QueryInput)

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, 'What is AI?')

		expect(screen.getByRole('button', { name: /research/i })).not.toBeDisabled()
	})

	it('calls onsubmit with trimmed query when button clicked', async () => {
		const user = userEvent.setup()
		const handleSubmit = vi.fn()

		render(QueryInput, {
			props: {
				onsubmit: handleSubmit,
			},
		})

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, '  What is AI?  ')
		await user.click(screen.getByRole('button', { name: /research/i }))

		expect(handleSubmit).toHaveBeenCalledWith('What is AI?')
	})

	it('submits on Ctrl+Enter', async () => {
		const user = userEvent.setup()
		const handleSubmit = vi.fn()

		render(QueryInput, {
			props: {
				onsubmit: handleSubmit,
			},
		})

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, 'What is AI?')
		await user.keyboard('{Control>}{Enter}{/Control}')

		expect(handleSubmit).toHaveBeenCalledWith('What is AI?')
	})

	it('submits on Cmd+Enter (Mac)', async () => {
		const user = userEvent.setup()
		const handleSubmit = vi.fn()

		render(QueryInput, {
			props: {
				onsubmit: handleSubmit,
			},
		})

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, 'What is AI?')
		await user.keyboard('{Meta>}{Enter}{/Meta}')

		expect(handleSubmit).toHaveBeenCalledWith('What is AI?')
	})

	it('shows clear button when there is input', async () => {
		const user = userEvent.setup()
		render(QueryInput)

		expect(screen.queryByLabelText('Clear input')).not.toBeInTheDocument()

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, 'Hello')

		expect(screen.getByLabelText('Clear input')).toBeInTheDocument()
	})

	it('clears input when clear button is clicked', async () => {
		const user = userEvent.setup()
		render(QueryInput)

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, 'Hello')
		await user.click(screen.getByLabelText('Clear input'))

		expect(textarea).toHaveValue('')
	})

	it('disables button when loading', () => {
		render(QueryInput, {
			props: {
				loading: true,
			},
		})
		expect(screen.getByRole('button', { name: /research/i })).toBeDisabled()
	})

	it('disables button when disabled', async () => {
		const user = userEvent.setup()
		render(QueryInput, {
			props: {
				disabled: true,
			},
		})

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, 'Hello')

		expect(screen.getByRole('button', { name: /research/i })).toBeDisabled()
	})

	it('shows error style when over character limit', async () => {
		const user = userEvent.setup()
		render(QueryInput, {
			props: {
				maxLength: 10,
			},
		})

		const textarea = screen.getByRole('textbox')
		await user.type(textarea, 'This is way too long')

		expect(screen.getByRole('button', { name: /research/i })).toBeDisabled()
	})

	it('has data-search-input attribute for keyboard shortcut', () => {
		render(QueryInput)
		const textarea = screen.getByRole('textbox')
		expect(textarea).toHaveAttribute('data-search-input')
	})
})
