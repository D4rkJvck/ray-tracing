launch: clean_terminal
	cargo r > assets/output.ppm
clean_terminal:
	clear
