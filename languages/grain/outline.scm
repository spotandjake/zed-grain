; Document outline for Grain (Zed). Captures: @name, @item.

(module_header
  name: (upper_identifier) @name) @item

(program
  (module_declaration
    name: (upper_identifier) @name) @item)

(module_declaration
  name: (_)
  (module_declaration
    name: (upper_identifier) @name) @item)

(program
  (let_declaration
    (value_bindings
      (value_binding
        pattern: [
          (variable_pattern) @name
          (typed_pattern
            (variable_pattern) @name)
          (alias_pattern
            _
            [
              (identifier) @name
              (special_identifier) @name
            ])
        ]))) @item)

(module_declaration
  name: (_)
  (let_declaration
    (value_bindings
      (value_binding
        pattern: [
          (variable_pattern) @name
          (typed_pattern
            (variable_pattern) @name)
          (alias_pattern
            _
            [
              (identifier) @name
              (special_identifier) @name
            ])
        ]))) @item)

(program
  (provide_declaration
    (value_bindings
      (value_binding
        pattern: [
          (variable_pattern) @name
          (typed_pattern
            (variable_pattern) @name)
          (alias_pattern
            _
            [
              (identifier) @name
              (special_identifier) @name
            ])
        ]))) @item)

(module_declaration
  name: (_)
  (provide_declaration
    (value_bindings
      (value_binding
        pattern: [
          (variable_pattern) @name
          (typed_pattern
            (variable_pattern) @name)
          (alias_pattern
            _
            [
              (identifier) @name
              (special_identifier) @name
            ])
        ]))) @item)

(program
  (data_declaration_statement
    (type_alias
      name: (upper_identifier) @name) @item))

(module_declaration
  name: (_)
  (data_declaration_statement
    (type_alias
      name: (upper_identifier) @name) @item))

(program
  (data_declaration_statement
    (enum_declaration
      name: (upper_identifier) @name) @item))

(module_declaration
  name: (_)
  (data_declaration_statement
    (enum_declaration
      name: (upper_identifier) @name) @item))

(program
  (data_declaration_statement
    (record_type_declaration
      name: (upper_identifier) @name) @item))

(module_declaration
  name: (_)
  (data_declaration_statement
    (record_type_declaration
      name: (upper_identifier) @name) @item))

(enum_declaration
  name: (_)
  (enum_body
    (enum_variant
      name: (upper_identifier) @name) @item))

(record_type_declaration
  name: (_)
  (record_declaration_body
    (record_field_declaration
      name: (_) @name) @item))

(program
  (exception_declaration
    name: (upper_identifier) @name) @item)

(module_declaration
  name: (_)
  (exception_declaration
    name: (upper_identifier) @name) @item)

(program
  (primitive_declaration
    name: (_) @name) @item)

(module_declaration
  name: (_)
  (primitive_declaration
    name: (_) @name) @item)
