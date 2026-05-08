<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, RouterLink, RouterView } from 'vue-router'

const route = useRoute()
const isMenuOpen = ref(false)

const navItems = [
  { path: '/', label: '仪表盘', icon: '◇' },
  { path: '/certificates', label: '证书列表', icon: '◈' },
  { path: '/certificates/issue', label: '颁发证书', icon: '✦' }
]

const currentPath = computed(() => route.path)
</script>

<template>
  <div class="app-container">
    <header class="app-header">
      <div class="header-content">
        <div class="logo">
          <span class="logo-icon">❖</span>
          <span class="logo-text">证书签发服务</span>
        </div>
        <nav class="main-nav" :class="{ open: isMenuOpen }">
          <RouterLink
            v-for="item in navItems"
            :key="item.path"
            :to="item.path"
            class="nav-link"
            :class="{ active: currentPath === item.path }"
          >
            <span class="nav-icon">{{ item.icon }}</span>
            {{ item.label }}
          </RouterLink>
        </nav>
        <button class="menu-toggle" @click="isMenuOpen = !isMenuOpen">
          <span></span>
          <span></span>
          <span></span>
        </button>
      </div>
    </header>

    <main class="app-main">
      <RouterView />
    </main>

    <footer class="app-footer">
      <p>Certificate Issuer Service &copy; 2026</p>
    </footer>
  </div>
</template>

<style scoped>
.app-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
}

.app-header {
  background: var(--bg-header);
  border-bottom: 1px solid var(--border-subtle);
  position: sticky;
  top: 0;
  z-index: 100;
  backdrop-filter: blur(12px);
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 1.5rem;
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  font-weight: 600;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.logo-icon {
  font-size: 1.5rem;
  color: var(--accent-primary);
  filter: drop-shadow(0 0 8px var(--accent-glow));
}

.main-nav {
  display: flex;
  gap: 0.25rem;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 0.9rem;
  font-weight: 500;
  transition: all 0.2s ease;
}

.nav-icon {
  font-size: 1rem;
  opacity: 0.7;
}

.nav-link:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.nav-link.active {
  color: var(--accent-primary);
  background: var(--bg-active);
}

.nav-link.active .nav-icon {
  opacity: 1;
}

.menu-toggle {
  display: none;
  flex-direction: column;
  gap: 4px;
  padding: 8px;
  background: none;
  border: none;
  cursor: pointer;
}

.menu-toggle span {
  width: 20px;
  height: 2px;
  background: var(--text-secondary);
  border-radius: 2px;
  transition: all 0.2s;
}

.app-main {
  flex: 1;
  max-width: 1400px;
  width: 100%;
  margin: 0 auto;
  padding: 2rem 1.5rem;
}

.app-footer {
  text-align: center;
  padding: 1.5rem;
  color: var(--text-muted);
  font-size: 0.85rem;
  border-top: 1px solid var(--border-subtle);
}

@media (max-width: 768px) {
  .menu-toggle {
    display: flex;
  }

  .main-nav {
    position: absolute;
    top: 64px;
    left: 0;
    right: 0;
    flex-direction: column;
    background: var(--bg-header);
    padding: 1rem;
    border-bottom: 1px solid var(--border-subtle);
    transform: translateY(-100%);
    opacity: 0;
    pointer-events: none;
    transition: all 0.3s ease;
  }

  .main-nav.open {
    transform: translateY(0);
    opacity: 1;
    pointer-events: auto;
  }
}
</style>