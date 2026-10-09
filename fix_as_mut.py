import re

with open('src/app/update/viewport.rs', 'r') as f:
    lines = f.readlines()

out = []
i = 0
while i < len(lines):
    line = lines[i]
    m = re.search(r'if let Some\(s\) = self\.tabs\[(.)\]\.snap_result\.get\(\)\.as_mut\(\)', line)
    if m:
        idx = m.group(1)
        out.append(line.replace('Some(s) =', 'Some(mut s) =').replace('.get().as_mut()', '.get()'))
        i += 1
        # Read until closing brace
        while i < len(lines):
            l2 = lines[i]
            if l2.strip() == '}':
                # Insert the set before closing brace
                indent = ' ' * (len(l2) - len(l2.lstrip()) + 4)
                out.append(f'{indent}self.tabs[{idx}].snap_result.set(Some(s));\n')
                out.append(l2)
                i += 1
                break
            else:
                out.append(l2)
                i += 1
    else:
        out.append(line)
        i += 1

with open('src/app/update/viewport.rs', 'w') as f:
    f.writelines(out)
