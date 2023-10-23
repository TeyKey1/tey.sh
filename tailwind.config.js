/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "class",
  theme: {
    colors: {
      light: "#FAF9FA",
      "light-accent": "#8AB0B8",
      main: "#E7BF00",
      dark: "#282835",
      "dark-accent": "#C9453E",
      white: "#EEEEEE",
      "white-dim": "#FFFFFF80",
      black: "#080808",
    },
    extend: {
      fontFamily: {
        sans: ["JetBrains Mono", "monospace"],
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
