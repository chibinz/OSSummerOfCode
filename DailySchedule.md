# DailySchedule

## Day 0 7.2
今天刚把最后一门考试考完了。由于之前已经写过不少Rust了，加上开发Pintos的艰难历程，前面的step 0，step 1 打算尽快完成，多留点时间给后面的rCore tutorial。
完成了rustlings中variables, functions, if部分.

## Day 1 7.3
完成了rustlings中primitive_types, structs, enums, tests, modules, macros, move_semantics, errors, result, option 部分。 做起来挺快的，明天再把剩下的做完估计就可以进入下一阶段了。

## Day 2 7.4
threads, conversions 这两部分因为之前用的不多花的时间比较长，不过总算是把Rustlings写完了。在犹豫要不要写15道小程序题，有点想直接开始做rCore lab了。

## Day 3 7.5
今天把实验环境搭好了，manjaro上面和ubuntu的package manager不太一样，还要下载一个qemu-extra才能运行risc-v，不知道是不是我操作有不对的地方。实验零相当于一个hello world，平时写的c都会链接到操作系统的运行时（linux下是crt0），libc里面的malloc其实是syscall的wrapper。但这次是自己写操作系统，所以稍微要复杂一些。Rust也一样，不能用标准库std里面的utility，另外还要定义一个自己的panic handler。

## Day 4 7.6
开始做lab 0和lab 1了，不过有一点不太明白，repository里面都是已经写好了的代码，并没有要填充的地方。no_std和panic handler都已经写好了，中断的时候保存寄存器还有内核栈指针的汇编也都有了，还要做些啥不太清楚。可能是因为tutorial还在更新的缘故吧。既然这样那明天先看看15道小程序题，写起来应该蛮快的。

## Day 5 7.7
一天都在上课，回到寝室差不多就要睡觉了。赶紧用Rust写了两道Learn C the Hard Way的练习题。前面主要是格式化输出和变量类型，没有控制流，所以很快就写完了。

## Day 6 7.8
完成了8道练习题。对于Rust语法上面没有啥理解的困难，主要是体会Rust和C Convention上的区别。以数组遍历为例，在C里面很自然的一种写法就是`for (int i = 0; i < len; i++) ...`。 但在Rust里面如果实现了Iterator这个trait的话更鼓励使用iterators，比如`for arg in std::env::args() {...}`。 最直观的好处有两个，一可读性更高了，二没有index不需要每次都做bounds check，效率变高了。按照C的写法来写Rust不是不可以，只是Bad Style。Rust是一个多范式语言，其中不少函数式编程的特性可以看到sml的影子。map, fold, filter, 还有match，以及||closure等。通常对array/vector/list等线性数据结构的元素做一些简单的操作完全没有必要写for loop，一是容易出错，二是map这样的函数更方便compiler做优化（vectorize）。Match对于乍接触Rust的C程序员来说就是switch语句，但事实上要强大的多，switch仅限于整型（char，int，long...)，而match可以destruct tuples/structs，实现了PartialEq的自定义类型可以直接match。就个人经历而言，这个解决的最大痛点就是字符串也可以match了。总之，学习过程中要注意两者的区别，不能写成Cish Rust。

