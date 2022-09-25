/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [require('daisyui')],
  daisyui: {
    themes: [
      {
        kglight: {
          primary: '#147ad4',
          secondary: '#5dcc37',
          accent: '#EB6B47',
          neutral: '#d1d5db',
          'base-100': '#fffef7',
          info: '#0092D6',
          success: '#6CB288',
          warning: '#DAAD58',
          error: '#AB3D30',
        },
      },
      {
        kgdark: {
          primary: '#085ec0',
          secondary: '#3f9361',
          accent: '#EB6B47',
          neutral: '#d1d5db',
          'base-100': '#111827',
          info: '#0092D6',
          success: '#6CB288',
          warning: '#DAAD58',
          error: '#AB3D30',
        },
      },
    ],
    darkTheme: 'kgdark',
  },
}
