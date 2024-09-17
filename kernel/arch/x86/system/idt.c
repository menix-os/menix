// Interrupt descriptor table setting

#include <menix/io/mmio.h>
#include <menix/system/arch.h>
#include <menix/util/log.h>

#include <gdt.h>
#include <idt.h>
#include <interrupts.h>
#include <io.h>
#include <pic.h>

ATTR(aligned(CONFIG_page_size)) static IdtDesc idt_table[IDT_MAX_SIZE];
ATTR(aligned(0x10)) static IdtRegister idtr;

void idt_set(u8 idx, void* handler, u8 type_attr)
{
	IdtDesc* const target = idt_table + idx;
	const usize ptr = (usize)handler;

	target->base_0_15 = ptr & 0xFFFF;
	target->base_16_31 = (ptr >> 16) & 0xFFFF;
	target->selector = offsetof(Gdt, kernel_code);
	target->type = type_attr;
	target->reserved = 0;
#if CONFIG_bits >= 64
	target->base_32_63 = (ptr >> 32) & 0xFFFFFFFF;
	target->reserved2 = 0;
#endif
}

void idt_reload()
{
	idtr.limit = sizeof(idt_table) - 1;	   // Limit is the last entry, not total size.
	idtr.base = idt_table;
	asm volatile("lidt %0" ::"m"(idtr));
}

// Macro garbage to declare 256 functions.
#define DUP2 \
	INT_HANDLER_DECL(__COUNTER__); \
	INT_HANDLER_DECL(__COUNTER__);
#define DUP4   DUP2 DUP2
#define DUP8   DUP4 DUP4
#define DUP16  DUP8 DUP8
#define DUP32  DUP16 DUP16
#define DUP64  DUP32 DUP32
#define DUP128 DUP64 DUP64
#define DUP256 DUP128 DUP128

DUP256
#define IDT_SET(num) idt_set(num, INT_HANDLER(num), IDT_TYPE(0, IDT_GATE_INT))

