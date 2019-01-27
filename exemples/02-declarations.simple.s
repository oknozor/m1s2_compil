  .data
print_dec_string:
  .string "%d\n"
  .text
  .global main
main:
  .data
x:
  .long 0 # warning this is an undefined value...
  .text
  .data
y:
  .long 2
  .text
  .data
z:
  .string "Hello, World!"
  .text
  ret
