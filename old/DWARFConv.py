import sys
from collections import defaultdict
from elftools.elf.elffile import ELFFile
from elftools.dwarf.die import DIE
from elftools.dwarf.dwarf_expr import DWARFExprParser

class Element:
    def __init__(self, name=None, line=None, parent=None):
        self.name = name
        self.line = line
        self.parent = parent
        self.children = []
        
    def add_child(self, child):
        self.children.append(child)
        child.parent = self
        
    def get_path(self):
        if self.parent is None:
            return []
        return self.parent.get_path() + [self.name] if self.name else self.parent.get_path()

class File(Element):
    def __init__(self, name, line=None):
        super().__init__(name, line)
        self.namespaces = {}  # Map namespace paths to namespace objects
        
    def __str__(self):
        result = []
        result.append(f"// {self.name}")
        
        # Direct children (not in namespaces)
        direct_children = [c for c in self.children if not isinstance(c, Namespace)]
        for child in direct_children:
            result.append(str(child))
            
        # Children in namespaces
        for ns in self.namespaces.values():
            result.append(str(ns))
            
        result.append(f"// {self.name}")
        return '\n\n'.join(result)

class Namespace(Element):
    def __init__(self, name, line=None, parent=None):
        super().__init__(name, line, parent)
        
    def __str__(self):
        result = []
        result.append(f"namespace {self.name} {{")
        
        for child in self.children:
            child_str = str(child)
            # Indent each line
            indented = '\n'.join(f"\t{line}" for line in child_str.split('\n'))
            result.append(indented)
            
        result.append(f"}} //{self.name}")
        return '\n\n'.join(result)

class TypeElement(Element):
    def __init__(self, name, element_type, line=None, parent=None):
        super().__init__(name, line, parent)
        self.element_type = element_type
        
class Compound(TypeElement):
    def __init__(self, name, type, line=None, parent=None, is_typedef=False, typedef_name=None, typedef_line=None):
        super().__init__(name, type, line, parent)
        self.members = []
        self.type = type
        self.is_typedef = is_typedef
        self.typedef_name = typedef_name
        self.typedef_line = typedef_line
        
    def add_member(self, member):
        self.members.append(member)
        
    def add_value(self, name, value=None):
        self.members.append((name, value))
        
    def __str__(self):
        result = []

        opener = []
        opener.append("typedef" if self.is_typedef else '')
        opener.append(self.type)
        opener.append(self.name + ('' if self.is_typedef else ';') if self.name else '')
        opener.append("{" if len(self.members) else '')
        opener.append(f"//{self.line}" if self.line else '')

        result.append(" ".join(x for x in opener if x))
            
        # Group members by line number to handle multiple declarations on same line
        if self.type == 'enum':
            for name, value in self.members:
                if value is not None:
                    result.append(f"\t{name} = {value},")
                else:
                    result.append(f"\t{name},")
        else:
            by_line = defaultdict(list)
            for m in self.members:
                by_line[m.line].append(m)
                
            for line, members in sorted(by_line.items()):
                member_str = ", ".join([m.to_decl_str() for m in members])
                result.append(f"\t{member_str};" + (f" //{line}" if line else ""))
        
        closer = []
        if len(self.members):
            closer.append("}" if self.is_typedef else '};')
        closer.append(self.typedef_name + ';' if self.is_typedef else '')
        closer.append(f"//{self.typedef_line}" if self.typedef_line else '')    

        result.append(" ".join(x for x in closer if x))

        return '\n'.join(result) if len(result) > 2 else ' '.join(result)

