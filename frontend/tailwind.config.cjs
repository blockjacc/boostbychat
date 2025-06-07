/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary:   '#29587c',
        secondary: '#513270',
        accent:    '#26e8e3',
        logo:      '#5be3c2',     // ← your bg-logo utility
      },
    },
  },
  plugins: [],
};
