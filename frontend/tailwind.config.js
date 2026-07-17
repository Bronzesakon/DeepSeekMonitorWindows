/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        brand: {
          50: "#EEF2FF",
          100: "#E0E7FF",
          200: "#C7D2FE",
          300: "#A5B4FC",
          400: "#818CF8",
          500: "#4F6EF7",
          600: "#4F46E5",
          700: "#4338CA",
          800: "#3730A3",
          900: "#312E81",
        },
        flash: {
          400: "#38BDF8",
          500: "#0EA5E9",
          600: "#0284C7",
        },
        pro: {
          400: "#A78BFA",
          500: "#8B5CF6",
          600: "#7C3AED",
        },
      },
      boxShadow: {
        glass: "0 8px 32px -4px rgba(0,0,0,0.08), 0 2px 8px rgba(0,0,0,0.04)",
        "glass-lg": "0 16px 48px -6px rgba(0,0,0,0.10), 0 4px 12px rgba(0,0,0,0.05)",
      },
      borderRadius: {
        "2xl": "16px",
      },
    },
  },
  plugins: [],
};
