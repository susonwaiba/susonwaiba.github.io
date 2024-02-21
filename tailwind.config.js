/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: ['class', '[data-theme="dark"]'],
  content: [
    './templates/**/*.html',
    './static/**/*.html',
    './static/**/*.js',
  ],
  theme: {
    container: {
      padding: '1.5rem',
      center: true,
      maxWidth: '920px',
    },
    extend: {},
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui"),
  ],
  daisyui: {
    themes: [
      {
        light: {
          ...require("daisyui/src/theming/themes")["light"],
          'primary': "#f44335",
          'primary-content': '#ffffff',
          'secondary': "teal",
          'base-100': '#f7fafc',
          'neutral': '#ffffff',
          'neutral-content': '#f44335',
        },
      },
      {
        dark: {
          ...require("daisyui/src/theming/themes")["dark"],
          'primary': "#f44335",
          'primary-content': '#ffffff',
          'secondary': "teal",
          'base-100': '#1c1b22',
          'neutral-content': '#f44335',
        },
      },
    ],
    darkTheme: "dark",
    styled: true,
    utils: true,
  },
}

