/* Remove all unwanted sections as they can break code layout. */
/DISCARD/ :
{
	*(.comment*)
	*(.eh_frame*)
	*(.gcc_except_table*)
	*(.note*)
	*(.rel.eh_frame*)
	*(.interp)
}
