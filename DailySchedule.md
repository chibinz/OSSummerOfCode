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
