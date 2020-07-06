# DailySchedule

## Day 0 7.2

今天刚把最后一门考试考完了。由于之前已经写过不少Rust了，加上开发Pintos的艰难历程，前面面的step 0，step 1 打算尽快完成，多留点时间给后面的rCore tutorial。

完成了rustlings中variables, functions, if部分

## Day 1 7.3

完成了rustlings中primitive_types, structs, enums, tests, modules, macros, move_semantics, errors, result, option 部分。 做起来挺快的，明天再把剩下的做完估计就可以进入下一阶段了。

## Day 2 7.4

threads, conversions 这两部分因为之前用的不多花的时间比较长，不过总算是把Rustlings写完了。在犹豫要不要写15道小程序题，有点想直接开始做rCore lab了。

## Day 3 7.5

今天把实验环境搭好了，manjaro上面和ubuntu的package manager不太一样，还要下载一个qemu-extra才能运行risc-v，不知道是不是我操作有不对的地方。实验零相当于一个hello world，平时写的c都会链接到操作系统的运行时（linux下是crt0），libc里面的malloc其实是syscall的wrapper。但这次是自己写操作系统，所以稍微要复杂一些。Rust也一样，不能用标准库std里面的utility，另外还要定义一个自己的panic handler。

## Day 4 7.6
开始做lab 0和lab 1了，不过有一点不太明白，repository里面都是已经写好了的代码，并没有要填充的地方。no_std和panic handler都已经写好了，中断的时候保存寄存器还有内核栈指针的汇编也都有了，还要做些啥不太清楚。可能是因为tutorial还在更新的缘故吧。既然这样那明天先看看15道小程序题，写起来应该蛮快的。