class Class(Compound):
    def __init__(self, name, line=None, parent=None):
        super().__init__(name, "class", line, parent)
        self.subprograms = []
        
    def add_subprogram(self, subprogram):
        self.subprograms.append(subprogram)
        
    def __str__(self):
        result = []
        result.append(f"class {self.name} {{" + (f" //{self.line}" if self.line else ""))
        
        # Group members by accessibility
        public_members = [m for m in self.members if m.accessibility == "public"]
        private_members = [m for m in self.members if m.accessibility != "public"]
        
        if private_members:
            result.append("private:")
            # Group by line number
            by_line = defaultdict(list)
            for m in private_members:
                by_line[m.line].append(m)
                
            for line, members in sorted(by_line.items()):
                member_str = ", ".join([m.to_decl_str() for m in members])
                result.append(f"\t{member_str};" + (f" //{line}" if line else ""))
        
        if public_members:
            result.append("public:")
            # Group by line number
            by_line = defaultdict(list)
            for m in public_members:
                by_line[m.line].append(m)
                
            for line, members in sorted(by_line.items()):
                member_str = ", ".join([m.to_decl_str() for m in members])
                result.append(f"\t{member_str};" + (f" //{line}" if line else ""))
        
        # Add methods
        for subprogram in self.subprograms:
            indented = '\n'.join(f"\t{line}" for line in str(subprogram).split('\n'))
            result.append(indented)
            
        result.append("};")
        return '\n'.join(result)

class Variable(Element):
    def __init__(self, name, var_type, line=None, parent=None, accessibility=None, is_extern=False, array_sizes=None, pointer=0):
        super().__init__(name, line, parent)
        self.var_type = var_type
        self.accessibility = accessibility
        self.is_extern = is_extern
        self.array_sizes = array_sizes
        self.pointer = pointer
        
    def to_decl_str(self):
        return self._name_prefix() + " " + self._name() + self._name_suffix()

    def _name_prefix(self):
        prefix = ""
        if self.is_extern:
            prefix = "extern "
        return prefix + self.var_type

    def _name_suffix(self):
        return ''.join([f'[{x}]' for x in self.array_sizes or []])

    def _name(self):
        return '*' * self.pointer + self.name
        
    def __str__(self):
        return f"{self.to_decl_str()};" + (f" //{self.line}" if self.line else "")

class FormalParameter(Variable):
    def __init__(self, name, var_type, line=None, parent=None, pointer=False):
        super().__init__(name, var_type, line, parent, pointer=pointer)
        
    def __str__(self):
        return self.to_decl_str() + (f" //{self.line}" if self.line else "")

