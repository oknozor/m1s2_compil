  .data
print_dec_string:
  .string "%d\n"
  .text
  .global main
main:
  .data
x:
  .long 0
  .text
loop_1:
  # eval cond
  pushq x(%rip)
  pushq $10
  popq %rbx
  popq %rax
  cmpq %rbx,%rax
  setl %al
  movsbq %al,%rax
  pushq %rax
  # exit if cond == false
  popq %rax
  cmpq $0,%rax
  je   exitloop_1
  # eval the body
  movq x(%rip),%rax
  inc %rax
  movq %rax,x(%rip)
  pushq %rax
  popq %rax
  pushq x(%rip)
  popq  %rsi
  leaq  print_dec_string(%rip), %rdi
  movl  $0, %eax
  call    printf@PLT
  pushq %rax
  popq %rax
  # end of body
  jmp loop_1
exitloop_1:
  # loop exit
  ret