void idt_init()
{
	asm_interrupt_disable();

	// Set all gates.

	// clang-format off
	IDT_SET(0); IDT_SET(1); IDT_SET(2); IDT_SET(3); IDT_SET(4); IDT_SET(5); IDT_SET(6); IDT_SET(7); IDT_SET(8); IDT_SET(9);
	IDT_SET(10); IDT_SET(11); IDT_SET(12); IDT_SET(13); IDT_SET(14); IDT_SET(15); IDT_SET(16); IDT_SET(17); IDT_SET(18); IDT_SET(19);
	IDT_SET(20); IDT_SET(21); IDT_SET(22); IDT_SET(23); IDT_SET(24); IDT_SET(25); IDT_SET(26); IDT_SET(27); IDT_SET(28); IDT_SET(29);
	IDT_SET(30); IDT_SET(31); IDT_SET(32); IDT_SET(33); IDT_SET(34); IDT_SET(35); IDT_SET(36); IDT_SET(37); IDT_SET(38); IDT_SET(39);
	IDT_SET(40); IDT_SET(41); IDT_SET(42); IDT_SET(43); IDT_SET(44); IDT_SET(45); IDT_SET(46); IDT_SET(47); IDT_SET(48); IDT_SET(49);
	IDT_SET(50); IDT_SET(51); IDT_SET(52); IDT_SET(53); IDT_SET(54); IDT_SET(55); IDT_SET(56); IDT_SET(57); IDT_SET(58); IDT_SET(59);
	IDT_SET(60); IDT_SET(61); IDT_SET(62); IDT_SET(63); IDT_SET(64); IDT_SET(65); IDT_SET(66); IDT_SET(67); IDT_SET(68); IDT_SET(69);
	IDT_SET(70); IDT_SET(71); IDT_SET(72); IDT_SET(73); IDT_SET(74); IDT_SET(75); IDT_SET(76); IDT_SET(77); IDT_SET(78); IDT_SET(79);
	IDT_SET(80); IDT_SET(81); IDT_SET(82); IDT_SET(83); IDT_SET(84); IDT_SET(85); IDT_SET(86); IDT_SET(87); IDT_SET(88); IDT_SET(89);
	IDT_SET(90); IDT_SET(91); IDT_SET(92); IDT_SET(93); IDT_SET(94); IDT_SET(95); IDT_SET(96); IDT_SET(97); IDT_SET(98); IDT_SET(99);
	IDT_SET(100); IDT_SET(101); IDT_SET(102); IDT_SET(103); IDT_SET(104); IDT_SET(105); IDT_SET(106); IDT_SET(107); IDT_SET(108); IDT_SET(109);
	IDT_SET(110); IDT_SET(111); IDT_SET(112); IDT_SET(113); IDT_SET(114); IDT_SET(115); IDT_SET(116); IDT_SET(117); IDT_SET(118); IDT_SET(119);
	IDT_SET(120); IDT_SET(121); IDT_SET(122); IDT_SET(123); IDT_SET(124); IDT_SET(125); IDT_SET(126); IDT_SET(127); IDT_SET(128); IDT_SET(129);
	IDT_SET(130); IDT_SET(131); IDT_SET(132); IDT_SET(133); IDT_SET(134); IDT_SET(135); IDT_SET(136); IDT_SET(137); IDT_SET(138); IDT_SET(139);
	IDT_SET(140); IDT_SET(141); IDT_SET(142); IDT_SET(143); IDT_SET(144); IDT_SET(145); IDT_SET(146); IDT_SET(147); IDT_SET(148); IDT_SET(149);
	IDT_SET(150); IDT_SET(151); IDT_SET(152); IDT_SET(153); IDT_SET(154); IDT_SET(155); IDT_SET(156); IDT_SET(157); IDT_SET(158); IDT_SET(159);
	IDT_SET(160); IDT_SET(161); IDT_SET(162); IDT_SET(163); IDT_SET(164); IDT_SET(165); IDT_SET(166); IDT_SET(167); IDT_SET(168); IDT_SET(169);
	IDT_SET(170); IDT_SET(171); IDT_SET(172); IDT_SET(173); IDT_SET(174); IDT_SET(175); IDT_SET(176); IDT_SET(177); IDT_SET(178); IDT_SET(179);
	IDT_SET(180); IDT_SET(181); IDT_SET(182); IDT_SET(183); IDT_SET(184); IDT_SET(185); IDT_SET(186); IDT_SET(187); IDT_SET(188); IDT_SET(189);
	IDT_SET(190); IDT_SET(191); IDT_SET(192); IDT_SET(193); IDT_SET(194); IDT_SET(195); IDT_SET(196); IDT_SET(197); IDT_SET(198); IDT_SET(199);
	IDT_SET(200); IDT_SET(201); IDT_SET(202); IDT_SET(203); IDT_SET(204); IDT_SET(205); IDT_SET(206); IDT_SET(207); IDT_SET(208); IDT_SET(209);
	IDT_SET(210); IDT_SET(211); IDT_SET(212); IDT_SET(213); IDT_SET(214); IDT_SET(215); IDT_SET(216); IDT_SET(217); IDT_SET(218); IDT_SET(219);
	IDT_SET(220); IDT_SET(221); IDT_SET(222); IDT_SET(223); IDT_SET(224); IDT_SET(225); IDT_SET(226); IDT_SET(227); IDT_SET(228); IDT_SET(229);
	IDT_SET(230); IDT_SET(231); IDT_SET(232); IDT_SET(233); IDT_SET(234); IDT_SET(235); IDT_SET(236); IDT_SET(237); IDT_SET(238); IDT_SET(239);
	IDT_SET(240); IDT_SET(241); IDT_SET(242); IDT_SET(243); IDT_SET(244); IDT_SET(245); IDT_SET(246); IDT_SET(247); IDT_SET(248); IDT_SET(249);
	IDT_SET(250); IDT_SET(251); IDT_SET(252); IDT_SET(253); IDT_SET(254); IDT_SET(255);
	// clang-format on

	idt_reload();

	asm_interrupt_enable();
}
