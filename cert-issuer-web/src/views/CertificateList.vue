<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { certificateApi, type Certificate } from '../services/api'
import { RouterLink } from 'vue-router'

const certificates = ref<Certificate[]>([])
const loading = ref(true)
const error = ref<string | null>(null)
const searchQuery = ref('')
const sortBy = ref<'not_before' | 'not_after' | 'cn'>('not_before')
const sortOrder = ref<'asc' | 'desc'>('desc')

// 删除确认相关
const deleteDialogVisible = ref(false)
const deletingCertId = ref<string | null>(null)
const deletingCertName = ref<string>('')
const deletingLoading = ref(false)

const filteredCertificates = computed(() => {
  let result = [...certificates.value]

  // 搜索过滤
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(cert =>
      cert.subject.cn.toLowerCase().includes(query) ||
      (cert.subject.o && cert.subject.o.toLowerCase().includes(query)) ||
      cert.id.toLowerCase().includes(query)
    )
  }

  // 排序
  result.sort((a, b) => {
    let aVal: string, bVal: string

    if (sortBy.value === 'cn') {
      aVal = a.subject.cn.toLowerCase()
      bVal = b.subject.cn.toLowerCase()
    } else if (sortBy.value === 'not_before') {
      aVal = a.not_before
      bVal = b.not_before
    } else {
      aVal = a.not_after
      bVal = b.not_after
    }

    if (sortOrder.value === 'asc') {
      return aVal.localeCompare(bVal)
    }
    return bVal.localeCompare(aVal)
  })

  return result
})

const fetchCertificates = async () => {
  loading.value = true
  error.value = null

  try {
    certificates.value = await certificateApi.getCertificates()
  } catch (e: any) {
    error.value = e.message || '获取证书列表失败'
  } finally {
    loading.value = false
  }
}

const toggleSort = (column: 'not_before' | 'not_after' | 'cn') => {
  if (sortBy.value === column) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortBy.value = column
    sortOrder.value = 'desc'
  }
}

const showDeleteDialog = (cert: Certificate) => {
  deletingCertId.value = cert.id
  deletingCertName.value = cert.subject.cn
  deleteDialogVisible.value = true
}

const hideDeleteDialog = () => {
  deleteDialogVisible.value = false
  deletingCertId.value = null
  deletingCertName.value = ''
}

const confirmDelete = async () => {
  if (!deletingCertId.value) return
  
  deletingLoading.value = true
  try {
    await certificateApi.deleteCertificate(deletingCertId.value)
    // 从列表中移除
    certificates.value = certificates.value.filter(c => c.id !== deletingCertId.value)
    hideDeleteDialog()
  } catch (e: any) {
    error.value = e.message || '删除证书失败'
  } finally {
    deletingLoading.value = false
  }
}

const getStatusTag = (cert: Certificate) => {
  const now = new Date()
  const notAfter = new Date(cert.not_after)

  if (notAfter < now) {
    return { text: '已过期', class: 'tag-danger' }
  }

  const thirtyDaysLater = new Date(now.getTime() + 30 * 24 * 60 * 60 * 1000)
  if (notAfter <= thirtyDaysLater) {
    return { text: '即将过期', class: 'tag-warning' }
  }

  return { text: '有效', class: 'tag-success' }
}

onMounted(fetchCertificates)
</script>

<template>
  <div class="certificate-list">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">证书列表</h1>
        <p class="page-subtitle">管理所有已签发的数字证书</p>
      </div>
      <RouterLink to="/certificates/issue" class="btn btn-primary">
        <span>✦</span>
        颁发新证书
      </RouterLink>
    </div>

    <!-- 搜索和筛选 -->
    <div class="toolbar">
      <div class="search-box">
        <span class="search-icon">⌕</span>
        <input
          v-model="searchQuery"
          type="text"
          class="search-input"
          placeholder="搜索证书 (CN, 组织, ID)..."
        />
      </div>
      <div class="sort-controls">
        <button
          class="sort-btn"
          :class="{ active: sortBy === 'not_before' }"
          @click="toggleSort('not_before')"
        >
          签发日期
          <span v-if="sortBy === 'not_before'" class="sort-indicator">
            {{ sortOrder === 'asc' ? '↑' : '↓' }}
          </span>
        </button>
        <button
          class="sort-btn"
          :class="{ active: sortBy === 'not_after' }"
          @click="toggleSort('not_after')"
        >
          到期日期
          <span v-if="sortBy === 'not_after'" class="sort-indicator">
            {{ sortOrder === 'asc' ? '↑' : '↓' }}
          </span>
        </button>
        <button
          class="sort-btn"
          :class="{ active: sortBy === 'cn' }"
          @click="toggleSort('cn')"
        >
          名称
          <span v-if="sortBy === 'cn'" class="sort-indicator">
            {{ sortOrder === 'asc' ? '↑' : '↓' }}
          </span>
        </button>
      </div>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>

    <div v-else-if="error" class="error-message">
      {{ error }}
    </div>

    <template v-else>
      <div v-if="certificates.length === 0" class="empty-state">
        <div class="empty-icon">◈</div>
        <h3>暂无证书</h3>
        <p>开始签发您的第一个数字证书</p>
        <RouterLink to="/certificates/issue" class="btn btn-primary">
          颁发证书
        </RouterLink>
      </div>

      <div v-else class="table-container">
        <table class="table">
          <thead>
            <tr>
              <th>证书名称</th>
              <th>组织</th>
              <th>签发日期</th>
              <th>到期日期</th>
              <th>状态</th>
              <th>操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="cert in filteredCertificates" :key="cert.id">
              <td>
                <div class="cert-cn">{{ cert.subject.cn }}</div>
                <div class="cert-id">{{ cert.id.substring(0, 12) }}...</div>
              </td>
              <td>{{ cert.subject.o || '-' }}</td>
              <td>{{ new Date(cert.not_before).toLocaleDateString() }}</td>
              <td>{{ new Date(cert.not_after).toLocaleDateString() }}</td>
              <td>
                <span class="tag" :class="getStatusTag(cert).class">
                  {{ getStatusTag(cert).text }}
                </span>
              </td>
               <td>
                 <div class="action-buttons">
                   <RouterLink :to="`/certificates/${cert.id}`" class="btn btn-ghost btn-sm">
                     查看详情
                   </RouterLink>
                   <button
                     class="btn btn-danger btn-sm"
                     @click="showDeleteDialog(cert)"
                   >
                     删除
                   </button>
                 </div>
               </td>
            </tr>
          </tbody>
        </table>
      </div>

       <div v-if="filteredCertificates.length > 0" class="table-footer">
         <span class="result-count">
           显示 {{ filteredCertificates.length }} / {{ certificates.length }} 条记录
         </span>
       </div>
     </template>
   </div>

  <!-- 删除确认对话框 -->
  <div v-if="deleteDialogVisible" class="dialog-overlay" @click.self="hideDeleteDialog">
    <div class="dialog">
      <div class="dialog-header">
        <h3 class="dialog-title">确认删除证书</h3>
        <button class="dialog-close" @click="hideDeleteDialog">×</button>
      </div>
      <div class="dialog-body">
        <p>您确定要删除证书 <strong>{{ deletingCertName }}</strong> 吗？</p>
        <p class="dialog-warning">此操作不可撤销，证书将被永久删除。</p>
      </div>
      <div class="dialog-footer">
        <button class="btn btn-secondary" @click="hideDeleteDialog" :disabled="deletingLoading">
          取消
        </button>
        <button class="btn btn-danger" @click="confirmDelete" :disabled="deletingLoading">
          <span v-if="deletingLoading" class="spinner" style="width: 16px; height: 16px;"></span>
          <span v-else>删除</span>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.certificate-list {
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
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 2rem;
  gap: 1rem;
  flex-wrap: wrap;
}