class Subprogram(Element):
    def __init__(self, name, return_type, line=None, parent=None, is_method=False, class_name=None, accessibility=None, is_inline=False):
        super().__init__(name, line, parent)
        self.return_type = return_type
        self.parameters = []
        self.variables = []
        self.body_elements = []
        self.is_method = is_method
        self.class_name = class_name
        self.accessibility = accessibility
        self.is_inline = is_inline
        self.is_pointer_return = False
        
    def add_parameter(self, param):
        self.parameters.append(param)
        
    def add_variable(self, var):
        self.variables.append(var)
        
    def add_body_element(self, element):
        self.body_elements.append(element)
        
    def __str__(self):
        result = []
        
        # Handle return type
        ret_str = self.return_type
        if self.is_pointer_return:
            ret_str = f"{ret_str}*"
            
        # Function declaration
        if self.is_method:
            prefix = ""
            if self.accessibility:
                prefix = f"{self.accessibility}: "
                
            func_decl = f"{prefix}{ret_str} {self.name}"
        else:
            func_decl = f"{ret_str} {self.name}"
            
        # Parameters
        if not self.parameters:
            result.append(f"{func_decl}()" + (f" //{self.line}" if self.line else ""))
        else:
            # Group parameters by line
            params_by_line = defaultdict(list)
            for p in self.parameters:
                # Skip 'this' parameter for methods
                if self.is_method and p.name == "this":
                    continue
                params_by_line[p.line].append(p)
                
            if len(params_by_line) == 1 and list(params_by_line.keys())[0] == self.line:
                # All params on same line as function
                param_str = ", ".join([p.to_decl_str() for line, params in params_by_line.items() for p in params])
                result.append(f"{func_decl}({param_str})" + (f" //{self.line}" if self.line else ""))
            else:
                # Params on different lines
                result.append(f"{func_decl}(" + (f" //{self.line}" if self.line else ""))
                
                # Sort by line number
                lines = sorted(params_by_line.keys())
                for i, line in enumerate(lines):
                    params = params_by_line[line]
                    param_str = ", ".join([p.to_decl_str() for p in params])
                    
                    if i == len(lines) - 1:  # Last parameter
                        result.append(f"\t\t\t\t{param_str})" + (f" //{line}" if line else ""))
                    else:
                        result.append(f"\t\t\t\t{param_str}," + (f" //{line}" if line else ""))
        
        # Function body
        if not self.body_elements and not self.variables:
            result.append(";")  # Function declaration only
        else:
            result.append("{")
            
            # Variables first
            vars_by_line = defaultdict(list)
            for v in self.variables:
                vars_by_line[v.line].append(v)
                
            for line, vars_list in sorted(vars_by_line.items()):
                var_str = ", ".join([v.to_decl_str() for v in vars_list])
                result.append(f"\t{var_str};" + (f" //{line}" if line else ""))
                
            # Other body elements (lexical blocks, inlined functions)
            for elem in self.body_elements:
                if isinstance(elem, LexicalBlock):
                    block_str = str(elem)
                    indented = '\n'.join(f"\t{line}" for line in block_str.split('\n'))
                    result.append(indented)
                elif isinstance(elem, InlinedSubroutine):
                    result.append(f"\t{elem.name}();" + (f" //{elem.line}" if elem.line else ""))
                    
            result.append("}")
            
        return '\n'.join(result)

class InlinedSubroutine(Element):
    def __init__(self, name, line=None, parent=None):
        super().__init__(name, line, parent)

class LexicalBlock(Element):
    def __init__(self, line=None, parent=None):
        super().__init__(None, line, parent)
        self.variables = []
        self.nested_blocks = []
        
    def add_variable(self, var):
        self.variables.append(var)
        
    def add_nested_block(self, block):
        self.nested_blocks.append(block)
        
    def __str__(self):
        result = ["{"]
        
        # Group variables by line
        vars_by_line = defaultdict(list)
        for v in self.variables:
            vars_by_line[v.line].append(v)
            
        for line, vars_list in sorted(vars_by_line.items()):
            var_str = ", ".join([v.to_decl_str() for v in vars_list])
            result.append(f"\t{var_str};" + (f" //{line}" if line else ""))
            
        # Nested blocks
        for block in self.nested_blocks:
            block_str = str(block)
            indented = '\n'.join(f"\t{line}" for line in block_str.split('\n'))
            result.append(indented)
            
        result.append("}")
        return '\n'.join(result)

