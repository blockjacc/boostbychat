<template>
  <div class="mt-6 p-4 bg-white/20 rounded-xl flex items-center space-x-4">
    <button v-if="!wallet.connected" class="btn" @click="wallet.connect">
      Connect Wallet
    </button>

    <div v-else>
      <button class="btn" @click="checkWallet">Check Wallet</button>
      <span v-if="status === 'connected'"
            class="text-green-400 font-bold">Connected ✅</span>
      <span v-else-if="status === 'mismatched accounts'"
            class="text-red-400 font-bold">Mismatched accounts ❌</span>
      <span v-else-if="status"
            class="text-yellow-300">{{ status }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useWallet } from 'solana-wallets-vue';

const props = defineProps<{ emp: { emp_id: string; pub_key: string } }>();

const wallet = useWallet();
const status = ref('');

async function checkWallet() {
  if (!wallet.publicKey.value) return;
  const resp = await fetch(
    `http://localhost:3001/api/wallet_check?emp_id=${props.emp.emp_id}`,
    {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ wallet_pubkey: wallet.publicKey.value.toBase58() }),
    },
  );
  const j = await resp.json();
  status.value = j.status || 'error';
}
</script>

<style scoped>
.btn { @apply bg-logo text-white rounded-md px-4 py-2; }
</style>