.page-title {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.page-subtitle {
  color: var(--text-muted);
  font-size: 0.9rem;
}

/* 工具栏 */
.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.search-box {
  position: relative;
  flex: 1;
  max-width: 400px;
}

.search-icon {
  position: absolute;
  left: 1rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  font-size: 1rem;
}

.search-input {
  width: 100%;
  padding: 0.625rem 1rem 0.625rem 2.5rem;
  font-size: 0.9rem;
  color: var(--text-primary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  transition: all 0.2s ease;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(129, 140, 248, 0.1);
}

.search-input::placeholder {
  color: var(--text-muted);
}

.sort-controls {
  display: flex;
  gap: 0.5rem;
}

.sort-btn {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.5rem 0.75rem;
  font-size: 0.8rem;
  color: var(--text-secondary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
}

.sort-btn:hover {
  color: var(--text-primary);
  border-color: var(--border-default);
}

.sort-btn.active {
  color: var(--accent-primary);
  border-color: var(--accent-primary);
  background: var(--bg-active);
}

.sort-indicator {
  font-size: 0.7rem;
}

/* 表格 */
.table-container {
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.table {
  margin: 0;
}

.table th {
  background: var(--bg-tertiary);
}

.cert-cn {
  font-weight: 500;
  color: var(--text-primary);
}

.cert-id {
  font-size: 0.75rem;
  color: var(--text-muted);
  font-family: 'JetBrains Mono', monospace;
}

.table-footer {
  display: flex;
  justify-content: flex-end;
  padding: 1rem;
}

.result-count {
  font-size: 0.8rem;
  color: var(--text-muted);
}

/* 模态对话框 */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease;
}

.dialog {
  background: var(--bg-primary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  width: 90%;
  max-width: 440px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid var(--border-subtle);
}

.dialog-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.dialog-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: var(--text-muted);
  cursor: pointer;
  line-height: 1;
  padding: 0;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  transition: all 0.2s;
}

.dialog-close:hover {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.dialog-body {
  padding: 1.5rem;
  color: var(--text-secondary);
  line-height: 1.6;
}

.dialog-body p {
  margin: 0 0 0.75rem 0;
}

.dialog-body p:last-child {
  margin-bottom: 0;
}

.dialog-warning {
  font-size: 0.85rem;
  color: var(--accent-danger);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--border-subtle);
  background: var(--bg-secondary);
  border-radius: 0 0 var(--radius-lg) var(--radius-lg);
}

/* 操作按钮组 */
.action-buttons {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.btn-sm {
  padding: 0.375rem 0.75rem;
  font-size: 0.8rem;
}

.btn-danger {
  color: var(--accent-danger);
  background: rgba(255, 77, 77, 0.1);
  border: 1px solid rgba(255, 77, 77, 0.3);
}

.btn-danger:hover {
  background: rgba(255, 77, 77, 0.2);
  border-color: var(--accent-danger);
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 4rem 2rem;
  background: var(--bg-secondary);
  border: 1px dashed var(--border-default);
  border-radius: var(--radius-lg);
}

.empty-icon {
  font-size: 3rem;
  color: var(--text-muted);
  margin-bottom: 1rem;
}

.empty-state h3 {
  font-size: 1.1rem;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.empty-state p {
  color: var(--text-muted);
  margin-bottom: 1.5rem;
}

@media (max-width: 768px) {
  .toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .search-box {
    max-width: none;
  }

  .sort-controls {
    justify-content: flex-start;
  }
}
</style>