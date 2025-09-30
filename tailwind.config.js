/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs", // Scan all .rs files in the src directory
    "./index.html",   // Scan the index.html file
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}