import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '../views/Dashboard.vue'
import CertificateList from '../views/CertificateList.vue'
import CertificateIssue from '../views/CertificateIssue.vue'
import CertificateDetail from '../views/CertificateDetail.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: Dashboard
    },
    {
      path: '/certificates',
      name: 'certificate-list',
      component: CertificateList
    },
    {
      path: '/certificates/issue',
      name: 'certificate-issue',
      component: CertificateIssue
    },
    {
      path: '/certificates/:id',
      name: 'certificate-detail',
      component: CertificateDetail
    }
  ]
})

export default router