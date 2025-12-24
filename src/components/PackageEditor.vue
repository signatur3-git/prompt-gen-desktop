<template>
  <div class="package-editor">
    <!-- Header Bar -->
    <div class="editor-header">
      <div class="header-left">
        <h1>{{ packageName || 'RPG Desktop' }}</h1>
        <span v-if="packageVersion" class="package-version">Package v{{ packageVersion }}</span>
        <span v-else class="app-version">App v{{ APP_VERSION }}</span>
      </div>
      <div class="header-actions">
        <button @click="showNewPackageDialog = true" class="btn-secondary">
          New Package
        </button>
        <button @click="loadPackage" class="btn-secondary">
          Open Package
        </button>
        <button @click="$router.push('/library')" class="btn-secondary" title="Library">
          📚 Library
        </button>
        <button @click="$router.push('/generate')" class="btn-secondary" title="Generate">
          ⚡ Generate
        </button>
        <button @click="savePackage" class="btn-primary" :disabled="!hasChanges">
          Save Package
        </button>
        <button @click="$router.push('/marketplace')" class="btn-secondary" title="Marketplace">
          📦 Marketplace
        </button>
      </div>
    </div>

    <!-- Main Editor Area -->
    <div class="editor-content">
      <div class="sidebar">
        <ComponentTree
          v-if="currentPackage"
          :package="currentPackage"
          @select="onComponentSelect"
          @add-namespace="showAddNamespaceDialog = true"
        />
      </div>

      <div class="main-panel">
        <div v-if="!currentPackage" class="welcome-message">
          <h2>Welcome to RPG Package Editor</h2>
          <p>Create a new package or open an existing one to get started.</p>
          <button @click="showNewPackageDialog = true" class="btn-primary btn-large">
            Create New Package
          </button>
        </div>

        <div v-else-if="selectedComponent">
          <!-- Datatype Editor -->
          <DatatypeEditor
            v-if="selectedComponent.data.type === 'datatype'"
            :datatypeName="selectedComponent.data.dtId"
            :data="selectedComponent.data.data"
            @update="onDatatypeUpdate(selectedComponent.data.nsId, selectedComponent.data.dtId, $event)"
            @close="selectedComponent = null"
          />

          <!-- SeparatorSet Editor -->
          <SeparatorSetEditor
            v-else-if="selectedComponent.data.type === 'separator'"
            :separatorName="selectedComponent.data.sepId"
            :data="selectedComponent.data.data"
            @update="onSeparatorUpdate(selectedComponent.data.nsId, selectedComponent.data.sepId, $event)"
            @close="selectedComponent = null"
          />

          <!-- PromptSection Editor -->
          <PromptSectionEditor
            v-else-if="selectedComponent.data.type === 'promptsection'"
            :promptSectionName="selectedComponent.data.psId"
            :data="selectedComponent.data.data"
            @update="onPromptSectionUpdate(selectedComponent.data.nsId, selectedComponent.data.psId, $event)"
            @close="selectedComponent = null"
          />

          <!-- Rules Editor -->
          <RulesEditor
            v-else-if="selectedComponent.data.type === 'rule'"
            :ruleName="selectedComponent.data.ruleId"
            :data="selectedComponent.data.data"
            @update="onRuleUpdate(selectedComponent.data.nsId, selectedComponent.data.ruleId, $event)"
            @close="selectedComponent = null"
          />

          <!-- Rulebook Editor -->
          <RulebookEditor
            v-else-if="selectedComponent.data.type === 'rulebook'"
            :rulebookName="selectedComponent.data.rbId"
            :data="selectedComponent.data.data"
            @update="onRulebookUpdate(selectedComponent.data.nsId, selectedComponent.data.rbId, $event)"
            @close="selectedComponent = null"
          />

          <!-- New Rulebook Creator -->
          <div v-else-if="selectedComponent.data.type === 'new' && selectedComponent.data.componentType === 'rulebook'" class="new-component-creator">
            <RulebookEditor
              rulebookName="new-rulebook"
              :data="newRulebookData"
              @update="newRulebookData = $event"
              @close="cancelNewComponent"
            />
            <div class="creation-actions">
              <button @click="createRulebook(selectedComponent.data.nsId)" class="btn-create" :disabled="!newRulebookData.name">
                Create Rulebook
              </button>
              <button @click="cancelNewComponent" class="btn-cancel">
                Cancel
              </button>
            </div>
          </div>

          <!-- New Datatype Creator -->
          <div v-else-if="selectedComponent.data.type === 'new' && selectedComponent.data.componentType === 'datatype'" class="new-component-creator">
            <DatatypeEditor
              datatypeName="new-datatype"
              :data="newDatatypeData"
              @update="newDatatypeData = $event"
              @close="cancelNewComponent"
            />
            <div class="creation-actions">
              <button @click="createDatatype(selectedComponent.data.nsId)" class="btn-create" :disabled="!newDatatypeData.name || newDatatypeData.values.length === 0">
                Create Datatype
              </button>
              <button @click="cancelNewComponent" class="btn-cancel">
                Cancel
              </button>
            </div>
          </div>

          <!-- New PromptSection Creator -->
          <div v-else-if="selectedComponent.data.type === 'new' && selectedComponent.data.componentType === 'promptsection'" class="new-component-creator">
            <PromptSectionEditor
              promptSectionName="new-promptsection"
              :data="newPromptSectionData"
              @update="newPromptSectionData = $event"
              @close="cancelNewComponent"
            />
            <div class="creation-actions">
              <button @click="createPromptSection(selectedComponent.data.nsId)" class="btn-create" :disabled="!newPromptSectionData.name || !newPromptSectionData.template">
                Create Prompt Section
              </button>
              <button @click="cancelNewComponent" class="btn-cancel">
                Cancel
              </button>
            </div>
          </div>

          <!-- New SeparatorSet Creator -->
          <div v-else-if="selectedComponent.data.type === 'new' && selectedComponent.data.componentType === 'separator'" class="new-component-creator">
            <SeparatorSetEditor
              separatorName="new-separator"
              :data="newSeparatorSetData"
              @update="newSeparatorSetData = $event"
              @close="cancelNewComponent"
            />
            <div class="creation-actions">
              <button @click="createSeparatorSet(selectedComponent.data.nsId)" class="btn-create" :disabled="!newSeparatorSetData.name || !newSeparatorSetData.primary">
                Create Separator Set
              </button>
              <button @click="cancelNewComponent" class="btn-cancel">
                Cancel
              </button>
            </div>
          </div>

          <!-- New Rule Creator -->
          <div v-else-if="selectedComponent.data.type === 'new' && selectedComponent.data.componentType === 'rule'" class="new-component-creator">
            <RulesEditor
              ruleName="new-rule"
              :data="newRuleData"
              @update="newRuleData = $event"
              @close="cancelNewComponent"
            />
            <div class="creation-actions">
              <button @click="createRule(selectedComponent.data.nsId)" class="btn-create" :disabled="!newRuleData.name || !newRuleData.when || !newRuleData.set">
                Create Rule
              </button>
              <button @click="cancelNewComponent" class="btn-cancel">
                Cancel
              </button>
            </div>
          </div>

          <!-- Package Metadata Editor -->
          <PackageMetadataEditor
            v-else-if="selectedComponent.id === 'package'"
            :data="currentPackage"
            @update="onPackageMetadataUpdate"
            @close="selectedComponent = null"
          />

          <!-- Unknown type placeholder -->
          <div v-else class="placeholder">
            <p>Unknown component type</p>
          </div>
        </div>

        <div v-else class="placeholder">
          <p>Select a component from the sidebar to edit</p>
        </div>
      </div>

      <div class="preview-panel">
        <LivePreview
          v-if="currentPackage"
          :package="currentPackage"
          :dependencies="loadedDependencies"
          :validation-errors="validationErrors"
        />
      </div>
    </div>

    <!-- New Package Dialog -->
    <NewPackageDialog
      v-if="showNewPackageDialog"
      @create="onPackageCreate"
      @cancel="showNewPackageDialog = false"
    />

    <!-- Add Namespace Dialog -->
    <AddNamespaceDialog
      v-if="showAddNamespaceDialog"
      @add="onAddNamespace"
      @cancel="showAddNamespaceDialog = false"
    />


    <!-- Validation Panel (bottom) -->
    <ValidationPanel
      v-if="validationErrors.length > 0 || validationWarnings.length > 0"
      :errors="validationErrors"
      :warnings="validationWarnings"
      @jump-to="jumpToError"
      @close="validationErrors = []; validationWarnings = []"
    />
  </div>
