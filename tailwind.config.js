/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            textColor: {
                bg: 'var(--bg-colour)',
                accent: 'var(--accent-colour)',
                primary: 'var(--primary-colour)',
            },
            backgroundColor: {
                bg: 'var(--bg-colour)',
                accent: 'var(--accent-colour)',
                primary: 'var(--primary-colour)',
            },
            borderColor: {
                bg: 'var(--bg-colour)',
                accent: 'var(--accent-colour)',
                primary: 'var(--primary-colour)',
            },
        },
    },
    plugins: [],
};
