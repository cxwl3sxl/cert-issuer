<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { certificateApi, type Certificate, type HealthStatus } from '../services/api'
import { RouterLink } from 'vue-router'

const health = ref<HealthStatus | null>(null)
const certificates = ref<Certificate[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

const stats = ref({
  total: 0,
  active: 0,
  expiring: 0
})

const fetchData = async () => {
  loading.value = true
  error.value = null

  try {
    const [healthData, certData] = await Promise.all([
      certificateApi.getHealth(),
      certificateApi.getCertificates()
    ])

    health.value = healthData
    certificates.value = certData

    const now = new Date()
    const thirtyDaysLater = new Date(now.getTime() + 30 * 24 * 60 * 60 * 1000)

    stats.value.total = certData.length
    stats.value.active = certData.filter(cert => {
      const notAfter = new Date(cert.not_after)
      return notAfter > now
    }).length
    stats.value.expiring = certData.filter(cert => {
      const notAfter = new Date(cert.not_after)
      return notAfter > now && notAfter <= thirtyDaysLater
    }).length
  } catch (e: any) {
    error.value = e.message || '获取数据失败'
  } finally {
    loading.value = false
  }
}

onMounted(fetchData)
</script>

<template>
  <div class="dashboard">
    <div class="page-header">
      <h1 class="page-title">仪表盘</h1>
      <p class="page-subtitle">系统状态概览与证书管理</p>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>

    <div v-else-if="error" class="error-message">
      {{ error }}
    </div>

    <template v-else>
      <!-- 健康状态卡片 -->
      <div class="health-section">
        <div class="status-card" :class="{ healthy: health?.status === 'healthy' }">
          <div class="status-indicator">
            <span class="status-dot"></span>
            <span class="status-label">服务状态</span>
          </div>
          <div class="status-value">
            {{ health?.status === 'healthy' ? '运行中' : '异常' }}
          </div>
          <div class="status-time" v-if="health?.timestamp">
            最后更新: {{ new Date(health.timestamp).toLocaleString() }}
          </div>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">◈</div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.total }}</div>
            <div class="stat-label">证书总数</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon active">✓</div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.active }}</div>
            <div class="stat-label">有效证书</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon warning">!</div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.expiring }}</div>
            <div class="stat-label">即将过期</div>
          </div>
        </div>
      </div>

      <!-- 快捷操作 -->
      <div class="quick-actions">
        <h2 class="section-title">快捷操作</h2>
        <div class="action-buttons">
          <RouterLink to="/certificates/issue" class="btn btn-primary">
            <span>✦</span>
            颁发新证书
          </RouterLink>
          <RouterLink to="/certificates" class="btn btn-secondary">
            <span>◈</span>
            查看所有证书
          </RouterLink>
        </div>
      </div>

      <!-- 最近证书 -->
      <div class="recent-section" v-if="certificates.length > 0">
        <h2 class="section-title">最近签发的证书</h2>
        <div class="recent-list">
          <RouterLink
            v-for="cert in certificates.slice(0, 5)"
            :key="cert.id"
            :to="`/certificates/${cert.id}`"
            class="recent-item"
          >
            <div class="recent-info">
              <div class="recent-cn">{{ cert.subject.cn }}</div>
              <div class="recent-meta">
                <span class="meta-item">{{ cert.subject.o || '个人' }}</span>
                <span class="meta-sep">•</span>
                <span class="meta-item">{{ new Date(cert.not_before).toLocaleDateString() }}</span>
              </div>
            </div>
            <div class="recent-arrow">→</div>
          </RouterLink>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.dashboard {
  animation: fadeIn 0.4s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.page-header {
  margin-bottom: 2rem;
}

.page-title {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
  letter-spacing: -0.02em;
}

.page-subtitle {
  color: var(--text-muted);
  font-size: 0.9rem;
}

/* 健康状态 */
.health-section {
  margin-bottom: 2rem;
}

.status-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.25rem 1.5rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  border-left: 3px solid var(--accent-danger);
}

.status-card.healthy {
  border-left-color: var(--accent-success);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--accent-danger);
  box-shadow: 0 0 8px var(--accent-danger);
}

.status-card.healthy .status-dot {
  background: var(--accent-success);
  box-shadow: 0 0 8px var(--accent-success);
}

.status-label {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.status-value {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  flex: 1;
}

.status-card.healthy .status-value {
  color: var(--accent-success);
}

.status-time {
  font-size: 0.75rem;
  color: var(--text-muted);
}

/* 统计卡片 */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  transition: all 0.2s ease;
}

.stat-card:hover {
  border-color: var(--border-default);
  transform: translateY(-2px);
}

.stat-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  color: var(--accent-primary);
}

.stat-icon.active {
  color: var(--accent-success);
}

.stat-icon.warning {
  color: var(--accent-warning);
}

.stat-value {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  font-size: 0.8rem;
  color: var(--text-muted);
  margin-top: 0.25rem;
}

/* 快捷操作 */
.quick-actions {
  margin-bottom: 2rem;
}

.section-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 1rem;
}

.action-buttons {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

/* 最近证书 */
.recent-section {
  margin-bottom: 2rem;
}

.recent-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.recent-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.25rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  text-decoration: none;
  color: inherit;
  transition: all 0.2s ease;
}

.recent-item:hover {
  border-color: var(--border-default);
  background: var(--bg-hover);
}

.recent-info {
  flex: 1;
}

.recent-cn {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.recent-meta {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.8rem;
  color: var(--text-muted);
}

.meta-sep {
  opacity: 0.5;
}

.recent-arrow {
  color: var(--text-muted);
  font-size: 1.25rem;
  transition: transform 0.2s ease;
}

.recent-item:hover .recent-arrow {
  transform: translateX(4px);
  color: var(--accent-primary);
}
</style>