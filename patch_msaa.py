import re

with open("src/scene/pipeline/mod.rs", "r") as f:
    content = f.read()

# 1. Replace the const definition with a function
# Find: const MSAA_SAMPLES: u32 = 4;
func_def = """pub(crate) fn msaa_samples() -> u32 {
    let prefs = crate::gpu_backend::load_prefs();
    if prefs.msaa > 0 {
        return prefs.msaa;
    }
    let status = gpu_status();
    if prefs.compat_renderer || status.identity().is_some_and(|id| id.contains("BYT") || id.contains("llvmpipe")) {
        1
    } else {
        4
    }
}
"""
content = re.sub(r'const MSAA_SAMPLES: u32 = 4;', func_def, content)

# 2. Fix BYTES_PER_PIXEL calculation
old_bytes = r'const BYTES_PER_PIXEL: u64 = \(MSAA_SAMPLES as u64\) \* 4 \+ \(MSAA_SAMPLES as u64\) \* 4 \+ 4;'
new_bytes = r'let msaa = msaa_samples() as u64;\n        let bytes_per_pixel = msaa * 4 + msaa * 4 + 4;'
content = re.sub(old_bytes, new_bytes, content)

# 3. Fix BYTES_PER_PIXEL usage
content = re.sub(r'pixels \* BYTES_PER_PIXEL', r'pixels * bytes_per_pixel', content)

# 4. Replace other MSAA_SAMPLES with msaa_samples()
content = re.sub(r'MSAA_SAMPLES', r'msaa_samples()', content)

with open("src/scene/pipeline/mod.rs", "w") as f:
    f.write(content)

with open("src/scene/pipeline/hatch_gpu/mod.rs", "r") as f:
    hatch = f.read()
hatch = re.sub(r'super::MSAA_SAMPLES', r'super::msaa_samples()', hatch)
with open("src/scene/pipeline/hatch_gpu/mod.rs", "w") as f:
    f.write(hatch)