## Day 7 7.9
终于把15道练习题写完了。Learn C the Hard Way后面主要是写一些数据结构。由于Ownership系统的原因，在Rust里面写一个双向链表都是一个非常痛苦的过程。甚至有人专门出了一本书研究Rust写链表多种不同的方式。链表其实在操作系统中是一个非常重要的数据结构。pintos里面记录线程的active_list, block_list, sleep_list都是链表，链表的每个节点内嵌在thread结构体里面，这样就不需要动态分配内存了。而需要取到指向thread结构体指针的时候可以通过offset()宏计算链表节点field据结构体base的偏移量获得。当时第一次看到这种骚操作被震撼了，不得不佩服想出这种办法的人。Page Eviction中的second chance algorithm(evict by used/dirty bit)中也有用到链表。有些C的概念不能直接转换到Rust里面，比如Rust的lifetime，ownership还有array和slice。C里面的lifetime有automatic（stack上），static（全局等），还有heap上自己管理的，safe Rust的就复杂很多。C里面函数传数组(指针）往往还需要一个长度的参数，Rust则合二为一成了slice，确保不会有数组越界的情况。基于以上原因，ex8，ex22更多的参考了Rust By Example里面的练习。

## Day 8 7.10
今天给rCore-Tutorial提交了第一个pull requst，写了一个`dbg!`macro。这个宏还是蛮有用的，可以显示行号，文件名，同时返回变量的值，自己平时用的蛮多的。rCore因为不能用标准库所以默认没有实现这个宏。实现起来也不麻烦，也就几十行，简单的在println！上面做一些包装。看了一下lab-1这个branch好像也是已经写好了的代码，没有题目…… 感觉sbi调用可以用enum包装的更好一些。比如:
```Rust
enum SBICall {
    SetTimer = 0,
    ConsolePutChar = 1,
    consoleGetChar = 2,
    ...
}

// Cast enum to usize when needed

or

enum SBICall {
    SetTimer(usize),
    ConsolePutChar(u8),
    ConsoleGetChar,
}

impl SBICall {
    fn get_id(&self) -> usize {
        match self {
            ConsolePutChar(_) => 1,
            ...
        }
    }
}
```
由于SBI_SET_TIMER这一类global constant本质上是一个usize，调用sbi_call的时候可以传一些非法的值进去，运行时还需做检查。而如果用enum的话可以在编译时就杜绝这种可能性,同时enum有自己独立的namespace不用大写的SBI_ Prefix了。当然RISC-V的sbi本身就是用C定义的，这么做存在着过度包装的风险，可以开个issue问一问。明天前面的lab题目还不出来的话，只能先从后面的开始做起了。

## Day 9 7.11
今天又仔细看了一下step2的要求，发现自己自己的确对实验的内容理解有误。rCore-Tutorial里面是已经写好的代码，自己要独立的实现一遍，或者分析有哪些不足，提出改进。每一个实验都要有实验报告和相关的代码。其实文档写的已经很详细了，但是为了获得更系统的理解还是去看了清华2020操作系统的慕课。讲的更细一些，偏向于原理。把中断这一部分看完了。

## Day 10 7.12
今天的交流会开完之后感慨颇深，已经有差不多1/4的同学进行到lab4之后的学习了，更有一小部分完成了所有rCore-Tutorial实验，而自己还止步于lab 0/1。其实如果一开始就写lab的话进度应该是赶得上的，但是开始还有犹豫了一下，把前面的部分做了做。
知道自己进度落后了就要努力追赶。今天完成了lab0的学习记录，lab1做的也差不多了。本来想一并把lab1的学习记录写了，但琢磨了一下，一天走两步可能太操之过急了。实现lab是一部分，更重要的是理解和自己的思考。因此lab0和以后的学习记录大概分为三个部分：**问题，思考，以及改进**。虽然说tutorial基本上每一步都已经spoonfed到了嘴边，但是自己写的过程中难免有些疏忽，出现一些问题。回头看看这些问题是不是自己某些概念理解有误，想当然当成了另外一种东西。思考是对tutorial中作出指示没有细讲原因的思考和拓展。仓库里的代码是直接能跑的，复制黏贴过来没有太大意义，只有尝试着进行改动或者改进才能说明真正的掌握了这一块内容。因为学习记录是帮助自己理解所以更要注重内容（自己的体悟），而没必要纠结于形式，毕竟不是为了老师写的报告。

下面截取一段lab0学习记录中的思考, 表达能力还有待提高……
> 2. 关于为什么要做objcopy

> Tutorial里面其实讲的已经蛮详细的了，这里谈谈自己对free standing binary的理解。一个程序在硬盘上和内存中的储存方式是不同的，通常来说可执行文件更紧凑一些，而在内存中运行的程序松散一些，是被拉伸过的。比如ELF中bss段只存大小和起始地址，不存数据，因为在文件中放一大串0是没有意义的。把可执行文件加载到内存中的过程是由操作系统完成的，bss段会分配已经置零闲置的内存页。这通常没有什么问题，但是这一次是我们自己写操作系统，因此没有段的概念。kernel文件本来二进制表示什么样在bootloader(qemu)装载到内存中就是什么样。这也是为什么kernel.bin被叫做镜像。

实验的代码和学习记录因为和DailySchedule没有太大关系，为了不污染commit记录，单独放在一个repository里面了。

## Day 11 7.13
完成了lab 1的实验与学习报告。这一部分因为riscv这个crate包装的已经很好了，所以代码量也不是很大，逻辑很清晰。写过程中遇到了一个很有意思的问题，因为interrupt.S大部分代码其实是在保存通用寄存器，所以不是很想一行一行的敲了，把rCore-Tutorial里面的interrupt.asm复制黏贴过来。结果运行的时候遇到了store exception，看来是往禁止的地方写了东西。仔细检查发现, 在保存context之前把sp和sscratch做了交换，sp指向了后面才会实现的内核栈，因此产生了exception。复制黏贴果然是一大bug来源，以后要慎之又慎。后面按照洛佳同学提出的方法把30个SAVE合成了一个loop，干净很多。

```RISC-V
# Essential for substitution %i
.altmacro

# length of general purpose registers in bytes
.set reg_size, 8
# No. of registers inside a context frame
.set context_size, 34

# Load register relative to sp
.macro load reg, offset
    ld  \reg, \offset * reg_size(sp)
.endm

.macro load_gp n
    load    x\n, \n
.endm

...

.global __restore
__restore:
    # Restore csr registers
    load    t0, 32
    load    t1, 33
    csrw    sstatus, t0
    csrw    sepc, t1

    .set i, 3
    .rept 29
        load_gp %i
        .set i, i + 1
    .endr

    # Restore ra and sp last
    load    x1, 1
    load    x2, 2

    # Return to the address stored in sepc
    sret
```
在相关的issue上面comment了一下，希望能有follow up。