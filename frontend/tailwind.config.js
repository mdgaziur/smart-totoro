/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'text': '#25293f',
        'background': '#d9d9d9',
        'primary': '#8cad58',
        'secondary': '#a0bad0',
        'accent': '#052b33',
      }
    },
  },
  plugins: [],
};
