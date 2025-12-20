<template>
  <div class="component-tree">
    <div class="tree-header">
      <h3>Package Structure</h3>
      <button @click="$emit('add-namespace')" class="btn-add" title="Add Namespace">
        +
      </button>
    </div>

    <div class="tree-content">
      <!-- Package Info -->
      <div
        class="tree-item package-item"
        :class="{ selected: selectedId === 'package' }"
        @click="selectComponent('package', pkg)"
      >
        <span class="item-icon">📦</span>
        <span class="item-label">{{ pkg.metadata.name || pkg.id }}</span>
      </div>

      <!-- Namespaces -->
      <div v-for="(namespace, nsId) in pkg.namespaces" :key="nsId" class="namespace-section">
        <div
          class="tree-item namespace-item"
          :class="{ selected: selectedId === `ns:${nsId}` }"
          @click="selectComponent(`ns:${nsId}`, namespace)"
        >
          <span class="expand-icon" @click.stop="toggleExpand(nsId)">
            {{ expanded[nsId] ? '▼' : '▶' }}
          </span>
          <span class="item-icon">📁</span>
          <span class="item-label">{{ nsId }}</span>
        </div>

        <div v-if="expanded[nsId]" class="tree-children">
          <!-- Datatypes -->
          <div class="tree-section">
            <div class="section-header">
              <span class="section-label">Datatypes ({{ Object.keys(namespace.datatypes).length }})</span>
              <button @click="addComponent(nsId, 'datatype')" class="btn-add-small">+</button>
            </div>
            <div
              v-for="(datatype, dtId) in namespace.datatypes"
              :key="`dt:${nsId}:${dtId}`"
              class="tree-item datatype-item"
              :class="{ selected: selectedId === `dt:${nsId}:${dtId}` }"
              @click="selectComponent(`dt:${nsId}:${dtId}`, { type: 'datatype', nsId, dtId, data: datatype })"
            >
              <span class="item-icon">🎲</span>
              <span class="item-label">{{ dtId }}</span>
              <span class="item-count">({{ datatype.values?.length || 0 }})</span>
            </div>
          </div>

          <!-- PromptSections -->
          <div class="tree-section">
            <div class="section-header">
              <span class="section-label">Prompt Sections ({{ Object.keys(namespace.prompt_sections).length }})</span>
              <button @click="addComponent(nsId, 'promptsection')" class="btn-add-small">+</button>
            </div>
            <div
              v-for="(ps, psId) in namespace.prompt_sections"
              :key="`ps:${nsId}:${psId}`"
              class="tree-item promptsection-item"
              :class="{ selected: selectedId === `ps:${nsId}:${psId}` }"
              @click="selectComponent(`ps:${nsId}:${psId}`, { type: 'promptsection', nsId, psId, data: ps })"
            >
              <span class="item-icon">📝</span>
              <span class="item-label">{{ psId }}</span>
            </div>
          </div>

          <!-- Separator Sets -->
          <div class="tree-section">
            <div class="section-header">
              <span class="section-label">Separators ({{ Object.keys(namespace.separator_sets).length }})</span>
              <button @click="addComponent(nsId, 'separator')" class="btn-add-small">+</button>
            </div>
            <div
              v-for="(sep, sepId) in namespace.separator_sets"
              :key="`sep:${nsId}:${sepId}`"
              class="tree-item separator-item"
              :class="{ selected: selectedId === `sep:${nsId}:${sepId}` }"
              @click="selectComponent(`sep:${nsId}:${sepId}`, { type: 'separator', nsId, sepId, data: sep })"
            >
              <span class="item-icon">➗</span>
              <span class="item-label">{{ sepId }}</span>
            </div>
          </div>

          <!-- Rules -->
          <div class="tree-section">
            <div class="section-header">
              <span class="section-label">Rules ({{ Object.keys(namespace.rules || {}).length }})</span>
              <button @click="addComponent(nsId, 'rule')" class="btn-add-small">+</button>
            </div>
            <div
              v-for="(rule, ruleId) in namespace.rules"
              :key="`rule:${nsId}:${ruleId}`"
              class="tree-item rule-item"
              :class="{ selected: selectedId === `rule:${nsId}:${ruleId}` }"
              @click="selectComponent(`rule:${nsId}:${ruleId}`, { type: 'rule', nsId, ruleId, data: rule })"
            >
              <span class="item-icon">⚙️</span>
              <span class="item-label">{{ ruleId }}</span>
            </div>
          </div>

          <!-- Rulebooks -->
          <div class="tree-section">
            <div class="section-header">
              <span class="section-label">Rulebooks ({{ Object.keys(namespace.rulebooks || {}).length }})</span>
              <button @click="addComponent(nsId, 'rulebook')" class="btn-add-small">+</button>
            </div>
            <div
              v-for="(rulebook, rbId) in namespace.rulebooks"
              :key="`rb:${nsId}:${rbId}`"
              class="tree-item rulebook-item"
              :class="{ selected: selectedId === `rb:${nsId}:${rbId}` }"
              @click="selectComponent(`rb:${nsId}:${rbId}`, { type: 'rulebook', nsId, rbId, data: rulebook })"
            >
              <span class="item-icon">📚</span>
              <span class="item-label">{{ rulebook.name || rbId }}</span>
              <span class="item-count">({{ rulebook.entry_points?.length || 0 }})</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, toRef } from 'vue'

