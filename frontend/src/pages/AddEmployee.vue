<template>
    <div class="page-wrapper">
      <h2 class="page-title">Add employees</h2>
  
      <div class="flex gap-2 mb-4">
        <input v-model="empId"  placeholder="8-digit ID" class="input w-40">
        <input v-model="pubKey" placeholder="Solana pubkey" class="input flex-1">
        <button class="btn-primary" @click="save">Save</button>
      </div>
  
      <table class="w-full text-left">
        <thead>
          <tr><th>Employee ID</th><th>Public Key</th></tr>
        </thead>
        <tbody>
          <tr v-for="row in rows" :key="row.emp_id">
            <td>{{ row.emp_id }}</td><td>{{ row.pub_key }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import axios from 'axios'
  
  const empId  = ref('')
  const pubKey = ref('')
  const rows   = ref<{ emp_id:string; pub_key:string }[]>([])
  
  async function fetchRows() {
    rows.value = (await axios.get('/api/employees')).data
  }
  async function save() {
    if (!/^\d{8}$/.test(empId.value) || pubKey.value.length === 0) return
    await axios.post('/api/employees',{ emp_id:empId.value, pub_key:pubKey.value })
    empId.value=''; pubKey.value=''
    fetchRows()
  }
  onMounted(fetchRows)
  </script>
  