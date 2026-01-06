/// <reference types="@testing-library/jest-dom" />
import { render, screen, cleanup } from '@testing-library/svelte'
import { afterEach, describe, it, expect, vi } from 'vitest'
import userEvent from '@testing-library/user-event'
import Button from './Button.svelte'
import { createRawSnippet } from 'svelte'

afterEach(() => {
	cleanup()
})

function textSnippet(text: string) {
	return createRawSnippet(() => ({
		render: () => `<span>${text}</span>`,
	}))
}

describe('Button', () => {
	it('renders with default props', () => {
		render(Button, {
			props: {
				children: textSnippet('Click me'),
			},
		})
		const button = screen.getByRole('button')
		expect(button).toBeInTheDocument()
		expect(button).toHaveTextContent('Click me')
		expect(button).not.toBeDisabled()
	})

	it('renders primary variant by default', () => {
		render(Button, {
			props: {
				children: textSnippet('Primary'),
			},
		})
		const button = screen.getByRole('button')
		expect(button.className).toContain('bg-(--color-accent)')
	})

	it('renders secondary variant', () => {
		render(Button, {
			props: {
				variant: 'secondary',
				children: textSnippet('Secondary'),
			},
		})
		const button = screen.getByRole('button')
		expect(button.className).toContain('bg-(--color-bg-subtle)')
	})

	it('renders ghost variant', () => {
		render(Button, {
			props: {
				variant: 'ghost',
				children: textSnippet('Ghost'),
			},
		})
		const button = screen.getByRole('button')
		expect(button.className).toContain('text-(--color-text-muted)')
	})

	it('is disabled when disabled prop is true', () => {
		render(Button, {
			props: {
				disabled: true,
				children: textSnippet('Disabled'),
			},
		})
		expect(screen.getByRole('button')).toBeDisabled()
	})

	it('is disabled when loading is true', () => {
		render(Button, {
			props: {
				loading: true,
				children: textSnippet('Loading'),
			},
		})
		expect(screen.getByRole('button')).toBeDisabled()
	})

	it('shows spinner when loading', () => {
		render(Button, {
			props: {
				loading: true,
				children: textSnippet('Loading'),
			},
		})
		expect(screen.getByRole('status', { name: 'Loading' })).toBeInTheDocument()
	})

	it('calls onclick handler when clicked', async () => {
		const user = userEvent.setup()
		const handleClick = vi.fn()

		render(Button, {
			props: {
				onclick: handleClick,
				children: textSnippet('Click me'),
			},
		})

		await user.click(screen.getByRole('button'))
		expect(handleClick).toHaveBeenCalledTimes(1)
	})

	it('does not call onclick when disabled', async () => {
		const user = userEvent.setup()
		const handleClick = vi.fn()

		render(Button, {
			props: {
				onclick: handleClick,
				disabled: true,
				children: textSnippet('Click me'),
			},
		})

		await user.click(screen.getByRole('button'))
		expect(handleClick).not.toHaveBeenCalled()
	})

	it('renders small size', () => {
		render(Button, {
			props: {
				size: 'sm',
				children: textSnippet('Small'),
			},
		})
		expect(screen.getByRole('button').className).toContain('h-8')
	})

	it('renders large size', () => {
		render(Button, {
			props: {
				size: 'lg',
				children: textSnippet('Large'),
			},
		})
		expect(screen.getByRole('button').className).toContain('h-12')
	})

	it('has type="button" by default', () => {
		render(Button, {
			props: {
				children: textSnippet('Button'),
			},
		})
		expect(screen.getByRole('button')).toHaveAttribute('type', 'button')
	})

	it('can be type="submit"', () => {
		render(Button, {
			props: {
				type: 'submit',
				children: textSnippet('Submit'),
			},
		})
		expect(screen.getByRole('button')).toHaveAttribute('type', 'submit')
	})
})