const props = defineProps({
  package: {
    type: Object,
    required: true
  }
})

const emit = defineEmits(['select', 'add-namespace'])

const pkg = toRef(props, 'package')
const expanded = ref({})
const selectedId = ref(null)

// Watch for package changes and reset expanded state
watch(() => props.package, (newPackage) => {
  if (newPackage) {
    // Reset expanded state for new package
    expanded.value = {}
    // Expand all namespaces by default
    Object.keys(newPackage.namespaces).forEach(nsId => {
      expanded.value[nsId] = true
    })
    // Clear selection when package changes
    selectedId.value = null
  }
}, { immediate: true })

function toggleExpand(nsId) {
  expanded.value[nsId] = !expanded.value[nsId]
}

function selectComponent(id, data) {
  selectedId.value = id
  emit('select', { id, data })
}

function addComponent(nsId, type) {
  emit('select', {
    id: `new:${nsId}:${type}`,
    data: { type: 'new', componentType: type, nsId }
  })
}
</script>

<style scoped>
.component-tree {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.tree-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #3e3e42;
}

.tree-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  text-transform: uppercase;
  color: #858585;
}

.btn-add {
  background: #0e639c;
  color: white;
  border: none;
  border-radius: 4px;
  width: 24px;
  height: 24px;
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
}

.btn-add:hover {
  background: #1177bb;
}

.tree-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.tree-item {
  display: flex;
  align-items: center;
  padding: 6px 16px;
  cursor: pointer;
  user-select: none;
  transition: background 0.2s;
}

.tree-item:hover {
  background: #2a2d2e;
}

.tree-item.selected {
  background: #094771;
}

.expand-icon {
  width: 16px;
  font-size: 10px;
  color: #858585;
  margin-right: 4px;
}

.item-icon {
  margin-right: 8px;
  font-size: 14px;
}

.item-label {
  flex: 1;
  font-size: 13px;
}

.item-count {
  font-size: 11px;
  color: #858585;
  margin-left: 4px;
}

.package-item {
  font-weight: 600;
  margin-bottom: 4px;
}

.namespace-item {
  font-weight: 500;
  margin-top: 8px;
}

.tree-children {
  margin-left: 20px;
}

.tree-section {
  margin-top: 8px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 4px 16px;
  color: #858585;
  font-size: 11px;
  text-transform: uppercase;
  font-weight: 600;
}

.btn-add-small {
  background: none;
  border: none;
  color: #858585;
  cursor: pointer;
  font-size: 14px;
  padding: 0 4px;
  border-radius: 2px;
}

.btn-add-small:hover {
  background: #3e3e42;
  color: #d4d4d4;
}

.datatype-item .item-icon { color: #4fc1ff; }
.promptsection-item .item-icon { color: #89d185; }
.separator-item .item-icon { color: #dcdcaa; }
.rule-item .item-icon { color: #c586c0; }
</style>

