import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import RulebookEditor from './RulebookEditor.vue'
import { createMockRulebook, createMockEntryPoint } from '../test/utils'

describe('RulebookEditor', () => {
  describe('Basic Rendering', () => {
    it('renders with empty rulebook', () => {
      const wrapper = mount(RulebookEditor, {
        props: {
          rulebookName: 'test-rulebook',
          data: createMockRulebook(),
        },
      })

      expect(wrapper.find('input').exists()).toBe(true)
      expect(wrapper.find('textarea').exists()).toBe(true)
    })

    it('displays rulebook name in header', () => {
      const wrapper = mount(RulebookEditor, {
        props: {
          rulebookName: 'my-awesome-rulebook',
          data: createMockRulebook(),
        },
      })

      expect(wrapper.find('h2').text()).toBe('my-awesome-rulebook')
    })
  })

  describe('Entry Points Display', () => {
    it('shows empty state when no entry points', () => {
      const wrapper = mount(RulebookEditor, {
        props: {
          rulebookName: 'test',
          data: createMockRulebook({ entry_points: [] }),
        },
      })

      expect(wrapper.text()).toContain('No entry points')
    })

    it('displays entry points count', () => {
      const wrapper = mount(RulebookEditor, {
        props: {
          rulebookName: 'test',
          data: createMockRulebook({
            entry_points: [
              createMockEntryPoint(),
              createMockEntryPoint(),
            ],
          }),
        },
      })

      expect(wrapper.text()).toContain('Entry Points (2)')
    })

    it('calculates total weight correctly', () => {
      const wrapper = mount(RulebookEditor, {
        props: {
          rulebookName: 'test',
          data: createMockRulebook({
            entry_points: [
              createMockEntryPoint({ weight: 1.5 }),
              createMockEntryPoint({ weight: 2.0 }),
              createMockEntryPoint({ weight: 0.5 }),
            ],
          }),
        },
      })

      expect(wrapper.text()).toContain('Total Weight: 4.00')
    })
  })

  describe('Context Defaults Display', () => {
    it('displays existing context defaults', () => {
      const wrapper = mount(RulebookEditor, {
        props: {
          rulebookName: 'test',
          data: createMockRulebook({
            context_defaults: {
              theme: 'nature',
              mood: 'peaceful',
            },
          }),
        },
      })

      expect(wrapper.text()).toContain('theme')
      expect(wrapper.text()).toContain('nature')
      expect(wrapper.text()).toContain('mood')
      expect(wrapper.text()).toContain('peaceful')
    })
  })

  describe('User Interactions', () => {
    it('emits close event when close button clicked', async () => {
      const wrapper = mount(RulebookEditor, {
        props: {
          rulebookName: 'test',
          data: createMockRulebook(),
        },
      })

      await wrapper.find('.btn-close').trigger('click')

      expect(wrapper.emitted('close')).toBeTruthy()
    })
  })
})

