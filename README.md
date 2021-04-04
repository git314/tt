Prints the contents of a toml file in `data`

```
./target/release/tt
```

```
	bill_length_mm          island         species
	         <dbl>           <str>           <str>
		39.1 	"Torgersen" 	"Adelie" 
		39.5 	"Torgersen" 	"Adelie" 
		40.3 	"Torgersen" 	"Adelie" 
		nan 	"Torgersen" 	"Adelie" 
		36.7 	"Torgersen" 	"Adelie" 
		39.3 	"Torgersen" 	"Adelie" 
		38.9 	"Torgersen" 	"Adelie" 
		39.2 	"Torgersen" 	"Adelie" 
		34.1 	"Torgersen" 	"Adelie" 
```

Next steps

1. make cli to accept toml as argument
2. Make sure values are aligned with column header
3. Make sure nan is printed as NA with distinct red

