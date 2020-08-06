# rCoreSummerOfCode
Code, Documentation, and Daily Record for rCore Summer of Code 2020

## 日历
| Mon  | Tues | Wed  | Thur | Fri  | Sat  | Sun  |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|      |      |  1   |  [2](##Day-0)   |  [3](##Day-1)   | [4](##Day-2) | [5](##Day-3) |
|  [6](##Day-4)   |  [7](##Day-5)   |  [8](##Day-6)   |  [9](##Day-7)   |  [10](##Day-8)  |  [11](##Day-9)  |  [12](##Day-10)  |
|  [13](##Day-11)  |  [14](##Day-12)  |  [15](##Day-13)  |  [16](##Day-14)  |  [17](##Day-15)  |  [18](##Day-16)  |  [19](##Day-17)  |
|  [20](##Day-18)  |  [21](##Day-19)  |  [22](##Day-20)  |  [23](##Day-21)  |  [24](##Day-22)  |  [25](##Day-23)  |  [26](##Day-24)  |
|  [27](##Day-25)  |  [28](##Day-26)  |  [29](##Day-27)  |  [30](##Day-28)  |      |      |      |

## 汇总
- Step 0 Rust 热身
    - [Rustlings](step0/rustlings)
    - [15道小编程题](step0/15_programming_exercise)
- Step 1 RISC-V ISA 学习
    - [RV32I 解释器/模拟器](step1/project/p1_emulator)
    - [用logisim实现一个简单的RISC-V CPU](step1/project/p2_simulator)
        - ALU
        - Register File
        - Control Unit
        - etc.
    - 阅读RISC-V Privileged ISA Specification
- Step 2 rCore-Tutorial 实验
    - [Lab 0](step2/doc/lab0.md)
    - [Lab 1](step2/doc/lab1.md)
    - [Lab 2](step2/doc/lab2.md)
    - [Lab 3](step2/doc/lab3.md)
    - [Lab 4](step2/doc/lab4.md)
    - [Lab 5](step2/doc/lab5.md)
    - [Lab 6](step2/doc/lab6.md)

## Day 0 7.2
- 正式入群
- rustlings

今天刚把最后一门考试考完了。由于之前已经写过不少Rust了，加上开发Pintos的艰难历程，前面的step 0，step 1 打算尽快完成，多留点时间给后面的rCore tutorial。
完成了rustlings中variables, functions, if部分.

## Day 1
- rustlings

完成了rustlings中primitive_types, structs, enums, tests, modules, macros, move_semantics, errors, result, option 部分。 做起来挺快的，明天再把剩下的做完估计就可以进入下一阶段了。

## Day 2
- rustlings
- 阅读rCore-Tutorial Lab 0

threads, conversions 这两部分因为之前用的不多花的时间比较长，不过总算是把Rustlings写完了。在犹豫要不要写15道小程序题，有点想直接开始做rCore lab了。

## Day 3
- 搭好实验环境

今天把实验环境搭好了，manjaro上面和ubuntu的package manager不太一样，还要下载一个qemu-extra才能运行risc-v，不知道是不是我操作有不对的地方。实验零相当于一个hello world，平时写的c都会链接到操作系统的运行时（linux下是crt0），libc里面的malloc其实是syscall的wrapper。但这次是自己写操作系统，所以稍微要复杂一些。Rust也一样，不能用标准库std里面的utility，另外还要定义一个自己的panic handler。

## Day 4
- 复现Lab 0
- 阅读Learn C the Hard Way

开始做lab 0和lab 1了，不过有一点不太明白，repository里面都是已经写好了的代码，并没有要填充的地方。no_std和panic handler都已经写好了，中断的时候保存寄存器还有内核栈指针的汇编也都有了，还要做些啥不太清楚。可能是因为tutorial还在更新的缘故吧。既然这样那明天先看看15道小程序题，写起来应该蛮快的。

## Day 5
- Learn C the Hard Way * 2

一天都在上课，回到寝室差不多就要睡觉了。赶紧用Rust写了两道Learn C the Hard Way的练习题。前面主要是格式化输出和变量类型，没有控制流，所以很快就写完了。

## Day 6
- Learn C the Hard Way * 8

完成了8道练习题。对于Rust语法上面没有啥理解的困难，主要是体会Rust和C Convention上的区别。以数组遍历为例，在C里面很自然的一种写法就是`for (int i = 0; i < len; i++) ...`。 但在Rust里面如果实现了Iterator这个trait的话更鼓励使用iterators，比如`for arg in std::env::args() {...}`。 最直观的好处有两个，一可读性更高了，二没有index不需要每次都做bounds check，效率变高了。按照C的写法来写Rust不是不可以，只是Bad Style。Rust是一个多范式语言，其中不少函数式编程的特性可以看到sml的影子。map, fold, filter, 还有match，以及||closure等。通常对array/vector/list等线性数据结构的元素做一些简单的操作完全没有必要写for loop，一是容易出错，二是map这样的函数更方便compiler做优化（vectorize）。Match对于乍接触Rust的C程序员来说就是switch语句，但事实上要强大的多，switch仅限于整型（char，int，long...)，而match可以destruct tuples/structs，实现了PartialEq的自定义类型可以直接match。就个人经历而言，这个解决的最大痛点就是字符串也可以match了。总之，学习过程中要注意两者的区别，不能写成Cish Rust。

## Day 7
- Learn C the Hard Way * 5（全部完成）

终于把15道练习题写完了。Learn C the Hard Way后面主要是写一些数据结构。由于Ownership系统的原因，在Rust里面写一个双向链表都是一个非常痛苦的过程。甚至有人专门出了一本书研究Rust写链表多种不同的方式。链表其实在操作系统中是一个非常重要的数据结构。pintos里面记录线程的active_list, block_list, sleep_list都是链表，链表的每个节点内嵌在thread结构体里面，这样就不需要动态分配内存了。而需要取到指向thread结构体指针的时候可以通过offset()宏计算链表节点field据结构体base的偏移量获得。当时第一次看到这种骚操作被震撼了，不得不佩服想出这种办法的人。Page Eviction中的second chance algorithm(evict by used/dirty bit)中也有用到链表。有些C的概念不能直接转换到Rust里面，比如Rust的lifetime，ownership还有array和slice。C里面的lifetime有automatic（stack上），static（全局等），还有heap上自己管理的，safe Rust的就复杂很多。C里面函数传数组(指针）往往还需要一个长度的参数，Rust则合二为一成了slice，确保不会有数组越界的情况。基于以上原因，ex8，ex22更多的参考了Rust By Example里面的练习。

## Day 8
- 提交dbg宏pr
- 阅读rCore-Tutorial Lab 1

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

## Day 9
- 看完操作系统慕课中断部分

今天又仔细看了一下step2的要求，发现自己自己的确对实验的内容理解有误。rCore-Tutorial里面是已经写好的代码，自己要独立的实现一遍，或者分析有哪些不足，提出改进。每一个实验都要有实验报告和相关的代码。其实文档写的已经很详细了，但是为了获得更系统的理解还是去看了清华2020操作系统的慕课。讲的更细一些，偏向于原理。把中断这一部分看完了。

## Day 10
- 交流会
- 完成 Lab 0 学习记录撰写

今天的交流会开完之后感慨颇深，已经有差不多1/4的同学进行到lab4之后的学习了，更有一小部分完成了所有rCore-Tutorial实验，而自己还止步于lab 0/1。其实如果一开始就写lab的话进度应该是赶得上的，但是开始还有犹豫了一下，把前面的部分做了做。
知道自己进度落后了就要努力追赶。今天完成了lab0的学习记录，lab1做的也差不多了。本来想一并把lab1的学习记录写了，但琢磨了一下，一天走两步可能太操之过急了。实现lab是一部分，更重要的是理解和自己的思考。因此lab0和以后的学习记录大概分为三个部分：**问题，思考，以及改进**。虽然说tutorial基本上每一步都已经spoonfed到了嘴边，但是自己写的过程中难免有些疏忽，出现一些问题。回头看看这些问题是不是自己某些概念理解有误，想当然当成了另外一种东西。思考是对tutorial中作出指示没有细讲原因的思考和拓展。仓库里的代码是直接能跑的，复制黏贴过来没有太大意义，只有尝试着进行改动或者改进才能说明真正的掌握了这一块内容。因为学习记录是帮助自己理解所以更要注重内容（自己的体悟），而没必要纠结于形式，毕竟不是为了老师写的报告。

下面截取一段lab0学习记录中的思考, 表达能力还有待提高……
> 2. 关于为什么要做objcopy

> Tutorial里面其实讲的已经蛮详细的了，这里谈谈自己对free standing binary的理解。一个程序在硬盘上和内存中的储存方式是不同的，通常来说可执行文件更紧凑一些，而在内存中运行的程序松散一些，是被拉伸过的。比如ELF中bss段只存大小和起始地址，不存数据，因为在文件中放一大串0是没有意义的。把可执行文件加载到内存中的过程是由操作系统完成的，bss段会分配已经置零闲置的内存页。这通常没有什么问题，但是这一次是我们自己写操作系统，因此没有段的概念。kernel文件本来二进制表示什么样在bootloader(qemu)装载到内存中就是什么样。这也是为什么kernel.bin被叫做镜像。

实验的代码和学习记录因为和DailySchedule没有太大关系，为了不污染commit记录，单独放在一个repository里面了。

## Day 11
- 完成 Lab 1 实验， 学习记录
- 在相关issue上comment了一下

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

## Day 12
- 完成了 Lab 2 实验
- 看完了操作系统慕课虚拟内存相关内容

完成了lab 2实验部分，明天写报告。说实话对于PhysicalAddress这一个wrapper不是很满意，一个地址本质上就是一个usize。包装下来产生了很多boilerplate代码，对内部的usize做加减法还需要implement Add，Sub这样的trait。如果想要对usize这样的primitive实现dynamic dispatch的话，可以定义一个trait，然后为usize implement这个trait。Buddy System Allocator这一部分之前的os和算法课没有讲过，后期有空可以自己实现一个了。在教学用的toy system上面碎片化其实不是一个很大的问题，因为用的内存很少，不需要垃圾回收。lab2部分为了对FRAME_ALLOCATOR这个全局变量进行访问使用了spin::Mutex，在没有os支持下不知道是怎么实现的，很有可能是busy wait，被阻塞的线程会进入一个死循环。lazy_static的使用还不是特别清楚。
预习了lab 3部分的实验，感觉难度上要比lab0/1大不少，还是需要多花一点功夫的。

## Day 13
- 完成 Lab 2 学习记录
- 预习 Lab 3 虚拟内存，页表相关内容

完成了lab 2 writeup。逐渐明白kernel在内存中是怎样放置的了。从0x8000_0000开始先是OpenSBI的firmware，0x80020000会放置第一个程序，这也是为什么linker.ld里面kernel_base_address是0x80020000。kernel本身主要是两部分组成，最前面的是.text代码部分，除了我们自己写的代码之外还有Rust自己生成的一些库函数。之后是数据，.data和.rodata，都是初始值非零的变量，一个可写另一个不可写。最一部分是.bss段，虽然是.bss但并不能保证操作系统运行第一条指令时这一段内存是0。个人认为操作系统应该手动置零，但是不知道为什么现在没有置零但是依然能够正常运行。.bss段除了操作系统方便自己动态分配内存的一个堆HEAP之外还有其他的变量。用rust-objdump -x还能看到类似FRAME_ALLOCATOR被mangle后的字样。.bss段的终止地址也是kernel_end_address。物理页的分配在这之后开始，不过要对齐到4K，也就是16进制后三位是0x000。QEMU默认的dram大小是128MiB，理论上这块内存想干什么都可以。物理页可以从0x88000000开始分配，到0x87fff000，一次往前分配也是可以的。

## Day 14
- 完成 Lab 3 实验， 学习记录
- 预习 Lab 4

完成了lab 3的实验和学习记录 。今天效率还是比较高的，实验部分tutorial里面有的是自己敲得，剩下的因为依赖声明太麻烦了，所以直接就从tutorial仓库复制黏贴过来了。感觉还是有不少可以改进的部分，不过现在的重点可能是先完成实验，把自己的想法放在了writeup“改进”的部分。entry.S里面有关相对地址和绝对地址的加载其实可以写的更简洁明了一些，明天提交一个pull request。

## Day 15
- 跟着rCore-Tutorial过了一遍Lab 4，存在问题
- 提交entry.S相关pr
- issue中回复Lab 2相关问题

今天尝试着在做lab 4，不是很顺利。tutorial里面的代码和仓库里面的代码还是有很多不一样的。自己修好编译错误之后操作系统就一直提示StoreFault，有很大的可能是给线程初始化页表的时候出了一些问题。下午提交了一个pr，并在issue中回答了一个关于lab2内存分配的问题。当时自己在这里也思考了很久，希望对提问题的同学有所帮助。

## Day 16
- Debug Lab 4
- 看完了操作系统慕课线程相关内容

介于昨天lab 4进展不顺，今天看了看OS课程视频，回顾线程和进程的概念。OS设计的初衷是为了实现multitasking，最早的雏形是time sharing system。简单的来说，就是把cpu时间进行分割，分配给不同的程序（multiplex cpu time）。最容易想到的multiplexing算法是round robin，把cpu时间均分，但是这显然不够灵活，不能满足日记增长复杂的需求。并且同时运行的两个程序，他们在运行过程中不能访问重叠的地址，这意味着说为了实现多任务需要重新编译用户程序。为了应对第一个问题，OS提出了scheduler，和thread的概念，把运行中的程序抽象成thread，方便scheduler进行调度。thread随时都能够被中断，中断后能够恢复原来的状态，这里存储的状态就是Context（上下文）。为了应对第二个问题，OS提出了虚拟地址的概念，不同程序的的虚拟地址可以有重叠，但是虚拟地址最后映射到的物理地址一定是没有冲突的。每次访问内存的话都做一次翻译软件上实现会十分耗时，因此虚拟地址到物理地址的翻译就交给了硬件（mmu）。

## Day 17
- 交流会
- 预习 Lab 5 内容

Lab 4 debug不是很成功。明天不打算硬磕了，先放一下，开始后面的Lab学习。Lab 5是设备树的初始化与文件系统，其实大部分文件系统的逻辑已经在rCore-fs上实现了，我们所要做的就是调用它。另外后面的实验题也出来了，还是先把实验指导的内容完成在考虑后面的吧。今天的会议上看到了别的同学的每日记录，坦诚地来讲要细致很多，自己在这方面还有提升的空间，把之前的记录分条整理整理。

## Day 18
- 修复 Lab 4 之前实验中的bug
- 完成 Lab 4 学习记录书写
- 开始 Lab 5 实验（部分）

Lab 4 的实验和学习记录书写完成。之前的问题好像出在了add_segment函数上面了。为了和教程匹配, 把init_data这一项参数删去了，因为本来从alloc_page_range传进来的时候就是None，没有用上。可能删的时候不小心把物理页分配相关的代码页删去了。导致往栈上读写数据的时候会有Store/LoadFault。最后的解决办法是把rCore-Tutorial lab-4分支memory文件夹覆盖了过来，重新搞好依赖声明。之前做的修改全都没有了，后面等所有lab完成之后再改吧。Lab 5的代码量不是很大，今天也已经完成了一部分，文件系统因为是调用外部的crate，只有寥寥几行，重点是理解virtio以及设备树的遍历。

## Day 19
- 完成 Lab 5 实验，学习记录
- 整理DailySchedule
    - 跳转到具体某一天的日历
    - 各个step成果汇总
- 添加step0，step1为submodule

相比于做Lab 4时的艰难，Lab 5实现的整个过程就很顺畅，基本上没有遇到什么问题。因为很快就把实验内容和学习记录写完了，剩下的时间回顾了一下这三个周学习rCore的成果，还是比较有收获的。在DailySchedule里面添加了日历和汇总，方便别的同学查看。同时在每天的记录里面分条呈现当日跟rCore相关的事件。整理完之后结构清晰了不少。

## Day 19
- 完成 Lab 6 实验，学习记录

终于终于，把所有lab都做完了！后面几个lab感觉压缩不少内容，看教程很快就过了，其实包含操作系统的许多不同方面。还剩下实验题没有完成，明天继续！

## Day 20
- 完成 Lab 1，Lab 2（部分）实验题
- 在两个issues上面做了回复

大佬们实习的实习，去实验室科研的科研。而我的暑假已经过了1/3，啥也不懂，啥也不会orz。I'm so vegetable...!

## Day 21
- 回复了一个issue
今天一天都在外边，说实话没有来得及看实验题。晚上回到寝室发现之前lab 4实验题我提出的一些疑惑由被发到了issues上面，做了些回复。实习的第一阶段即将步入尾声，还有一部分实验题没有完成。不过不打算在剩下的两天里面搞突击了，这不是实习的本意。梳理一下之前的成果，认真写完总结报告。看看第二阶段项目列举出来的有没有自己感兴趣的，起草一份proposal。为第一阶段画上句号。

## Day 22
- 完成 Lab 3 实验题
- 完成总结报告（部分）
总结报告写是写个差不多了，但自己读一读总是有流水账的嫌疑。最后希望呈现的不是各个实验学习记录的简单组合而是自己的体验和感悟，提炼出来一些统领性的东西。明天再打磨一下，实在不行把今天这份交上去……

## Day 22
- 参加交流会
- 完成总结报告
- 提交调查问卷
之前Lab 6的代码是在linux那台机器上写的，一直忘了push了，今天补push了一下。总结报告写的比较随性，不过也是真实的感悟。老师说接下来的几天里面可以一直改，明天打算再polish一下。说了这么多，相比于hackathon还是希望能够参加到正式的实习中去。不过这个要看学校的行政了，明天去问问。

## Day 23
- 整理daily schedule
- 预习zCore
本来还想再readme里面对于repository做一些说明，干脆把daily schedule搬到了readme里面。

## Day 24
- 阅读fuchsia官方文档中关于zircon kernel的文档
官方文档基本上描述的是What，how但是没有说why。kernel object的设计和传统的unix 理念不太一样。这一部分还是不太明白。可能需要再搜一搜，读一读王润基学长的theisis。

## Day 25
学校这边开证明无果，问陈渝和李睿老师也没有回复，积极性受到了很大的打击。

## Day 27
- 学校这边实习备案通过了！
- 订好了机票

## Day 28
- 收拾行李
- 最后给自己放一天假，和同学出去玩

# zCore

## Day 1
终于见到了参加活动的各位同学，助教和老师们。上午是向勇老师和王润基学长对zCore的介绍，下午是各个同学的自我介绍，谈一谈自己感兴趣的方向以及接下来的打算。还是有不少同学挺有想法的。个人选的是测试与文档维护，把这个做好也不错啦。

## Day 2
上午是对昨天讨论的继续，王润基学长非常有耐心的给我们介绍了一遍zCore内部的组件，做了一些答疑。他也提到写文档是需要对代码有一定理解的。如果我们自己对这个项目一知半解，可能写出来的tutorial对于读者可能是一种误解。下午听了国科大的同学介绍他们的一生一芯项目，了解到他们在流片过程中遇到的种种困难，真人真事和新闻报道还是有一定区别的。他们遇到各种诡异的bug和debug的奇技淫巧，以及为了绕过硬件缺陷而做的软件上的经历十分有意思。希望我们学校以后也能够给本科生流片的机会。晚上王润基学长谈了谈zCore实现的心路历程，tql……

## Day 3
上午初步完成了对文档工作的分工，我负责的是虚拟内存。代码主要是位于zircon-object/vm/vmar.rs文件里面。大概有1000多行，数量上还是不少的。但是实现过一遍rCore-Tutorial，vmar可以大概类比memory_set,包含了进程大部分的内存管理信息。详细阅读了一遍fucshia.dev中concept和reference section中关于memory management的描述。之后和王润基学长讨论一下这部分代码，印证了一些自己的想法，学长还对代码中的一些写法做出了解释。下午参观了RIOS实验室，得到了一些关于PicoRio开发情况的信息。还是蛮期待的，不过距离板子出来可能还有一段时间。晚上请的是sipeed的工程师给我们做k210的介绍，后来发现他们做产品的初衷可能和我们的focus不太一样，maixpy的主要目标客户是STEAM（Science，Technology……）领域，需要高层次的抽象和封装，只暴露方便好用的接口就行了，具体硬件实现不需要关心。而我们的侧重点则恰恰相反，os本身就需要对硬件资源进行抽象和管理。不过作为一个pioneer做成这样已经不错了，更何况性价比这么高。

## Day 4
上午和向老师进行了座谈，对于文档完善过程中的一些细节进行了讨论。文档的完善主要分为两个部分，一个是在zCore代码中直接嵌入注释和单元测试；二是对于zCore的某一部分（我分到的是虚拟内存）进行分析和记录，作为后期整合到简明zCore教程中的一个模本。第一部分在阅读代码的过程中顺道完成，第二部分需要对于模块的代码文档进行记录和深入思考，把思绪梳理成一个文档。明天需要对这几天的讨论结果做一个报告，又是报告……好多啊……简单分点列一下吧

### Deliverables
- unit test
    - coverage 90%
- inline doc
    - pass `#[deny(missing_doc)]`
    - related files
        - zircon-object/src/vm/vmar.rs
        - zircon-syscall/src/vmar.rs
        - etc.
- tutorial/notes
    - synopsis
    - dependencies/dependents
    -

- Timeline
    - week 1
        - finish inline docs
    - week 2
        - all unit test coverage > 90%
        - first draft tutorial complete
        - communicate with partner to propose a wrap up section
    - week 3
        - polish draft tutorial
        - 8.28
        - everything should be done by then
        - 8.28 ~ 8.31
        - buffer for wrap up work

- Reach out goal
    - Refactor zCore code along way
    - Organize notes as tutorial
    - Descriptive -> Procedural
    - What, How -> Why
