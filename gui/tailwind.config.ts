import type { Config } from 'tailwindcss';

export default {
  content: [
    './index.html',
    './src/**/*.{svelte,ts,tsx}',
  ],
  theme: {
    extend: {
      fontFamily: {
        upheaval: ['Upheaval', 'sans-serif'],
      },
    },
  },
} satisfies Config;
