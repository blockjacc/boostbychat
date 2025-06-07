<!-- src/pages/VerifyEmployee.vue -->
<template>
  <div class="page-wrapper">
    <h2 class="page-title">Verify wallet</h2>

    <table class="w-full text-left mb-6">
      <thead>
        <tr><th class="w-8"></th><th>Employee ID</th><th>Public Key</th></tr>
      </thead>

      <tbody>
        <tr v-for="row in rows" :key="row.emp_id">
          <td>
            <input
              type="radio"
              name="emp"
              :value="row"
              v-model="selected"
              class="accent-violet-500"
            />
          </td>
          <td>{{ row.emp_id }}</td>
          <td class="truncate max-w-[16rem]">{{ row.pub_key }}</td>
        </tr>
      </tbody>
    </table>

    <button
      v-if="rows.length"
      :disabled="!selected || status === 'checking'"
      class="btn-primary disabled:opacity-40"
      @click="verify"
    >
      {{ buttonLabel }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import axios          from 'axios';
import { useWallet }  from 'solana-wallets-vue';

interface Row { emp_id: string; pub_key: string }

const rows     = ref<Row[]>([]);
const selected = ref<Row | null>(null);
const status   = ref<'idle'|'checking'|'connected'|
                 'mismatched accounts'|'not found'|'error'>('idle');

const wallet   = useWallet();   // store object

/* fetch the list once ------------------------------------------------ */
async function fetchRows() {
  rows.value = (await axios.get<Row[]>('/api/employees')).data;
}
onMounted(fetchRows);

/* verify ------------------------------------------------------------- */
async function verify() {
  if (!selected.value) return;
  status.value = 'checking';

  /* connect wallet if needed */
  if (!wallet.connected) {
    try { await wallet.connect(); }
    catch { status.value = 'error'; return; }
  }

  const walletPub = wallet.publicKey;   // <-- already a base-58 string
  if (!walletPub) { status.value = 'error'; return; }

  try {
    const { data } = await axios.post(
      `/api/employees/${selected.value.emp_id}/verify`,
      { wallet_pubkey: walletPub },
    );
    status.value = data.status;
  } catch {
    status.value = 'error';
  }
}

/* label -------------------------------------------------------------- */
const buttonLabel = computed(() => {
  switch (status.value) {
    case 'checking':  return 'Checking…';
    case 'connected': return '✅ Connected';
    case 'mismatched accounts': return '❌ Mismatch';
    case 'not found': return 'Not in DB';
    case 'error':     return 'Error';
    default:          return 'Connect wallet';
  }
});

watch(() => wallet.connected, c => { if (!c) status.value = 'idle'; });
</script>
