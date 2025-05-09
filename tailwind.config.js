module.exports = {
	content: {
		files: ["*.html", "./src/**/*.rs"]
	},
	theme: {
		extend: {},
	},
	plugins: [require("@catppuccin/tailwindcss")({
		prefix: "ctp",
		defaultFlavour: "frappe",
	})],
}

