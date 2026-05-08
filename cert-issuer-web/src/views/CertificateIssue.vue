<script setup lang="ts">
import { ref, computed } from 'vue'
import { certificateApi, type IssueRequest } from '../services/api'
import { useRouter } from 'vue-router'

const router = useRouter()

const form = ref<IssueRequest>({
  cn: '',
  o: '',
  ou: '',
  l: '',
  st: '',
  c: '',
  validity_days: 365
})

const loading = ref(false)
const error = ref<string | null>(null)
const success = ref<string | null>(null)

const validityOptions = [
  { value: 30, label: '30 天' },
  { value: 90, label: '90 天' },
  { value: 180, label: '180 天' },
  { value: 365, label: '1 年' },
  { value: 730, label: '2 年' },
  { value: 1095, label: '3 年' }
]

const isValid = computed(() => {
  return form.value.cn.trim().length > 0
})

const handleSubmit = async () => {
  if (!isValid.value) return

  loading.value = true
  error.value = null
  success.value = null

  try {
    const result = await certificateApi.issueCertificate(form.value)
    success.value = `证书颁发成功！ID: ${result.id}`
    setTimeout(() => {
      router.push(`/certificates/${result.id}`)
    }, 1500)
  } catch (e: any) {
    error.value = e.response?.data?.message || e.message || '证书颁发失败'
  } finally {
    loading.value = false
  }
}

const resetForm = () => {
  form.value = {
    cn: '',
    o: '',
    ou: '',
    l: '',
    st: '',
    c: '',
    validity_days: 365
  }
  error.value = null
  success.value = null
}
</script>

<template>
  <div class="certificate-issue">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">颁发证书</h1>
        <p class="page-subtitle">创建新的数字证书</p>
      </div>
    </div>

    <div class="form-container">
      <form @submit.prevent="handleSubmit" class="issue-form">
        <div class="form-section">
          <h3 class="section-title">基本信息</h3>

          <div class="form-group">
            <label class="form-label required">Common Name (CN)</label>
            <input
              v-model="form.cn"
              type="text"
              class="input"
              placeholder="例如: example.com"
              required
            />
            <p class="form-hint">证书的主体名称，必须填写</p>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label class="form-label">Organization (O)</label>
              <input
                v-model="form.o"
                type="text"
                class="input"
                placeholder="组织名称"
              />
            </div>

            <div class="form-group">
              <label class="form-label">Organizational Unit (OU)</label>
              <input
                v-model="form.ou"
                type="text"
                class="input"
                placeholder="部门/单位"
              />
            </div>
          </div>
        </div>

        <div class="form-section">
          <h3 class="section-title">地理位置（可选）</h3>

          <div class="form-row">
            <div class="form-group">
              <label class="form-label">Locality (L)</label>
              <input
                v-model="form.l"
                type="text"
                class="input"
                placeholder="城市"
              />
            </div>

            <div class="form-group">
              <label class="form-label">State (ST)</label>
              <input
                v-model="form.st"
                type="text"
                class="input"
                placeholder="省份/州"
              />
            </div>

            <div class="form-group">
              <label class="form-label">Country (C)</label>
              <input
                v-model="form.c"
                type="text"
                class="input"
                placeholder="国家代码 (如 CN)"
                maxlength="2"
              />
            </div>
          </div>
        </div>

        <div class="form-section">
          <h3 class="section-title">有效期</h3>

          <div class="form-group">
            <label class="form-label">证书有效期</label>
            <div class="validity-options">
              <label
                v-for="option in validityOptions"
                :key="option.value"
                class="validity-option"
                :class="{ selected: form.validity_days === option.value }"
              >
                <input
                  type="radio"
                  :value="option.value"
                  v-model="form.validity_days"
                  class="sr-only"
                />
                {{ option.label }}
              </label>
            </div>
          </div>
        </div>

        <div v-if="error" class="form-message error-message">
          {{ error }}
        </div>

        <div v-if="success" class="form-message success-message">
          {{ success }}
        </div>

        <div class="form-actions">
          <button
            type="button"
            class="btn btn-secondary"
            @click="resetForm"
          >
            重置
          </button>
          <button
            type="submit"
            class="btn btn-primary"
            :disabled="!isValid || loading"
          >
            <span v-if="loading" class="spinner" style="width: 16px; height: 16px;"></span>
            <span v-else>✦</span>
            {{ loading ? '签发中...' : '签发证书' }}
          </button>
        </div>
      </form>
    </div>

    <div class="info-panel">
      <h4>签发须知</h4>
      <ul>
        <li>CN (Common Name) 是证书的核心标识，必须唯一且有效</li>
        <li>证书私钥由系统自动生成并妥善保管</li>
        <li>证书过期后需要重新签发</li>
        <li>签发后的证书可随时下载使用</li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.certificate-issue {
  animation: fadeIn 0.4s ease;
  max-width: 800px;
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
}

.page-subtitle {
  color: var(--text-muted);
  font-size: 0.9rem;
}

.form-container {
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  padding: 2rem;
  margin-bottom: 1.5rem;
}

.form-section {
  margin-bottom: 2rem;
}

.form-section:last-of-type {
  margin-bottom: 0;
}

.section-title {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-subtle);
}

.form-group {
  margin-bottom: 1rem;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
}

.form-label {
  display: block;
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.form-label.required::after {
  content: '*';
  color: var(--accent-danger);
  margin-left: 0.25rem;
}

.form-hint {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin-top: 0.375rem;
}

/* 有效期选项 */
.validity-options {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.validity-option {
  display: inline-flex;
  align-items: center;
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
  color: var(--text-secondary);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
}

.validity-option:hover {
  color: var(--text-primary);
  border-color: var(--accent-primary);
}

.validity-option.selected {
  color: var(--accent-primary);
  border-color: var(--accent-primary);
  background: var(--bg-active);
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

/* 表单消息 */
.form-message {
  margin: 1.5rem 0;
}

/* 表单操作 */
.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-subtle);
  margin-top: 1.5rem;
}

/* 信息面板 */
.info-panel {
  padding: 1.25rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
}

.info-panel h4 {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.75rem;
}

.info-panel ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.info-panel li {
  font-size: 0.8rem;
  color: var(--text-muted);
  padding: 0.375rem 0;
  padding-left: 1rem;
  position: relative;
}

.info-panel li::before {
  content: '›';
  position: absolute;
  left: 0;
  color: var(--accent-primary);
}

@media (max-width: 600px) {
  .form-container {
    padding: 1.25rem;
  }

  .form-actions {
    flex-direction: column;
  }

  .form-actions .btn {
    width: 100%;
  }
}
</style>