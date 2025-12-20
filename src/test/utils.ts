// Test utilities and mock data factories

export const createMockPackage = () => ({
  id: 'test.package',
  version: '1.0.0',
  metadata: {
    name: 'Test Package',
    description: 'A test package',
    authors: ['Test Author'],
    bypass_filters: false,
  },
  namespaces: {
    test: {
      id: 'test',
      datatypes: {},
      prompt_sections: {},
      separator_sets: {},
      rules: [],
      decisions: [],
      rulebooks: {},
    },
  },
  dependencies: [],
})

export const createMockRulebook = (overrides = {}) => ({
  name: 'Test Rulebook',
  description: 'A test rulebook',
  entry_points: [],
  batch_variety: false,
  context_defaults: {},
  ...overrides,
})

export const createMockEntryPoint = (overrides = {}) => ({
  prompt_section: 'test:section',
  weight: 1.0,
  ...overrides,
})

export const createMockRenderResult = (overrides = {}) => ({
  output: 'test output',
  seed: 42,
  selected_values: {},
  ...overrides,
})

export const createMockBatchRenderResult = (overrides = {}) => ({
  output: 'test output',
  seed: 42,
  index: 0,
  ...overrides,
})

