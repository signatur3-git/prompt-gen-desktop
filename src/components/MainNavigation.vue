<template>
  <nav class="main-navigation">
    <!-- Left: Logo/Brand -->
    <div class="nav-brand">
      <router-link to="/" class="brand-link">
        <span class="brand-icon">🎲</span>
        <span class="brand-name">Prompt Gen</span>
      </router-link>
    </div>

    <!-- Center: Main Navigation Links -->
    <div class="nav-links">
      <router-link to="/generate" class="nav-link" active-class="active">
        <span class="nav-icon">⚡</span>
        <span class="nav-label">Generate</span>
      </router-link>

      <router-link to="/edit" class="nav-link" active-class="active">
        <span class="nav-icon">✏️</span>
        <span class="nav-label">Edit</span>
      </router-link>

      <router-link to="/library" class="nav-link" active-class="active">
        <span class="nav-icon">📚</span>
        <span class="nav-label">Library</span>
      </router-link>

      <router-link to="/marketplace" class="nav-link" active-class="active">
        <span class="nav-icon">🏪</span>
        <span class="nav-label">Marketplace</span>
      </router-link>
    </div>

    <!-- Right: Status/Actions -->
    <div class="nav-status">
      <slot name="status"></slot>
    </div>
  </nav>
</template>

<script setup lang="ts">
// Main navigation component with 3-section layout:
// - Left: Logo/brand (takes user home)
// - Center: Main navigation links (Generate, Edit, Library, Marketplace)
// - Right: Marketplace connection status and other global actions
</script>

<style scoped>
.main-navigation {
  display: grid;
  grid-template-columns: 200px 1fr 200px;
  align-items: center;
  height: 56px; /* Fixed height to prevent jumping */
  padding: 0 24px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  gap: 24px;
}

/* Left: Brand/Logo */
.nav-brand {
  display: flex;
  align-items: center;
}

.brand-link {
  display: flex;
  align-items: center;
  gap: 10px;
  text-decoration: none;
  color: var(--text-primary);
  padding: 6px 12px;
  border-radius: 6px;
  transition: background-color 0.2s;
}

.brand-link:hover {
  background: var(--bg-tertiary);
}

.brand-icon {
  font-size: 24px;
  line-height: 1;
}

.brand-name {
  font-size: 16px;
  font-weight: 700;
  letter-spacing: -0.02em;
}

/* Center: Navigation Links */
.nav-links {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 4px;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border-radius: 6px;
  text-decoration: none;
  color: var(--text-primary);
  transition: background-color 0.2s, border-color 0.2s;
  white-space: nowrap;
  border: 1px solid transparent;
}

.nav-link:hover {
  background: var(--bg-tertiary);
}

.nav-link.active {
  background: var(--bg-primary);
  font-weight: 600;
  border: 1px solid var(--border-color);
}

.nav-icon {
  font-size: 18px;
  line-height: 1;
}

.nav-label {
  font-size: 14px;
  font-weight: 500;
}

/* Right: Status/Actions */
.nav-status {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
  min-height: 40px; /* Prevent height changes */
}
</style>

