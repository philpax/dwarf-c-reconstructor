Help me write a program to recover C header from DWARF info. Parse it into a tree architecture.
• Most element type will have a name and an optional line number
• The outer layer will be a list of file. They could be c file which is the CU name, or h file which is the file referenced in DW_AT_decl_file. There is no need to discern if a file is a source or a header file. In these file there can be namespace or not. Then there could be structure, enums, class, global variable, subprogram..., these could be children of that namespace(s) or direct children of the file.
• A structure or enum will have its name (or not, as it can be typedefed later), its line number and its member
• A class is the same as structure but can have child subprogram and accessibility field to indicicate private or public
• A subprogram can have its line number, its argument, its return type, and its body. This body can be a bunch of things like variable or inlined subrountine, or a direct lexical_block which can be a bunch of other thing
• As lots of things will be referenced from this to that, you have to keep track of stuff. An inlined subroutine can reveal more info about a subprogram (such as line number that is called when inlined in parent subprogram, or its child variable and lexical block). To get die that reference other die, use `get_DIE_from_attribute` instead of offset
• A class subprogram can have the same name (overload)
• The subprogram of class will not have much info, instead somewhere later (outside of that class DIE) there will be subprograms where `specification` points back to these subprogram.
• One of its formal_parameter will be called `this` which can be skipped
• As there are multiple CU, there can be many duplicated information
• Pointer asterisk should be next to variable, except in the case of function return value where it should be next to the return type
• Array size should be on the right side of the variable, i.e char str[2048]
• Try to write code in a compact ways to reduce token usage, if anything have same behaviour merge them to make code shorter. Struct, Enum, Union are basically the same with minor different. Global variable, function body variable, function arguments, class members are basically the same as an element that has a type, a name and a line number (with accessibility for class member, extern or static for global, ...)
• Take notice that sometimes things can be on the same line, such as function definition and its arguments, or multiple variable declartion on the same line 
Finally output in this format
```
// file_implementation.c
namespace my_namespace {
void doThing() //193
{
	int a; //195
	int b, *c;	//196

	inlined_function(); //198
}

int anotherThing(int a, //523
				float b) //524
{
	{
		int i; //530
		int y; //534
		{
			float z; //536
		}
	}
}
} //my_namespace
// file_implementation.c
// file_header.h
namespace my_namespace {
struct Vector3 { //72
	int x, *y, z; //73
}; 

typedef struct {
	char r, g, b, a; //56
} Color; //57
} //my_namespace
// file_header.h
```