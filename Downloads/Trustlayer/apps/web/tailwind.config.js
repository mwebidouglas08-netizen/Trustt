/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        trust: {
          50: "#eef4ff",
          100: "#dbe7ff",
          200: "#bfd6ff",
          300: "#93bbff",
          400: "#6094ff",
          500: "#3b6df8",
          600: "#254eed",
          700: "#1d3dd9",
          800: "#1f34af",
          900: "#1f328a",
        },
        ink: {
          900: "#0b1220",
          800: "#121a2b",
          700: "#1a263d",
        },
      },
      fontFamily: {
        sans: ["var(--font-sans)", "ui-sans-serif", "system-ui", "sans-serif"],
      },
    },
  },
  plugins: [],
};
