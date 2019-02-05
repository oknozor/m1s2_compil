  .data
print_dec_string:
  .string "%d\n"
  .text
  .global main
main:
  call factorial_end
factorial:
  .set n,16
  pushq %rbp
  movq  %rsp,%rbp
  # test cond of If
  pushq n(%rbp)
  pushq $0
  popq %rbx
  popq %rax
  cmpq %rbx,%rax
  sete %al
  movsbq %al,%rax
  pushq %rax
  pushq n(%rbp)
  pushq $1
  popq %rbx
  popq %rax
  cmpq %rbx,%rax
  sete %al
  movsbq %al,%rax
  pushq %rax
  popq %rbx
  popq %rax
  orq  %rbx,%rax
  pushq %rax
  # if cond==false jmp else
  pop %rax
  cmp $0,%rax
  je  else_0
  # 
  pushq $1
  popq %rax
  popq %rbp
  ret
  jmp endif_0
else_0:
endif_0:
  pushq n(%rbp)
  pushq n(%rbp)
  pushq $1
  popq %rbx
  popq %rax
  subq %rbx,%rax
  pushq %rax
  call  factorial
  addq $8,%rsp
  pushq %rax
  popq %rbx
  popq %rax
  imulq %rbx
  pushq %rax
  popq %rax
  popq %rbp
  ret
  popq  %rbp
  ret
factorial_end: # nb function address is on top of stack
  popq %rax
  pushq $3
  call  factorial
  addq $8,%rsp
  pushq %rax
  popq %rax
  ret