class DwarfParser:
    def __init__(self, elf_file_path):
        self.elf_file_path = elf_file_path
        self.files = {}  # Map file paths to File objects
        self.die_cache = {}  # Cache for DIEs referenced by offset
        self.typedefs = {}  # Cache for typedefs
        
    def parse(self):
        with open(self.elf_file_path, 'rb') as f:
            elffile = ELFFile(f)
            
            if not elffile.has_dwarf_info():
                print("File has no DWARF info", file=sys.stderr)
                return
                
            dwarfinfo = elffile.get_dwarf_info()
            
            # First pass: collect all DIEs and create the cache
            for cu_idx, cu in enumerate(dwarfinfo.iter_CUs()):
                top_die = cu.get_top_DIE()
                self._process_die_for_cache(top_die, dwarfinfo)
                
            # Second pass: build the element tree
            for cu_idx, cu in enumerate(dwarfinfo.iter_CUs()):
                top_die = cu.get_top_DIE()
                
                # Get file path for this CU
                file_path = top_die.get_full_path()
                if not file_path:
                    # Use CU name if available
                    file_path = self._get_attribute_value(top_die, 'DW_AT_name', '')
                    
                if file_path not in self.files:
                    self.files[file_path] = File(file_path)
                    
                # Process compile unit children
                self._process_die(top_die, self.files[file_path], dwarfinfo)
                
        return self.files
    
    def _process_die_for_cache(self, die, dwarfinfo, parent_die=None):
        self.die_cache[die.offset] = die
        
        # Check for typedef
        if die.tag == 'DW_TAG_typedef':
            name = self._get_attribute_value(die, 'DW_AT_name', None)
            line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
            if name:
                type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
                if type_ref:
                    self.typedefs[type_ref.offset] = (name, line)
        
        # Process children recursively
        for child in die.iter_children():
            self._process_die_for_cache(child, dwarfinfo, die)
    
    def _process_die(self, die, parent_element, dwarfinfo):
        if die.tag == 'DW_TAG_namespace':
            self._process_namespace(die, parent_element, dwarfinfo)
        elif die.tag == 'DW_TAG_structure_type':
            self._process_compound(die, "struct", parent_element, dwarfinfo)
        elif die.tag == 'DW_TAG_class_type':
            self._process_class(die, parent_element, dwarfinfo)
        elif die.tag == 'DW_TAG_enumeration_type':
            self._process_compound(die, "enum", parent_element, dwarfinfo)
        elif die.tag == 'DW_TAG_union_type':
            self._process_compound(die, "union", parent_element, dwarfinfo)
        elif die.tag == 'DW_TAG_variable' and isinstance(parent_element, File):
            self._process_global_variable(die, parent_element, dwarfinfo)
        elif die.tag == 'DW_TAG_subprogram':
            self._process_subprogram(die, parent_element, dwarfinfo)
        elif die.tag == 'DW_TAG_subroutine_type':
            pass
        elif die.tag in ('DW_TAG_base_type', 'DW_TAG_typedef', 'DW_TAG_pointer_type', 'DW_TAG_array_type', 'DW_TAG_subrange_type', 'DW_TAG_const_type', 'DW_TAG_formal_parameter'):
            pass
        elif die.tag == 'DW_TAG_compile_unit':    
            for child in die.iter_children():
                self._process_die(child, parent_element, dwarfinfo)
        else:
            print("Unhandled die: " + die.tag)

    def _process_namespace(self, die, parent_element, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        if not name:
            return
            
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        # Get the file this namespace is in
        file_element = parent_element
        while not isinstance(file_element, File):
            file_element = file_element.parent
            
        # Use the namespace path as a key
        if isinstance(parent_element, Namespace):
            ns_path = tuple(parent_element.get_path() + [name])
        else:
            ns_path = (name,)
            
        # Create or reuse the namespace
        if ns_path in file_element.namespaces:
            namespace = file_element.namespaces[ns_path]
        else:
            namespace = Namespace(name, line, parent_element)
            file_element.namespaces[ns_path] = namespace
            parent_element.add_child(namespace)
            
        # Process children in the namespace context
        for child in die.iter_children():
            self._process_die(child, namespace, dwarfinfo)
    
    def _process_class(self, die, parent_element, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        class_element = Class(name, line, parent_element)
        parent_element.add_child(class_element)
        
        # Process members and methods
        for child in die.iter_children():
            if child.tag == 'DW_TAG_member':
                self._process_member(child, class_element, dwarfinfo)
            elif child.tag == 'DW_TAG_subprogram':
                self._process_class_subprogram(child, class_element, dwarfinfo)
                
        # Check for subprogram implementations outside class
        self._find_class_method_implementations(class_element, dwarfinfo)
    
    def _process_compound(self, die, type, parent_element, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        # Check if this is typedefed
        is_typedef = False
        typedef_name = None
        typedef_line = None
        
        if die.offset in self.typedefs:
            is_typedef = True
            typedef_name, typedef_line = self.typedefs[die.offset]

        compound = Compound(name, type, line, parent_element, is_typedef, typedef_name, typedef_line)
        parent_element.add_child(compound)
        
        # Process members
        for child in die.iter_children():
            if child.tag == 'DW_TAG_member':
                self._process_member(child, compound, dwarfinfo)
            elif child.tag == 'DW_TAG_enumerator':
                name = self._get_attribute_value(child, 'DW_AT_name', None)
                value = self._get_attribute_value(child, 'DW_AT_const_value', None)
                if name:
                    compound.add_value(name, value)
    
    def _process_member(self, die, parent_element, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        if not name:
            return
            
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        # Get type information
        type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
        var_type = "void"
        pointer = 0
        array_sizes = None
        
        if type_ref:
            var_type, pointer, array_sizes = self._resolve_type(type_ref, dwarfinfo)
            
        # Get accessibility for class members
        accessibility = None
        if isinstance(parent_element, Class):
            accessibility = self._get_attribute_value(die, 'DW_AT_accessibility', "private")
            
        var = Variable(name, var_type, line, parent_element, accessibility, pointer=pointer, array_sizes=array_sizes)
        parent_element.add_member(var)
    
    def _process_global_variable(self, die, parent_element, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        if not name:
            return
            
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        # Get type information
        type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
        var_type = "void"
        pointer = 0
        array_sizes = None
        
        if type_ref:
            var_type, pointer, array_sizes = self._resolve_type(type_ref, dwarfinfo)
            
        # Check for extern/static
        is_extern = self._get_attribute_value(die, 'DW_AT_external', False)
        
        var = Variable(name, var_type, line, parent_element, is_extern=is_extern, 
                      pointer=pointer, array_sizes=array_sizes)
        parent_element.add_child(var)
    
    def _process_subprogram(self, die, parent_element, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        if not name:
            return
            
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        # Skip if this is just a declaration (handled elsewhere)
        if self._get_attribute_value(die, 'DW_AT_declaration', False):
            # Check if this is a class method declaration
            if isinstance(parent_element, Class):
                # We'll handle this later with _find_class_method_implementations
                pass
            return
            
        # Get return type
        return_type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
        return_type = "void"
        pointer = 0
        
        if return_type_ref:
            return_type, pointer, _ = self._resolve_type(return_type_ref, dwarfinfo)
            
        # Check if this is a method implementation
        spec_ref = self._get_attribute_value(die, 'DW_AT_specification', None)
        is_method = False
        class_name = None
        accessibility = None
        
        if spec_ref:
            spec_die = die.get_DIE_from_attribute('DW_AT_specification')
            if spec_die and spec_die.get_parent() and spec_die.get_parent().tag == 'DW_TAG_class_type':
                is_method = True
                class_name = self._get_attribute_value(spec_die.get_parent(), 'DW_AT_name', None)
                accessibility = self._get_attribute_value(spec_die, 'DW_AT_accessibility', "private")
        
        # Check if inline
        is_inline = self._get_attribute_value(die, 'DW_AT_inline', False)
        
        subprogram = Subprogram(name, return_type, line, parent_element, is_method, class_name, accessibility, is_inline)
        subprogram.is_pointer_return = pointer > 0
        parent_element.add_child(subprogram)
        
        # Process parameters
        for child in die.iter_children():
            if child.tag == 'DW_TAG_formal_parameter':
                self._process_parameter(child, subprogram, dwarfinfo)
            elif child.tag == 'DW_TAG_variable':
                self._process_local_variable(child, subprogram, dwarfinfo)
            elif child.tag == 'DW_TAG_lexical_block':
                self._process_lexical_block(child, subprogram, dwarfinfo)
            elif child.tag == 'DW_TAG_inlined_subroutine':
                self._process_inlined_subroutine(child, subprogram, dwarfinfo)
            else:
                print(name)
                print("Unhandled member: " + child.tag)
    
    def _process_class_subprogram(self, die, class_element, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        if not name:
            return
            
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        # Get return type
        return_type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
        return_type = "void"
        pointer = 0
        
        if return_type_ref:
            return_type, pointer, _ = self._resolve_type(return_type_ref, dwarfinfo)
            
        # Get accessibility
        accessibility = self._get_attribute_value(die, 'DW_AT_accessibility', "private")
        
        subprogram = Subprogram(name, return_type, line, class_element, True, class_element.name, accessibility)
        subprogram.is_pointer_return = pointer > 0
        class_element.add_subprogram(subprogram)
        
        # Process parameters (potentially just declaration)
        for child in die.iter_children():
            if child.tag == 'DW_TAG_formal_parameter':
                self._process_parameter(child, subprogram, dwarfinfo)
    
    def _process_parameter(self, die, subprogram, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        if not name:
            return
            
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        # Get type information
        type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
        var_type = "void"
        pointer = 0
        
        if type_ref:
            var_type, pointer, _ = self._resolve_type(type_ref, dwarfinfo)
            
        param = FormalParameter(name, var_type, line, subprogram, pointer)
        subprogram.add_parameter(param)

    def _process_local_variable(self, die, parent, dwarfinfo):
        name = self._get_attribute_value(die, 'DW_AT_name', None)
        if not name:
            return
            
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        # Get type information
        type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
        var_type = "void"
        pointer = 0
        array_sizes = None
        
        if type_ref:
            var_type, pointer, array_sizes = self._resolve_type(type_ref, dwarfinfo)
            
        var = Variable(name, var_type, line, parent, pointer=pointer, array_sizes=array_sizes)
        
        if isinstance(parent, Subprogram):
            parent.add_variable(var)
        elif isinstance(parent, LexicalBlock):
            parent.add_variable(var)
    
    def _process_lexical_block(self, die, parent, dwarfinfo):
        line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
        
        block = LexicalBlock(line, parent)
        
        if isinstance(parent, Subprogram):
            parent.add_body_element(block)
        elif isinstance(parent, LexicalBlock):
            parent.add_nested_block(block)
            
        # Process block contents
        for child in die.iter_children():
            if child.tag == 'DW_TAG_variable':
                self._process_local_variable(child, block, dwarfinfo)
            elif child.tag == 'DW_TAG_lexical_block':
                self._process_lexical_block(child, block, dwarfinfo)
            elif child.tag == 'DW_TAG_inlined_subroutine':
                self._process_inlined_subroutine(child, block, dwarfinfo)
    
    def _process_inlined_subroutine(self, die, parent, dwarfinfo):
        # Get the abstract origin to find the actual function info
        origin_ref = self._get_attribute_value(die, 'DW_AT_abstract_origin', None)
        
        name = None
        if origin_ref:
            origin_die = die.get_DIE_from_attribute('DW_AT_abstract_origin')
            if origin_die:
                name = self._get_attribute_value(origin_die, 'DW_AT_name', None)
                
        if not name:
            return
            
        line = self._get_attribute_value(die, 'DW_AT_call_line', None)
        if not line:
            line = self._get_attribute_value(die, 'DW_AT_decl_line', None)
            
        inlined = InlinedSubroutine(name, line, parent)
        
        if isinstance(parent, Subprogram):
            parent.add_body_element(inlined)
        elif isinstance(parent, LexicalBlock):
            # Not currently handling this, but could add to block
            pass
    
    def _find_class_method_implementations(self, class_element, dwarfinfo):
        # This would need to scan all subprograms to find those that
        # reference this class's method declarations via DW_AT_specification
        pass
    
    def _resolve_type(self, type_ref, dwarfinfo, pointer=0, array_sizes=None):
        """Resolve a type reference to a type name."""
        die = type_ref
        if array_sizes is None:
            array_sizes = []
        
        # Handle various type tags
        if die.tag == 'DW_TAG_pointer_type':
            # Get the type this points to
            pointer += 1
            pointed_type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
            if pointed_type_ref:
                return self._resolve_type(pointed_type_ref, dwarfinfo, pointer, array_sizes)
            return "void", pointer, array_sizes
            
        elif die.tag == 'DW_TAG_array_type':
            # Get base type
            base_type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
            base_type = "void"
            
            if base_type_ref:
                base_type, pointer, array_sizes = self._resolve_type(base_type_ref, dwarfinfo, pointer, array_sizes)
                
            # Get array size
            for child in die.iter_children():
                if child.tag == 'DW_TAG_subrange_type':
                    size = None
                    if 'DW_AT_upper_bound' in child.attributes:
                        size = self._get_attribute_value(child, 'DW_AT_upper_bound', 0) + 1
                    elif 'DW_AT_count' in child.attributes:
                        size = self._get_attribute_value(child, 'DW_AT_count', 0)
                    if size is not None:
                        array_sizes.append(size)

            return base_type, pointer, array_sizes
            
        elif die.tag == 'DW_TAG_base_type':
            return self._get_attribute_value(die, 'DW_AT_name', "void"), pointer, array_sizes
            
        elif die.tag == 'DW_TAG_typedef':
            return self._get_attribute_value(die, 'DW_AT_name', "void"), pointer, array_sizes
            
        elif die.tag in ['DW_TAG_structure_type', 'DW_TAG_class_type', 'DW_TAG_union_type', 'DW_TAG_enumeration_type']:
            name = self._get_attribute_value(die, 'DW_AT_name', None)
            if name:
                prefix = ""
                if die.tag == 'DW_TAG_structure_type':
                    prefix = "struct "
                elif die.tag == 'DW_TAG_class_type':
                    prefix = "class "
                elif die.tag == 'DW_TAG_union_type':
                    prefix = "union "
                elif die.tag == 'DW_TAG_enumeration_type':
                    prefix = "enum "
                    
                return f"{prefix}{name}", pointer, array_sizes
            
            # Anonymous struct/union/enum
            if die.offset in self.typedefs:
                return self.typedefs[die.offset][0], pointer, array_sizes
                
            return "void", pointer, array_sizes
            
        elif die.tag == 'DW_TAG_const_type':
            const_type_ref = self._get_attribute_value(die, 'DW_AT_type', None)
            if const_type_ref:
                base_type, pointer, array_sizes = self._resolve_type(const_type_ref, dwarfinfo, pointer, array_sizes)
                base_type = "const " + base_type
                return base_type, pointer, array_sizes
            return "void", pointer, array_sizes
            
        elif die.tag == 'DW_TAG_subroutine_type':
            # Function pointer type - simplify for now
            return "void(*)()", True, None
            
        else:
            print("Unhandled type " + die.tag)
            # Unhandled type
            return "void", pointer, array_sizes
    
    def _get_attribute_value(self, die, attr_name, default=None):
        """Extract an attribute value from a DIE."""
        if not die:
            return default

        if attr_name == "DW_AT_type" and 'DW_AT_type' in die.attributes:
            return die.get_DIE_from_attribute('DW_AT_type')
            
        try:
            attr_value = die.attributes[attr_name].value
            if isinstance(attr_value, int):
                return attr_value
            else:
                return attr_value.decode()
        except (KeyError, AttributeError):
            return default

def process_file(file_path):
    parser = DwarfParser(file_path)
    files = parser.parse()
    
    res = ''
    # Sort files by path for consistent output
    for file_path in sorted(files.keys()):
        res += str(files[file_path])

    return res

if __name__ == "__main__":
    # if len(sys.argv) != 2:
    #     print(f"Usage: {sys.argv[0]} <elf_file>", file=sys.stderr)
    #     sys.exit(1)
    # process_file(sys.argv[1])
        
    res = process_file("mp-x86.so")
    with open("out.cpp", "w") as f:
        f.write(res)