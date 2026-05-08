import axios from 'axios'

const api = axios.create({
  baseURL: '/api',
  timeout: 10000
})

export interface Certificate {
  id: string
  subject: {
    cn: string
    o?: string
    ou?: string
    l?: string
    st?: string
    c?: string
  }
  issuer: string
  serial_number: string
  not_before: string
  not_after: string
  fingerprint?: string
  status?: string
}

export interface IssueRequest {
  cn: string
  o?: string
  ou?: string
  l?: string
  st?: string
  c?: string
  validity_days: number
}

export interface HealthStatus {
  status: string
  timestamp?: string
}

export const certificateApi = {
  getHealth: async (): Promise<HealthStatus> => {
    const response = await api.get('/health')
    return response.data
  },

  getCertificates: async (): Promise<Certificate[]> => {
    const response = await api.get('/certificates')
    return response.data
  },

  getCertificate: async (id: string): Promise<Certificate> => {
    const response = await api.get(`/certificates/${id}`)
    return response.data
  },

  issueCertificate: async (data: IssueRequest): Promise<Certificate> => {
    const response = await api.post('/certificates/issue', data)
    return response.data
  },

  downloadCertificate: async (id: string, format: string = 'pem', password?: string): Promise<Blob> => {
    let url = `/certificates/${id}/cert?format=${format}`
    if (password && format === 'pfx') {
      url += `&password=${encodeURIComponent(password)}`
    }
    const response = await api.get(url, {
      responseType: 'blob'
    })
    return response.data
  }
}

export default api