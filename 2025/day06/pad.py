import functools

def pad_lines(text: str) -> str:
    lines = text.splitlines()
    width = max(len(line) for line in lines)
    return '\n'.join(line.ljust(width) for line in lines)

def flip_input(text: str) -> str:
    lines = text.splitlines()
    rotated = '\n'.join(''.join(vert).strip() for vert in zip(*lines))
    return rotated

if __name__ == "__main__":
    text = """123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"""

    lines = text.splitlines()
    lines.insert(-1, '')
    expanded_text = '\n'.join(lines)
    padded = pad_lines(expanded_text)

    rotated = flip_input(pad_lines(expanded_text))
    blocks = rotated.split('\n\n')
    total = 0
    for block in blocks:
        numbers = [int(s) for s in block.split() if s.isdigit()]
        print(f"Block:\n{block}\nNumbers: {numbers}")
        if '*' in block:
            total += functools.reduce(lambda x, y: x * y, numbers, 1)
        elif '+' in block:
            total += sum(numbers)
    print(f"Total: {total}")
