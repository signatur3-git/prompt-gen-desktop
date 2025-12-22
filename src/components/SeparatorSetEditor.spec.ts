import { describe, it, expect, beforeEach } from 'vitest'
import { mount, VueWrapper } from '@vue/test-utils'
import SeparatorSetEditor from './SeparatorSetEditor.vue'

describe('SeparatorSetEditor', () => {
  let wrapper: VueWrapper

  const createWrapper = (props = {}) => {
    return mount(SeparatorSetEditor, {
      props: {
        separatorName: 'test-separator',
        data: {
          name: 'test-separator',
          primary: ', ',
          secondary: ' and ',
          tertiary: null,
        },
        ...props,
      },
    })
  }

  describe('Basic Rendering', () => {
    beforeEach(() => {
      wrapper = createWrapper()
    })

    it('renders with separator name in header', () => {
      expect(wrapper.find('h2').text()).toContain('test-separator')
    })

    it('shows primary separator input', () => {
      const input = wrapper.find('#primary')
      expect(input.exists()).toBe(true)
      expect((input.element as HTMLInputElement).value).toBe(', ')
    })

    it('shows secondary separator input', () => {
      const input = wrapper.find('#secondary')
      expect(input.exists()).toBe(true)
      expect((input.element as HTMLInputElement).value).toBe(' and ')
    })

    it('shows tertiary separator input', () => {
      const input = wrapper.find('#tertiary')
      expect(input.exists()).toBe(true)
    })

    it('displays preview section', () => {
      expect(wrapper.text()).toContain('Preview')
      expect(wrapper.text()).toContain('One item:')
      expect(wrapper.text()).toContain('Two items:')
      expect(wrapper.text()).toContain('Three items:')
      expect(wrapper.text()).toContain('Four items:')
    })
  })

  describe('Preview Formatting - Without Tertiary', () => {
    beforeEach(() => {
      wrapper = createWrapper({
        data: {
          name: 'comma_and',
          primary: ', ',
          secondary: ' and ',
          tertiary: null,
        },
      })
    })

    it('formats one item correctly', () => {
      const previews = wrapper.findAll('.preview-output')
      expect(previews[0].text()).toBe('A')
    })

    it('formats two items with secondary separator', () => {
      const previews = wrapper.findAll('.preview-output')
      expect(previews[1].text()).toBe('A and B')
    })

    it('formats three items without tertiary (uses secondary)', () => {
      const previews = wrapper.findAll('.preview-output')
      expect(previews[2].text()).toBe('A, B and C')
    })

    it('formats four items without tertiary (uses secondary)', () => {
      const previews = wrapper.findAll('.preview-output')
      expect(previews[3].text()).toBe('A, B, C and D')
    })
  })

  describe('Preview Formatting - With Tertiary (Oxford Comma)', () => {
    beforeEach(() => {
      wrapper = createWrapper({
        data: {
          name: 'oxford_comma',
          primary: ', ',
          secondary: ' and ',
          tertiary: ', and ',
        },
      })
    })

    it('formats one item correctly', () => {
      const previews = wrapper.findAll('.preview-output')
      expect(previews[0].text()).toBe('A')
    })

    it('formats two items with secondary (NOT tertiary)', () => {
      const previews = wrapper.findAll('.preview-output')
      // This is the key test: two items should use secondary, not tertiary
      expect(previews[1].text()).toBe('A and B')
      expect(previews[1].text()).not.toBe('A, and B')
    })

    it('formats three items with tertiary (Oxford comma)', () => {
      const previews = wrapper.findAll('.preview-output')
      // This is the key test: three items should use tertiary
      expect(previews[2].text()).toBe('A, B, and C')
      expect(previews[2].text()).not.toBe('A, B and C')
    })

    it('formats four items with tertiary (Oxford comma)', () => {
      const previews = wrapper.findAll('.preview-output')
      // This is the key test: four items should use tertiary
      expect(previews[3].text()).toBe('A, B, C, and D')
      expect(previews[3].text()).not.toBe('A, B, C and D')
    })

    it('shows real example with Oxford comma', () => {
      const realExample = wrapper.find('.example-output')
      expect(realExample.text()).toBe('red, blue, green, and yellow')
    })
  })

  describe('Preview Formatting - Edge Cases', () => {
    it('handles empty separators gracefully', () => {
      wrapper = createWrapper({
        data: {
          name: 'empty',
          primary: '',
          secondary: '',
          tertiary: null,
        },
      })

      const previews = wrapper.findAll('.preview-output')
      expect(previews[0].text()).toBe('A')
      expect(previews[1].text()).toBe('AB')
      expect(previews[2].text()).toBe('ABC')
    })

    it('handles only primary separator defined', () => {
      wrapper = createWrapper({
        data: {
          name: 'primary_only',
          primary: ', ',
          secondary: null,
          tertiary: null,
        },
      })

      const previews = wrapper.findAll('.preview-output')
      expect(previews[1].text()).toBe('A, B') // Falls back to primary
      expect(previews[2].text()).toBe('A, B, C') // Falls back to primary
    })

    it('handles unusual tertiary separator (semicolon)', () => {
      wrapper = createWrapper({
        data: {
          name: 'semicolon_final',
          primary: ', ',
          secondary: ' and ',
          tertiary: '; ',
        },
      })

      const previews = wrapper.findAll('.preview-output')
      expect(previews[1].text()).toBe('A and B') // Two items use secondary
      expect(previews[2].text()).toBe('A, B; C') // Three+ use tertiary
      expect(previews[3].text()).toBe('A, B, C; D')
    })
  })

  describe('Presets', () => {
    beforeEach(() => {
      wrapper = createWrapper()
    })

    it('applies comma_and preset', async () => {
      const buttons = wrapper.findAll('.preset-btn')
      const commaAndBtn = buttons.find(btn => btn.text().includes('Comma And'))
      await commaAndBtn?.trigger('click')

      const previews = wrapper.findAll('.preview-output')
      expect(previews[2].text()).toBe('A, B and C')
    })

    it('applies oxford_comma preset', async () => {
      const buttons = wrapper.findAll('.preset-btn')
      const oxfordBtn = buttons.find(btn => btn.text().includes('Oxford Comma'))
      await oxfordBtn?.trigger('click')

      const previews = wrapper.findAll('.preview-output')
      expect(previews[1].text()).toBe('A and B') // Two items: secondary
      expect(previews[2].text()).toBe('A, B, and C') // Three+ items: tertiary
      expect(previews[3].text()).toBe('A, B, C, and D')
    })

    it('applies comma_or preset', async () => {
      const buttons = wrapper.findAll('.preset-btn')
      const commaOrBtn = buttons.find(btn => btn.text().includes('Comma Or'))
      await commaOrBtn?.trigger('click')

      const previews = wrapper.findAll('.preview-output')
      expect(previews[2].text()).toBe('A, B or C')
    })

    it('applies comma_only preset', async () => {
      const buttons = wrapper.findAll('.preset-btn')
      const commaOnlyBtn = buttons.find(btn => btn.text().includes('Comma Only'))
      await commaOnlyBtn?.trigger('click')

      const previews = wrapper.findAll('.preview-output')
      expect(previews[2].text()).toBe('A, B, C')
    })
  })

  describe('User Interactions', () => {
    beforeEach(() => {
      wrapper = createWrapper()
    })

    it('emits close event when close button clicked', async () => {
      await wrapper.find('.btn-secondary').trigger('click')
      expect(wrapper.emitted('close')).toBeTruthy()
    })

    it('emits update event when primary separator changes', async () => {
      const input = wrapper.find('#primary')
      await input.setValue('; ')

      expect(wrapper.emitted('update')).toBeTruthy()
      const updateEvent = wrapper.emitted('update')?.[0]?.[0] as any
      expect(updateEvent.primary).toBe('; ')
    })

    it('emits update event when tertiary separator changes', async () => {
      const input = wrapper.find('#tertiary')
      await input.setValue(', and ')

      expect(wrapper.emitted('update')).toBeTruthy()
      const updateEvent = wrapper.emitted('update')?.[0]?.[0] as any
      expect(updateEvent.tertiary).toBe(', and ')
    })
  })

  describe('Regression Tests - Tertiary Separator Bug', () => {
    it('BUG FIX: tertiary separator should NOT be used for 2 items', () => {
      wrapper = createWrapper({
        data: {
          name: 'oxford_comma',
          primary: ', ',
          secondary: ' and ',
          tertiary: ', and ',
        },
      })

      const previews = wrapper.findAll('.preview-output')
      // BEFORE FIX: Would have been "A, and B" (wrong)
      // AFTER FIX: Should be "A and B" (correct)
      expect(previews[1].text()).toBe('A and B')
      expect(previews[1].text()).not.toContain(', and')
    })

    it('BUG FIX: tertiary separator MUST be used for 3+ items when defined', () => {
      wrapper = createWrapper({
        data: {
          name: 'oxford_comma',
          primary: ', ',
          secondary: ' and ',
          tertiary: ', and ',
        },
      })

      const previews = wrapper.findAll('.preview-output')

      // Three items
      // BEFORE FIX: Would have been "A, B and C" (wrong - missing comma)
      // AFTER FIX: Should be "A, B, and C" (correct - Oxford comma)
      expect(previews[2].text()).toBe('A, B, and C')
      expect(previews[2].text()).not.toBe('A, B and C')

      // Four items
      // BEFORE FIX: Would have been "A, B, C and D" (wrong - missing comma)
      // AFTER FIX: Should be "A, B, C, and D" (correct - Oxford comma)
      expect(previews[3].text()).toBe('A, B, C, and D')
      expect(previews[3].text()).not.toBe('A, B, C and D')
    })

    it('BUG FIX: without tertiary, should fall back to secondary for 3+ items', () => {
      wrapper = createWrapper({
        data: {
          name: 'comma_and',
          primary: ', ',
          secondary: ' and ',
          tertiary: null,
        },
      })

      const previews = wrapper.findAll('.preview-output')
      // Without tertiary defined, should use secondary
      expect(previews[2].text()).toBe('A, B and C')
      expect(previews[3].text()).toBe('A, B, C and D')
    })
  })
})

