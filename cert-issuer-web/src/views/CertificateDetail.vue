<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { certificateApi, type Certificate } from '../services/api'

const route = useRoute()
const router = useRouter()

const certificate = ref<Certificate | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)
const downloading = ref(false)
const selectedFormat = ref('pem')
const pfxPassword = ref('')

const certId = computed(() => route.params.id as string)

const formatOptions = [
  { value: 'pem', label: 'PEM (.pem)', desc: '适用于大多数服务器 (Apache, Nginx, Linux)' },
  { value: 'der', label: 'DER (.der)', desc: '二进制格式 (Java, Windows)' },
  { value: 'pfx', label: 'PFX/PKCS12 (.pfx)', desc: '包含私钥 (IIS, Windows Server)' },
  { value: 'nginx', label: 'Nginx 配置', desc: 'Nginx 配置文件模板' },
  { value: 'iis', label: 'IIS 配置', desc: 'IIS 导入说明' }
]

const fetchCertificate = async () => {
  loading.value = true
  error.value = null

  try {
    certificate.value = await certificateApi.getCertificate(certId.value)
  } catch (e: any) {
    error.value = e.message || '获取证书详情失败'
  } finally {
    loading.value = false
  }
}

const downloadCertificate = async () => {
  if (downloading.value) return

  // Validate password for PFX
  if (selectedFormat.value === 'pfx' && !pfxPassword.value) {
    error.value = '请输入PFX证书密码'
    return
  }

  downloading.value = true
  error.value = null

  try {
    const format = selectedFormat.value
    const password = selectedFormat.value === 'pfx' ? pfxPassword.value : undefined
    const blob = await certificateApi.downloadCertificate(certId.value, format, password)
    
    // 获取文件名
    let filename = `${certificate.value?.subject.cn || 'certificate'}`
    if (format === 'pem') filename += '.pem'
    else if (format === 'der') filename += '.der'
    else if (format === 'pfx') filename += '.pfx'
    else if (format === 'nginx') filename += '-nginx.conf'
    else if (format === 'iis') filename += '-iis.txt'
    
    // 创建下载链接
    const url = window.URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = filename
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    window.URL.revokeObjectURL(url)
  } catch (e: any) {
    error.value = '证书下载失败'
  } finally {
    downloading.value = false
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

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleString('zh-CN')
}

onMounted(fetchCertificate)
</script>

<template>
  <div class="certificate-detail">
    <div class="page-header">
      <button class="back-btn" @click="router.push('/certificates')">
        ← 返回列表
      </button>
    </div>

    <div v-if="loading" class="loading">
      <div class="spinner"></div>
    </div>

    <div v-else-if="error" class="error-message">
      {{ error }}
    </div>

    <template v-else-if="certificate">
      <div class="detail-header">
        <div class="cert-title">
          <h1 class="page-title">{{ certificate.subject.cn }}</h1>
          <span class="tag" :class="getStatusTag(certificate).class">
            {{ getStatusTag(certificate).text }}
          </span>
        </div>
        <div class="header-actions">
          <div class="download-section">
            <div class="format-selector">
              <label class="format-label">下载格式</label>
              <select v-model="selectedFormat" class="format-select">
                <option v-for="opt in formatOptions" :key="opt.value" :value="opt.value">
                  {{ opt.label }}
                </option>
              </select>
            </div>
            <button
              class="btn btn-primary"
              @click="downloadCertificate"
              :disabled="downloading"
            >
              <span v-if="downloading" class="spinner" style="width: 16px; height: 16px;"></span>
              <span v-else>↓</span>
              下载
            </button>
          </div>
        </div>
      </div>

      <!-- 格式说明 -->
      <div class="format-info">
        <div class="format-cards">
          <div 
            v-for="opt in formatOptions" 
            :key="opt.value"
            class="format-card"
            :class="{ selected: selectedFormat === opt.value }"
            @click="selectedFormat = opt.value"
          >
            <div class="format-radio">
              <span v-if="selectedFormat === opt.value">●</span>
              <span v-else>○</span>
            </div>
            <div class="format-details">
              <div class="format-name">{{ opt.label }}</div>
              <div class="format-desc">{{ opt.desc }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- PFX密码输入 -->
      <div v-if="selectedFormat === 'pfx'" class="pfx-password-section">
        <div class="pfx-password-card">
          <div class="pfx-password-header">
            <span class="pfx-icon">🔐</span>
            <span>PFX证书密码设置</span>
          </div>
          <div class="pfx-password-form">
            <input 
              v-model="pfxPassword"
              type="password" 
              class="input pfx-password-input" 
              placeholder="输入PFX证书密码"
              autocomplete="new-password"
            />
            <p class="pfx-password-hint">
              密码用于保护PFX证书文件，导入IIS或Windows时需要此密码<br>
              <span class="hint-default">默认密码: changeit</span>
            </p>
          </div>
        </div>
      </div>

      <div class="detail-grid">
        <!-- 基本信息 -->
        <div class="detail-card">
          <h3 class="card-title">基本信息</h3>
          <div class="info-list">
            <div class="info-item">
              <span class="info-label">证书 ID</span>
              <span class="info-value mono">{{ certificate.id }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">序列号</span>
              <span class="info-value mono">{{ certificate.serial_number }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">签发机构</span>
              <span class="info-value">{{ certificate.issuer }}</span>
            </div>
          </div>
        </div>

        <!-- 主体信息 -->
        <div class="detail-card">
          <h3 class="card-title">主体信息 (Subject)</h3>
          <div class="info-list">
            <div class="info-item">
              <span class="info-label">Common Name (CN)</span>
              <span class="info-value">{{ certificate.subject.cn }}</span>
            </div>
            <div class="info-item" v-if="certificate.subject.o">
              <span class="info-label">Organization (O)</span>
              <span class="info-value">{{ certificate.subject.o }}</span>
            </div>
            <div class="info-item" v-if="certificate.subject.ou">
              <span class="info-label">Organizational Unit (OU)</span>
              <span class="info-value">{{ certificate.subject.ou }}</span>
            </div>
            <div class="info-item" v-if="certificate.subject.l">
              <span class="info-label">Locality (L)</span>
              <span class="info-value">{{ certificate.subject.l }}</span>
            </div>
            <div class="info-item" v-if="certificate.subject.st">
              <span class="info-label">State (ST)</span>
              <span class="info-value">{{ certificate.subject.st }}</span>
            </div>
            <div class="info-item" v-if="certificate.subject.c">
              <span class="info-label">Country (C)</span>
              <span class="info-value">{{ certificate.subject.c }}</span>
            </div>
          </div>
        </div>

        <!-- 有效期 -->
        <div class="detail-card">
          <h3 class="card-title">有效期</h3>
          <div class="info-list">
            <div class="info-item">
              <span class="info-label">生效时间</span>
              <span class="info-value">{{ formatDate(certificate.not_before) }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">到期时间</span>
              <span class="info-value">{{ formatDate(certificate.not_after) }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">剩余天数</span>
              <span class="info-value">
                {{
                  Math.max(0, Math.ceil(
                    (new Date(certificate.not_after).getTime() - Date.now()) / (1000 * 60 * 60 * 24)
                  ))
                }} 天
              </span>
            </div>
          </div>
        </div>

        <!-- 指纹信息 -->
        <div class="detail-card" v-if="certificate.fingerprint">
          <h3 class="card-title">证书指纹</h3>
          <div class="info-list">
            <div class="info-item full">
              <span class="info-label">SHA-256</span>
              <span class="info-value mono fingerprint">{{ certificate.fingerprint }}</span>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.certificate-detail {
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
  margin-bottom: 1.5rem;
}

.back-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  font-size: 0.85rem;
  color: var(--text-secondary);
  background: none;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
}

.back-btn:hover {
  color: var(--text-primary);
  border-color: var(--border-default);
  background: var(--bg-hover);
}

.detail-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  margin-bottom: 2rem;
  flex-wrap: wrap;
}

.cert-title {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.page-title {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text-primary);
}

.header-actions {
  display: flex;
  gap: 0.75rem;
}

/* 详情卡片网格 */
.detail-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 1rem;
}

.detail-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  padding: 1.5rem;
  transition: all 0.2s ease;
}

.detail-card:hover {
  border-color: var(--border-default);
}

.card-title {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 1rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--border-subtle);
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 0.875rem;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}

.info-item.full {
  flex-direction: column;
}

.info-label {
  font-size: 0.8rem;
  color: var(--text-muted);
  flex-shrink: 0;
}

.info-value {
  font-size: 0.85rem;
  color: var(--text-primary);
  text-align: right;
  word-break: break-all;
}

.info-value.mono {
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.75rem;
}

.info-value.fingerprint {
  font-size: 0.7rem;
  line-height: 1.6;
  background: var(--bg-tertiary);
  padding: 0.75rem;
  border-radius: var(--radius-md);
  margin-top: 0.5rem;
  word-break: break-all;
}

/* 下载区域 */
.download-section {
  display: flex;
  align-items: flex-end;
  gap: 0.75rem;
}

.format-selector {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.format-label {
  font-size: 0.75rem;
  color: var(--text-muted);
  font-weight: 500;
}

.format-select {
  padding: 0.5rem 0.75rem;
  font-size: 0.85rem;
  color: var(--text-primary);
  background: var(--bg-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 140px;
}

.format-select:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(129, 140, 248, 0.1);
}

/* 格式说明卡片 */
.format-info {
  margin-bottom: 2rem;
}

.format-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 0.75rem;
}

.format-card {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding: 0.875rem 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
}

.format-card:hover {
  border-color: var(--border-default);
}

.format-card.selected {
  border-color: var(--accent-primary);
  background: var(--bg-active);
}

.format-radio {
  font-size: 1rem;
  color: var(--accent-primary);
  flex-shrink: 0;
  line-height: 1;
}

.format-details {
  flex: 1;
  min-width: 0;
}

.format-name {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.125rem;
}

.format-desc {
  font-size: 0.7rem;
  color: var(--text-muted);
  line-height: 1.4;
}

/* PFX密码区域 */
.pfx-password-section {
  margin-bottom: 2rem;
  animation: slideDown 0.3s ease;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.pfx-password-card {
  background: linear-gradient(135deg, var(--bg-secondary), var(--bg-tertiary));
  border: 1px solid var(--accent-warning);
  border-radius: var(--radius-lg);
  padding: 1.25rem;
}

.pfx-password-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--accent-warning);
  margin-bottom: 1rem;
}

.pfx-icon {
  font-size: 1.1rem;
}

.pfx-password-form {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.pfx-password-input {
  max-width: 400px;
}

.pfx-password-hint {
  font-size: 0.75rem;
  color: var(--text-muted);
  line-height: 1.5;
}

.hint-default {
  color: var(--text-secondary);
  font-weight: 500;
}

@media (max-width: 600px) {
  .detail-header {
    flex-direction: column;
  }

  .header-actions {
    width: 100%;
  }

  .download-section {
    flex-direction: column;
    align-items: stretch;
  }

  .format-selector {
    width: 100%;
  }

  .format-select {
    width: 100%;
  }

  .header-actions .btn {
    width: 100%;
  }

  .format-cards {
    grid-template-columns: 1fr;
  }

  .info-item {
    flex-direction: column;
    gap: 0.25rem;
  }

  .info-value {
    text-align: left;
  }
}
</style>