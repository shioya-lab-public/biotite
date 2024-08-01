irs := $(wildcard ir/*.ll)
objs := main.o {objs} $(irs:.ll=.o)
out := $(notdir $(CURDIR))

$(out): $(objs) image.s mapping.ld
	$$CLANG -O$$OPT -static $(objs) image.s -T mapping.ld -lm -o $@

$(objs): %.o: %.ll
	$$CLANG -O$$OPT -c $< -o $@

clean:
	rm -rf $(objs) $(out)
