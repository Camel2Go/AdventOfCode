
ifndef year
year := `date +"%Y"`
endif
ifndef day
day := `date +"%d"`
endif

default: 
	@cd $(year) && cargo run --release --bin $(day)

.PHONY: help
help:
	@echo "usage:"
	@echo "	make [year=YYYY] [day=DD]"
	@echo "default:"
	@echo "	run current day"