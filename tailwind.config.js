/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    colors: {
      light: "#FAF9FA",
      "light-accent": "#8AB0B8",
      main: "#E7BF00",
      dark: "#282835",
      "dark-accent": "#C9453E",
      white: "#EEEEEE",
      black: "#080808",
    },
    extend: {
      fontFamily: {
        sans: ["Ubuntu Mono", "monospace"],
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
