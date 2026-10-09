import os, sys

def process_file(path):
    with open(path, 'r') as f:
        content = f.read()

    props = ['snap_result', 'last_cursor_world', 'last_cursor_screen']
    
    # We will do this in multiple passes.
    # Pass 1: Replace assignments: .prop = expr; -> .prop.set(expr);
    for prop in props:
        target = f'.{prop}'
        idx = 0
        while True:
            # Find '.prop =' or '.prop  ='
            # Let's search for '.prop'
            idx = content.find(target, idx)
            if idx == -1: break
            
            # check if it's an assignment
            after_prop = idx + len(target)
            
            # skip whitespaces
            j = after_prop
            while j < len(content) and content[j] in ' \t\n\r':
                j += 1
                
            if j < len(content) and content[j] == '=' and (j+1 == len(content) or content[j+1] != '='):
                # found assignment!
                start_expr = j + 1
                # find the matching semicolon at top level
                k = start_expr
                nesting = 0
                while k < len(content):
                    c = content[k]
                    if c in '({[': nesting += 1
                    elif c in ')}]': nesting -= 1
                    elif c == ';' and nesting == 0:
                        break
                    k += 1
                
                if k < len(content):
                    expr = content[start_expr:k]
                    # replace!
                    replacement = f'{target}.set({expr})'
                    content = content[:idx] + replacement + content[k:]
                    idx = idx + len(replacement)
                else:
                    idx = after_prop
            else:
                idx = after_prop

    # Pass 2: Replace reads: .prop -> .prop.get()
    # But ONLY if not already .prop.set(...) or .prop.get(...) or .prop (as part of something else)
    for prop in props:
        target = f'.{prop}'
        idx = 0
        while True:
            idx = content.find(target, idx)
            if idx == -1: break
            
            after_prop = idx + len(target)
            if after_prop < len(content) and (content[after_prop].isalnum() or content[after_prop] == '_'):
                idx = after_prop
                continue
                
            # skip whitespaces
            j = after_prop
            while j < len(content) and content[j] in ' \t\n\r':
                j += 1
            
            if content[j:j+4] == '.set' or content[j:j+4] == '.get':
                idx = after_prop
                continue
                
            # It's a read! Replace .prop with .prop.get()
            content = content[:after_prop] + '.get()' + content[after_prop:]
            idx = after_prop + 6

    with open(path, 'w') as f:
        f.write(content)

for root, dirs, files in os.walk('src'):
    for f in files:
        if f.endswith('.rs'):
            process_file(os.path.join(root, f))