</template>

<script setup lang="ts">
// @ts-nocheck - Functional code, TypeScript migration pending (see MARKETPLACE_INTEGRATION_STATUS.md)
import { ref, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'
import ComponentTree from './ComponentTree.vue'
import NewPackageDialog from './NewPackageDialog.vue'
import AddNamespaceDialog from './AddNamespaceDialog.vue'
import LivePreview from './LivePreview.vue'
import ValidationPanel from './ValidationPanel.vue'
import DatatypeEditor from './DatatypeEditor.vue'
import SeparatorSetEditor from './SeparatorSetEditor.vue'
import PromptSectionEditor from './PromptSectionEditor.vue'
import RulesEditor from './RulesEditor.vue'
import RulebookEditor from './RulebookEditor.vue'
import PackageMetadataEditor from './PackageMetadataEditor.vue'
import MarketplaceSettings from './MarketplaceSettings.vue'
import { marketplaceClient, type MarketplacePackage } from '../services/marketplace-client'
import { useRoute, useRouter } from 'vue-router'
import { useVersion } from '../composables/useVersion'

// Get app version from composable
const { version: APP_VERSION } = useVersion()

// State
const currentPackage = ref(null)
const loadedDependencies = ref({}) // M9 Phase 3: Store loaded dependencies
const selectedComponent = ref(null)
const validationErrors = ref([])
const validationWarnings = ref([])
const hasChanges = ref(false)
const showNewPackageDialog = ref(false)
const showAddNamespaceDialog = ref(false)
const newRulebookData = ref({
  name: '',
  description: '',
  entry_points: [],
  batch_variety: false,
  context_defaults: {}
})
const newDatatypeData = ref({
  name: '',
  values: []
})
const newPromptSectionData = ref({
  name: '',
  template: '',
  references: {}
})
const newSeparatorSetData = ref({
  name: '',
  primary: '',
  secondary: '',
  tertiary: ''
})
const newRuleData = ref({
  name: '',
  when: '',
  logic: '',
  set: '',
  value: ''
})
let validationTimeout = null

// Computed
const packageName = computed(() => currentPackage.value?.metadata?.name)
const packageVersion = computed(() => currentPackage.value?.version)

// Debounced validation to avoid too many calls
function scheduleValidation() {
  console.log('⏰ Validation scheduled (300ms debounce)')
  if (validationTimeout) {
    clearTimeout(validationTimeout)
    console.log('   Clearing previous timeout')
  }
  validationTimeout = setTimeout(() => {
    console.log('⏱️  Debounce complete, running validation now')
    if (currentPackage.value) {
      validatePackage(currentPackage.value)
    }
  }, 300) // 300ms debounce
}

// Watch for changes to trigger validation
watch(currentPackage, async (newPkg) => {
  if (newPkg) {
    scheduleValidation()
  }
}, { deep: true })

// Watch for package changes to clear selection
watch(currentPackage, (newPkg, oldPkg) => {
  // Clear selection when loading a different package
  if (newPkg && oldPkg && newPkg.id !== oldPkg.id) {
    selectedComponent.value = null
  }
})

// Methods
async function loadPackage() {
  try {
    const selected = await open({
      filters: [{
        name: 'Package',
        extensions: ['yaml', 'yml', 'json']
      }]
    })

    if (selected) {
      // Load package with dependencies
      const result = await invoke('load_package_with_dependencies', {
        path: selected,
        searchPaths: ['./packages', './test-packages']
      })
      currentPackage.value = result.package
      loadedDependencies.value = result.dependencies // Store dependencies
      hasChanges.value = false
    }
  } catch (error) {
    console.error('Failed to load package:', error)
    alert(`Failed to load package: ${error}`)
  }
}

async function savePackage() {
  try {
    const path = await save({
      filters: [{
        name: 'Package',
        extensions: ['yaml']
      }],
      defaultPath: `${currentPackage.value.id}.yaml`
    })

    if (path) {
      await invoke('save_package', {
        package: currentPackage.value,
        path
      })
      hasChanges.value = false
      alert('Package saved successfully!')
    }
  } catch (error) {
    console.error('Failed to save package:', error)
    alert(`Failed to save package: ${error}`)
  }
}

async function validatePackage(pkg) {
  console.log('🔍 Validation triggered for package:', pkg.id)
  try {
    // M9 Phase 3: Validate with dependencies if available
    const result = await invoke('validate_package_with_dependencies', {
      package: pkg,
      dependencies: loadedDependencies.value
    })
    console.log('✅ Validation result:', result)
    validationErrors.value = result.errors || []
    validationWarnings.value = result.warnings || []
  } catch (error) {
    console.error('❌ Validation failed:', error)
    // Show error in validation panel
    validationErrors.value = [{
      message: `Validation error: ${error}`,
      location: null,
      suggestion: null
    }]
    validationWarnings.value = []
  }
}

function onPackageCreate(packageData) {
  currentPackage.value = packageData
  hasChanges.value = true
  showNewPackageDialog.value = false
}

function onAddNamespace(namespaceId) {
  if (!currentPackage.value) return

  // Check if namespace already exists
  if (currentPackage.value.namespaces[namespaceId]) {
    alert(`Namespace "${namespaceId}" already exists`)
    return
  }

  // Create new namespace
  currentPackage.value.namespaces[namespaceId] = {
    id: namespaceId,
    datatypes: {},
    prompt_sections: {},
    separator_sets: {},
    rules: {},
    decisions: [],
    rulebooks: {}
  }

  hasChanges.value = true
  showAddNamespaceDialog.value = false
}

function onComponentSelect(component) {
  selectedComponent.value = component
}

function onDatatypeUpdate(nsId, dtId, updatedData) {
  if (currentPackage.value && currentPackage.value.namespaces[nsId]) {
    currentPackage.value.namespaces[nsId].datatypes[dtId] = updatedData
    hasChanges.value = true
    scheduleValidation() // Explicit validation trigger
  }
}

function onSeparatorUpdate(nsId, sepId, updatedData) {
  if (currentPackage.value && currentPackage.value.namespaces[nsId]) {
    currentPackage.value.namespaces[nsId].separator_sets[sepId] = updatedData
    hasChanges.value = true
    scheduleValidation() // Explicit validation trigger
  }
}

function onPromptSectionUpdate(nsId, psId, updatedData) {
  console.log('📝 PromptSection update received:', { nsId, psId, updatedData })
  if (currentPackage.value && currentPackage.value.namespaces[nsId]) {
    currentPackage.value.namespaces[nsId].prompt_sections[psId] = updatedData
    hasChanges.value = true
    console.log('🔄 Scheduling validation after promptsection update')
    scheduleValidation() // Explicit validation trigger
  }
}

function onRuleUpdate(nsId, ruleId, updatedData) {
  if (currentPackage.value && currentPackage.value.namespaces[nsId]) {
    currentPackage.value.namespaces[nsId].rules[ruleId] = updatedData
    hasChanges.value = true
    scheduleValidation() // Explicit validation trigger
  }
}

function onRulebookUpdate(nsId, rbId, updatedData) {
  console.log('📚 Rulebook update received:', { nsId, rbId, updatedData })
  if (currentPackage.value && currentPackage.value.namespaces[nsId]) {
    // Ensure rulebooks object exists
    if (!currentPackage.value.namespaces[nsId].rulebooks) {
      currentPackage.value.namespaces[nsId].rulebooks = {}
    }
    currentPackage.value.namespaces[nsId].rulebooks[rbId] = updatedData
    hasChanges.value = true
    console.log('🔄 Scheduling validation after rulebook update')
    scheduleValidation() // Explicit validation trigger
  }
}

function createRulebook(nsId) {
  console.log('📚 Creating new rulebook:', { nsId, data: newRulebookData.value })
  if (!currentPackage.value || !currentPackage.value.namespaces[nsId]) return

  // Validate minimum requirements
  if (!newRulebookData.value.name || !newRulebookData.value.name.trim()) {
    alert('Rulebook name is required')
    return
  }

  // Ensure rulebooks object exists
  if (!currentPackage.value.namespaces[nsId].rulebooks) {
    currentPackage.value.namespaces[nsId].rulebooks = {}
  }

  // Generate a unique ID from the name
  let rulebookId = newRulebookData.value.name.trim()
  rulebookId = rulebookId.toLowerCase().replace(/\s+/g, '_').replace(/[^a-z0-9_]/g, '')

  // Ensure unique ID
  let counter = 1
  let finalId = rulebookId
  while (currentPackage.value.namespaces[nsId].rulebooks[finalId]) {
    finalId = `${rulebookId}_${counter}`
    counter++
  }

  // Create the rulebook
  const rulebookData = JSON.parse(JSON.stringify(newRulebookData.value))
  currentPackage.value.namespaces[nsId].rulebooks[finalId] = rulebookData
  hasChanges.value = true

  // Reset the form
  newRulebookData.value = {
    name: '',
    description: '',
    entry_points: [],
    batch_variety: false,
    context_defaults: {}
  }

  // Switch to editing the new rulebook
  selectedComponent.value = {
    id: `rb:${nsId}:${finalId}`,
    data: {
      type: 'rulebook',
      nsId,
      rbId: finalId,
      data: rulebookData
    }
  }

  console.log('✅ Rulebook created:', finalId)
  scheduleValidation()
}

function createDatatype(nsId) {
  console.log('🎲 Creating new datatype:', { nsId, data: newDatatypeData.value })
  if (!currentPackage.value || !currentPackage.value.namespaces[nsId]) return

  // Validate minimum requirements
  if (!newDatatypeData.value.name || !newDatatypeData.value.name.trim()) {
    alert('Datatype name is required')
    return
  }
  if (!newDatatypeData.value.values || newDatatypeData.value.values.length === 0) {
    alert('At least one value is required')
    return
  }

  // Generate a unique ID from the name
  let datatypeId = newDatatypeData.value.name.trim()
  datatypeId = datatypeId.toLowerCase().replace(/\s+/g, '_').replace(/[^a-z0-9_]/g, '')

  if (!datatypeId) {
    datatypeId = 'new_datatype'
  }

  // Ensure unique ID
  let counter = 1
  let finalId = datatypeId
  while (currentPackage.value.namespaces[nsId].datatypes[finalId]) {
    finalId = `${datatypeId}_${counter}`
    counter++
  }

  // Create the datatype (remove name from data - it's the key)
  const datatypeData = JSON.parse(JSON.stringify(newDatatypeData.value))
  delete datatypeData.name // Name is the ID, not part of data
  currentPackage.value.namespaces[nsId].datatypes[finalId] = datatypeData
  hasChanges.value = true

  // Reset the form
  newDatatypeData.value = { name: '', values: [] }

  // Switch to editing the new datatype
  selectedComponent.value = {
    id: `dt:${nsId}:${finalId}`,
    data: {
      type: 'datatype',
      nsId,
      dtId: finalId,
      data: datatypeData
    }
  }

  console.log('✅ Datatype created:', finalId)
  scheduleValidation()
}

function createPromptSection(nsId) {
  console.log('📝 Creating new prompt section:', { nsId, data: newPromptSectionData.value })
  if (!currentPackage.value || !currentPackage.value.namespaces[nsId]) return

  // Validate minimum requirements
  if (!newPromptSectionData.value.name || !newPromptSectionData.value.name.trim()) {
    alert('Prompt section name is required')
    return
  }
  if (!newPromptSectionData.value.template || !newPromptSectionData.value.template.trim()) {
    alert('Template is required')
    return
  }

  // Generate a unique ID from the name
  let promptSectionId = newPromptSectionData.value.name.trim()
  promptSectionId = promptSectionId.toLowerCase().replace(/\s+/g, '_').replace(/[^a-z0-9_]/g, '')

  if (!promptSectionId) {
    promptSectionId = 'new_prompt_section'
  }

  // Ensure unique ID
  let counter = 1
  let finalId = promptSectionId
  while (currentPackage.value.namespaces[nsId].prompt_sections[finalId]) {
    finalId = `${promptSectionId}_${counter}`
    counter++
  }

  // Create the prompt section (remove name from data - it's the key)
  const promptSectionData = JSON.parse(JSON.stringify(newPromptSectionData.value))
  delete promptSectionData.name // Name is the ID, not part of data
  currentPackage.value.namespaces[nsId].prompt_sections[finalId] = promptSectionData
  hasChanges.value = true

  // Reset the form
  newPromptSectionData.value = { name: '', template: '', references: {} }

  // Switch to editing the new prompt section
  selectedComponent.value = {
    id: `ps:${nsId}:${finalId}`,
    data: {
      type: 'promptsection',
      nsId,
      psId: finalId,
      data: promptSectionData
    }
  }

  console.log('✅ Prompt section created:', finalId)
  scheduleValidation()
}

function createSeparatorSet(nsId) {
  console.log('➗ Creating new separator set:', { nsId, data: newSeparatorSetData.value })
  if (!currentPackage.value || !currentPackage.value.namespaces[nsId]) return

  // Validate minimum requirements
  if (!newSeparatorSetData.value.name || !newSeparatorSetData.value.name.trim()) {
    alert('Separator set name is required')
    return
  }
  if (!newSeparatorSetData.value.primary) {
    alert('Primary separator is required')
    return
  }

  // Generate a unique ID from the name
  let separatorId = newSeparatorSetData.value.name.trim()
  separatorId = separatorId.toLowerCase().replace(/\s+/g, '_').replace(/[^a-z0-9_]/g, '')

  if (!separatorId) {
    separatorId = 'separator_set'
  }

  // Ensure unique ID
  let counter = 1
  let finalId = separatorId
  while (currentPackage.value.namespaces[nsId].separator_sets[finalId]) {
    finalId = `${separatorId}_${counter}`
    counter++
  }

  // Create the separator set (remove name from data - it's the key)
  const separatorSetData = JSON.parse(JSON.stringify(newSeparatorSetData.value))
  delete separatorSetData.name // Name is the ID, not part of data
  currentPackage.value.namespaces[nsId].separator_sets[finalId] = separatorSetData
  hasChanges.value = true

  // Reset the form
  newSeparatorSetData.value = { name: '', primary: '', secondary: '', tertiary: '' }

  // Switch to editing the new separator set
  selectedComponent.value = {
    id: `sep:${nsId}:${finalId}`,
    data: {
      type: 'separator',
      nsId,
      sepId: finalId,
      data: separatorSetData
    }
  }

  console.log('✅ Separator set created:', finalId)
  scheduleValidation()
}

function createRule(nsId) {
  console.log('⚙️ Creating new rule:', { nsId, data: newRuleData.value })
  if (!currentPackage.value || !currentPackage.value.namespaces[nsId]) return

  // Validate minimum requirements
  if (!newRuleData.value.name || !newRuleData.value.name.trim()) {
    alert('Rule name is required')
    return
  }
  if (!newRuleData.value.when || !newRuleData.value.set) {
    alert('When and Set fields are required')
    return
  }

  // Ensure rules object exists
  if (!currentPackage.value.namespaces[nsId].rules) {
    currentPackage.value.namespaces[nsId].rules = {}
  }

  // Generate a unique ID from the name
  let ruleId = newRuleData.value.name.trim()
  ruleId = ruleId.toLowerCase().replace(/\s+/g, '_').replace(/[^a-z0-9_]/g, '')

  if (!ruleId) {
    ruleId = 'new_rule'
  }

  // Ensure unique ID
  let counter = 1
  let finalId = ruleId
  while (currentPackage.value.namespaces[nsId].rules[finalId]) {
    finalId = `${ruleId}_${counter}`
    counter++
  }

  // Create the rule (remove name from data - it's the key)
  const ruleData = JSON.parse(JSON.stringify(newRuleData.value))
  delete ruleData.name // Name is the ID, not part of data
  currentPackage.value.namespaces[nsId].rules[finalId] = ruleData
  hasChanges.value = true

  // Reset the form
  newRuleData.value = { name: '', when: '', logic: '', set: '', value: '' }

  // Switch to editing the new rule
  selectedComponent.value = {
    id: `rule:${nsId}:${finalId}`,
    data: {
      type: 'rule',
      nsId,
      ruleId: finalId,
      data: ruleData
    }
  }

  console.log('✅ Rule created:', finalId)
  scheduleValidation()
}

function cancelNewComponent() {
  // Reset all forms
  newRulebookData.value = {
    name: '',
    description: '',
    entry_points: [],
    batch_variety: false,
    context_defaults: {}
  }
  newDatatypeData.value = { name: '', values: [] }
  newPromptSectionData.value = { name: '', template: '', references: {} }
  newSeparatorSetData.value = { name: '', primary: '', secondary: '', tertiary: '' }
  newRuleData.value = { name: '', when: '', logic: '', set: '', value: '' }

  // Close the editor
  selectedComponent.value = null
}

function onPackageMetadataUpdate(updatedPackage) {
  // Update the entire package (metadata changes)
  currentPackage.value = updatedPackage
  hasChanges.value = true
  scheduleValidation() // Explicit validation trigger
}

function jumpToError(error) {
  if (!error.location || !currentPackage.value) return

  const location = error.location
  console.log('Jumping to error location:', location)

  // Parse location string
  // Format: "namespace:component_name" or just "namespace"

  // Try to parse as namespace:component
  if (location.includes(':')) {
    const [nsId, componentName] = location.split(':')

    if (!currentPackage.value.namespaces[nsId]) {
      console.warn('Namespace not found:', nsId)
      return
    }

    const namespace = currentPackage.value.namespaces[nsId]

    // Check datatypes
    if (namespace.datatypes && namespace.datatypes[componentName]) {
      selectedComponent.value = {
        id: `dt:${nsId}:${componentName}`,
        data: {
          type: 'datatype',
          nsId,
          dtId: componentName,
          data: namespace.datatypes[componentName]
        }
      }
      return
    }

    // Check promptsections
    if (namespace.prompt_sections && namespace.prompt_sections[componentName]) {
      selectedComponent.value = {
        id: `ps:${nsId}:${componentName}`,
        data: {
          type: 'promptsection',
          nsId,
          psId: componentName,
          data: namespace.prompt_sections[componentName]
        }
      }
      return
    }

    // Check separator sets
    if (namespace.separator_sets && namespace.separator_sets[componentName]) {
      selectedComponent.value = {
        id: `sep:${nsId}:${componentName}`,
        data: {
          type: 'separator',
          nsId,
          sepId: componentName,
          data: namespace.separator_sets[componentName]
        }
      }
      return
    }

    console.warn('Component not found:', componentName, 'in namespace:', nsId)
    return
  }

  // Check if it's just a namespace name
  if (currentPackage.value.namespaces[location]) {
    selectedComponent.value = {
      id: `ns:${location}`,
      data: {
        type: 'namespace',
        nsId: location,
        data: currentPackage.value.namespaces[location]
      }
    }
    return
  }

  console.warn('Could not find component for location:', location)
}

/**
 * Handle package installation from marketplace
 */
async function handlePackageInstall(pkg, version) {
  try {
    // Download package YAML content
    const yamlContent = await marketplaceClient.downloadPackage(
      pkg.namespace,
      pkg.name,
      version
    )

    // Determine save location (app data directory)
    const appDir = await appDataDir()
    const packagesDir = await join(appDir, 'packages')

    // Create packages directory if it doesn't exist
    try {
      await mkdir(packagesDir, { recursive: true })
    } catch (error) {
      // Directory might already exist, ignore error
      console.log('Packages directory exists or created')
    }

    const fileName = `${pkg.namespace}.${pkg.name}.yaml`
    const filePath = await join(packagesDir, fileName)

    // Save package file
    await writeTextFile(filePath, yamlContent)

    // Show success message
    alert(`Package ${pkg.namespace}/${pkg.name} v${version} installed successfully!\nLocation: ${filePath}`)

    // Optionally load the package immediately
    const shouldLoad = confirm('Would you like to open the installed package now?')
    if (shouldLoad) {
      await loadPackageFromPath(filePath)
    }

    // Close marketplace modal
    showMarketplaceSettings.value = false
  } catch (error) {
    console.error('Package installation failed:', error)
    alert(`Failed to install package: ${error.message}`)
  }
}

/**
 * Load package from a specific path
 */
async function loadPackageFromPath(filePath) {
  try {
    const result = await invoke('load_package_with_dependencies', { path: filePath })
    currentPackage.value = result.package
    loadedDependencies.value = result.dependencies || {}
    hasChanges.value = false
    selectedComponent.value = null

    // Validate after loading
    await validatePackage(currentPackage.value)
  } catch (error) {
    console.error('Failed to load package:', error)
    alert(`Failed to load package: ${error.message}`)
  }
}

// Check if we should load a package from query params (e.g., after marketplace install)
const route = useRoute()
const router = useRouter()
onMounted(async () => {
  // Handle loading from library via query param
  const loadLibraryPackageParam = route.query.loadLibraryPackage as string
  if (loadLibraryPackageParam) {
    // Parse package@version format
    const [packageId, version] = loadLibraryPackageParam.split('@')
    if (packageId && version) {
      try {
        const { loadPackageFromLibrary } = await import('../services/package-library.service')
        const pkg = await loadPackageFromLibrary(packageId, version)
        currentPackage.value = pkg
        hasChanges.value = false
        // Clear the query param
        router.replace({ path: '/', query: {} })
      } catch (error) {
        console.error('Failed to load package from library:', error)
        alert(`Failed to load package from library: ${(error as Error).message}`)
      }
      return
    }
  }

  // Handle loading from file path via query param (legacy)
  const loadPackagePath = route.query.loadPackage as string
  if (loadPackagePath) {
    await loadPackageFromPath(loadPackagePath)
  }
})
</script>

<style scoped>
.package-editor {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 20px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.header-left {
  display: flex;
  align-items: baseline;
  gap: 12px;
}

.header-left h1 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.package-version {
  font-size: 14px;
  color: var(--text-muted);
}

.app-version {
  font-size: 14px;
  color: var(--text-muted);
  font-style: italic;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.editor-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 280px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  overflow-y: auto;
}

.main-panel {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  background: var(--bg-primary);
}

.preview-panel {
  width: 400px;
  background: var(--bg-primary);
  border-left: 1px solid var(--border-color);
  overflow-y: auto;
}

.welcome-message {
  text-align: center;
  padding: 60px 20px;
}

.welcome-message h2 {
  margin-bottom: 16px;
  font-size: 24px;
  color: var(--text-primary);
}

.welcome-message p {
  margin-bottom: 32px;
  color: var(--text-muted);
}

.placeholder {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-muted);
}

.btn-primary,
.btn-secondary {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--accent-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--button-secondary-bg);
  color: var(--text-primary);
}

.btn-secondary:hover {
  background: var(--button-secondary-hover);
}

.btn-large {
  padding: 12px 24px;
  font-size: 16px;
}

/* New Component Creator */
.new-component-creator {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.creation-actions {
  display: flex;
  gap: 12px;
  padding: 16px 20px;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.btn-create {
  padding: 10px 20px;
  background: var(--accent-color);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  font-size: 14px;
}

.btn-create:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-create:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-cancel {
  padding: 10px 20px;
  background: var(--button-secondary-bg);
  color: var(--text-primary);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  font-size: 14px;
}

.btn-cancel:hover {
  background: var(--button-secondary-hover);
}

/* Modal Overlay */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-primary);
  border-radius: 8px;
  max-width: 600px;
  width: 90%;
  max-height: 80vh;
  overflow: auto;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
}

.modal-large {
  max-width: 1200px;
  width: 95%;
  max-height: 90vh;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  color: var(--text-secondary);
  font-size: 18px;
}

.btn-close {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 28px;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.btn-close:hover {
  background: var(--button-secondary-bg);
}

</style>



