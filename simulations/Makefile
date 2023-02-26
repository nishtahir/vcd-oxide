SIMULATOR_COMPILER ?= iverilog
SIMULATOR_COMPILER_FLAGS ?= -g2012 -DSIMULATION -Y .sv 

SIMULATOR ?= vvp
SIMULATOR_FLAGS ?= -v -N

SOURCES = $(wildcard *.sv)
SIM_SOURCES = $(wildcard *.sv)
VVPS = $(patsubst %.sv, %.vvp, $(SIM_SOURCES))
VCDS = $(patsubst %.sv, %.vcd, $(SIM_SOURCES))

default: all

.PHONY:all
all: $(VVPS)

$(VVPS): %.vvp: %.sv $(SOURCES)
	$(SIMULATOR_COMPILER) $(SIMULATOR_COMPILER_FLAGS) -o $@ $^ 
	$(SIMULATOR) $(SIMULATOR_FLAGS) $@