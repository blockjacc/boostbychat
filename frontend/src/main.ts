/* ───────────────── src/main.ts ───────────────── */
import { createApp } from 'vue';
import './index.css';
import App from './App.vue';

/* ── router ---------------------------------------------------------------- */
import { router } from './router';     // src/router/index.ts

/* ── Buffer shim (required by some wallet libs) ---------------------------- */
import { Buffer } from 'buffer';
(window as any).Buffer = Buffer;

/* ── Solana wallets -------------------------------------------------------- */
import SolanaWallets            from 'solana-wallets-vue';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { SolflareWalletAdapter }from '@solana/wallet-adapter-solflare';
import 'solana-wallets-vue/styles.css';

/* ───────────────── create + mount app ───────────────── */
createApp(App)
  .use(router)                               // Vue-Router
  .use(SolanaWallets, {                      // wallet plugin
    autoConnect: false,
    wallets: [
      new PhantomWalletAdapter(),
      new SolflareWalletAdapter(),
    ],
  })
  .mount('#app');

  