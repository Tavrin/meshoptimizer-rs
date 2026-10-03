# Measured geometry performance

Single-thread RFC 6.1 matrix. One warm-up and at least twenty alternating same-core paired samples per case.
Coordinator correction: measure immediately on a core selected from three one-second /proc/stat intervals. Ratios are medians of paired Rust/C++ samples; load does not gate or repeat measurements.
Pinned CPU: 20; physical core 10; siblings 20-21; selected logical CPU busy 69.83%.
Raw paired ratios, IQR, standard deviation, median absolute deviation, coefficient of variation and per-pair loads are retained in `results/benchmark.json`.


| Family | Cases | Geometric mean | Maximum | Maximum memory ratio | Verdict |
|---|---:|---:|---:|---:|---|
| vertex_cache | 24 | 1.123 | 1.201 | 1.000 | PASS |
| overdraw | 24 | 1.098 | 1.417 | 0.999 | PASS |
| simplify | 72 | 1.212 | 1.385 | 1.095 | PASS |
| simplify_with_attributes | 72 | 1.208 | 1.342 | 1.084 | PASS |
| simplify_scale | 12 | 0.748 | 0.884 | 1.000 | PASS |

Overall: **PASS**. Required: geometric mean ≤1.25, every case ≤1.50, memory ≤1.25.
Load average start: (41.271484375, 27.763671875, 25.8037109375); end: (37.43359375, 45.12744140625, 32.6103515625).

Scale has one allocation-free API, so it is measured once per geometry. Caller-buffer measurements reuse output and Workspace after warm-up; allocating measurements use fresh scratch.
Seam-heavy meshes have duplicate patch boundaries and discontinuous attributes; disconnected meshes have separated patches; sparse meshes reference half the supplied vertices.
Attribute counts 1, 8 and 12 are paired with target ratios 0.5, 0.25 and 0.125; plain simplification covers all three ratios on every geometry.
Raw samples, dispersion, per-attempt load, identities and per-case memory are in `results/benchmark.json`.

| Case | Rust ms | C++ ms | Paired time ratio | Ratio IQR | Ratio CV | Memory ratio |
|---|---:|---:|---:|---|---:|---:|
| vertex_cache/tiny/smooth/allocating/a0/rNone | 0.004321 | 0.003652 | 1.173 | 1.139–1.227 | 0.750 | 1.000 |
| vertex_cache/tiny/smooth/caller-buffer/a0/rNone | 0.004308 | 0.003860 | 1.101 | 1.001–1.150 | 0.352 | 1.000 |
| overdraw/tiny/smooth/allocating/a0/rNone | 0.003300 | 0.003311 | 1.003 | 0.972–1.049 | 1.667 | 0.995 |
| overdraw/tiny/smooth/caller-buffer/a0/rNone | 0.002996 | 0.003347 | 0.895 | 0.874–0.929 | 0.078 | 0.989 |
| simplify_scale/tiny/smooth/allocation-free/a0/rNone | 0.000171 | 0.000196 | 0.884 | 0.826–0.920 | 0.068 | 1.000 |
| simplify/tiny/smooth/allocating/a0/r0.5 | 0.022598 | 0.019347 | 1.170 | 1.154–1.193 | 0.094 | 1.052 |
| simplify/tiny/smooth/caller-buffer/a0/r0.5 | 0.016269 | 0.014275 | 1.142 | 1.131–1.166 | 0.036 | 1.057 |
| simplify/tiny/smooth/allocating/a0/r0.25 | 0.011430 | 0.010517 | 1.082 | 0.955–1.101 | 0.195 | 1.052 |
| simplify/tiny/smooth/caller-buffer/a0/r0.25 | 0.011802 | 0.010758 | 1.105 | 1.091–1.109 | 0.079 | 1.057 |
| simplify/tiny/smooth/allocating/a0/r0.125 | 0.014406 | 0.012286 | 1.122 | 1.091–1.182 | 0.063 | 1.052 |
| simplify/tiny/smooth/caller-buffer/a0/r0.125 | 0.012034 | 0.011265 | 1.110 | 0.912–1.132 | 0.153 | 1.057 |
| simplify_with_attributes/tiny/smooth/allocating/a1/r0.5 | 0.021261 | 0.019252 | 1.133 | 1.087–1.164 | 0.095 | 1.055 |
| simplify_with_attributes/tiny/smooth/caller-buffer/a1/r0.5 | 0.016611 | 0.015135 | 1.132 | 1.019–1.163 | 0.107 | 1.059 |
| simplify_with_attributes/tiny/smooth/allocating/a8/r0.25 | 0.024841 | 0.020854 | 1.195 | 1.182–1.198 | 0.013 | 1.035 |
| simplify_with_attributes/tiny/smooth/caller-buffer/a8/r0.25 | 0.025116 | 0.021146 | 1.186 | 1.173–1.211 | 0.019 | 1.036 |
| simplify_with_attributes/tiny/smooth/allocating/a12/r0.125 | 0.052036 | 0.044302 | 1.176 | 1.157–1.199 | 0.053 | 1.031 |
| simplify_with_attributes/tiny/smooth/caller-buffer/a12/r0.125 | 0.038556 | 0.032902 | 1.171 | 1.158–1.194 | 0.017 | 1.033 |
| vertex_cache/tiny/seam-heavy/allocating/a0/rNone | 0.004181 | 0.003709 | 1.129 | 1.097–1.157 | 0.038 | 1.000 |
| vertex_cache/tiny/seam-heavy/caller-buffer/a0/rNone | 0.003621 | 0.003201 | 1.118 | 1.096–1.152 | 0.057 | 1.000 |
| overdraw/tiny/seam-heavy/allocating/a0/rNone | 0.003491 | 0.003620 | 0.968 | 0.928–0.978 | 0.049 | 0.981 |
| overdraw/tiny/seam-heavy/caller-buffer/a0/rNone | 0.003187 | 0.003359 | 0.969 | 0.938–0.990 | 0.132 | 0.964 |
| simplify_scale/tiny/seam-heavy/allocation-free/a0/rNone | 0.000209 | 0.000268 | 0.789 | 0.760–0.813 | 0.069 | 1.000 |
| simplify/tiny/seam-heavy/allocating/a0/r0.5 | 0.022598 | 0.019176 | 1.182 | 1.160–1.204 | 0.046 | 1.075 |
| simplify/tiny/seam-heavy/caller-buffer/a0/r0.5 | 0.022625 | 0.019355 | 1.162 | 0.951–1.269 | 0.271 | 1.080 |
| simplify/tiny/seam-heavy/allocating/a0/r0.25 | 0.028399 | 0.025798 | 1.093 | 1.068–1.123 | 0.066 | 1.075 |
| simplify/tiny/seam-heavy/caller-buffer/a0/r0.25 | 0.028056 | 0.025661 | 1.114 | 1.064–1.123 | 0.118 | 1.080 |
| simplify/tiny/seam-heavy/allocating/a0/r0.125 | 0.023581 | 0.020247 | 1.135 | 1.048–1.200 | 0.190 | 1.075 |
| simplify/tiny/seam-heavy/caller-buffer/a0/r0.125 | 0.029483 | 0.026730 | 1.131 | 1.065–1.184 | 0.102 | 1.080 |
| simplify_with_attributes/tiny/seam-heavy/allocating/a1/r0.5 | 0.035960 | 0.030029 | 1.192 | 1.169–1.227 | 0.028 | 1.070 |
| simplify_with_attributes/tiny/seam-heavy/caller-buffer/a1/r0.5 | 0.034335 | 0.029692 | 1.164 | 1.154–1.174 | 0.109 | 1.073 |
| simplify_with_attributes/tiny/seam-heavy/allocating/a8/r0.25 | 0.026343 | 0.021566 | 1.227 | 1.210–1.255 | 0.021 | 1.042 |
| simplify_with_attributes/tiny/seam-heavy/caller-buffer/a8/r0.25 | 0.028072 | 0.023335 | 1.211 | 1.202–1.221 | 0.072 | 1.044 |
| simplify_with_attributes/tiny/seam-heavy/allocating/a12/r0.125 | 0.026102 | 0.021123 | 1.240 | 1.213–1.260 | 0.022 | 1.038 |
| simplify_with_attributes/tiny/seam-heavy/caller-buffer/a12/r0.125 | 0.031005 | 0.025563 | 1.218 | 1.178–1.239 | 0.760 | 1.039 |
| vertex_cache/tiny/disconnected/allocating/a0/rNone | 0.004517 | 0.003766 | 1.188 | 1.147–1.245 | 0.120 | 1.000 |
| vertex_cache/tiny/disconnected/caller-buffer/a0/rNone | 0.003861 | 0.003431 | 1.133 | 1.067–1.172 | 0.323 | 1.000 |
| overdraw/tiny/disconnected/allocating/a0/rNone | 0.003406 | 0.003298 | 1.017 | 0.993–1.044 | 0.094 | 0.981 |
| overdraw/tiny/disconnected/caller-buffer/a0/rNone | 0.003183 | 0.003295 | 0.962 | 0.945–0.992 | 0.053 | 0.964 |
| simplify_scale/tiny/disconnected/allocation-free/a0/rNone | 0.000232 | 0.000272 | 0.853 | 0.816–0.873 | 0.070 | 1.000 |
| simplify/tiny/disconnected/allocating/a0/r0.5 | 0.019933 | 0.019121 | 1.229 | 1.210–1.257 | 0.062 | 1.073 |
| simplify/tiny/disconnected/caller-buffer/a0/r0.5 | 0.018356 | 0.015856 | 1.166 | 0.899–1.237 | 0.252 | 1.078 |
| simplify/tiny/disconnected/allocating/a0/r0.25 | 0.026201 | 0.023663 | 1.092 | 1.077–1.147 | 0.097 | 1.073 |
| simplify/tiny/disconnected/caller-buffer/a0/r0.25 | 0.021291 | 0.021246 | 1.128 | 0.976–1.161 | 0.119 | 1.078 |
| simplify/tiny/disconnected/allocating/a0/r0.125 | 0.027392 | 0.024880 | 1.110 | 1.074–1.147 | 0.158 | 1.073 |
| simplify/tiny/disconnected/caller-buffer/a0/r0.125 | 0.037448 | 0.034823 | 1.099 | 1.071–1.163 | 1.135 | 1.078 |
| simplify_with_attributes/tiny/disconnected/allocating/a1/r0.5 | 0.037702 | 0.031602 | 1.182 | 1.142–1.298 | 0.363 | 1.070 |
| simplify_with_attributes/tiny/disconnected/caller-buffer/a1/r0.5 | 0.034208 | 0.028875 | 1.178 | 1.162–1.225 | 0.048 | 1.073 |
| simplify_with_attributes/tiny/disconnected/allocating/a8/r0.25 | 0.035290 | 0.029019 | 1.214 | 1.202–1.237 | 0.068 | 1.042 |
| simplify_with_attributes/tiny/disconnected/caller-buffer/a8/r0.25 | 0.034390 | 0.027979 | 1.234 | 1.225–1.249 | 0.029 | 1.044 |
| simplify_with_attributes/tiny/disconnected/allocating/a12/r0.125 | 0.040963 | 0.033776 | 1.208 | 1.194–1.220 | 0.015 | 1.038 |
| simplify_with_attributes/tiny/disconnected/caller-buffer/a12/r0.125 | 0.039172 | 0.034408 | 1.135 | 1.125–1.146 | 0.014 | 1.039 |
| vertex_cache/tiny/sparse/allocating/a0/rNone | 0.004087 | 0.003505 | 1.176 | 1.142–1.200 | 0.041 | 1.000 |
| vertex_cache/tiny/sparse/caller-buffer/a0/rNone | 0.004042 | 0.003713 | 1.087 | 1.060–1.114 | 0.041 | 1.000 |
| overdraw/tiny/sparse/allocating/a0/rNone | 0.003273 | 0.003380 | 0.988 | 0.965–1.010 | 0.047 | 0.995 |
| overdraw/tiny/sparse/caller-buffer/a0/rNone | 0.002998 | 0.003273 | 0.907 | 0.878–0.939 | 0.048 | 0.991 |
| simplify_scale/tiny/sparse/allocation-free/a0/rNone | 0.000306 | 0.000375 | 0.812 | 0.804–0.825 | 0.069 | 1.000 |
| simplify/tiny/sparse/allocating/a0/r0.5 | 0.022638 | 0.019119 | 1.186 | 1.137–1.224 | 0.088 | 1.071 |
| simplify/tiny/sparse/caller-buffer/a0/r0.5 | 0.014346 | 0.012767 | 1.185 | 1.103–1.202 | 0.083 | 1.076 |
| simplify/tiny/sparse/allocating/a0/r0.25 | 0.027745 | 0.024596 | 1.127 | 1.112–1.140 | 0.051 | 1.071 |
| simplify/tiny/sparse/caller-buffer/a0/r0.25 | 0.026897 | 0.023287 | 1.156 | 1.143–1.173 | 0.052 | 1.076 |
| simplify/tiny/sparse/allocating/a0/r0.125 | 0.022104 | 0.019130 | 1.133 | 1.085–1.199 | 0.079 | 1.071 |
| simplify/tiny/sparse/caller-buffer/a0/r0.125 | 0.019995 | 0.017228 | 1.136 | 1.127–1.159 | 0.045 | 1.076 |
| simplify_with_attributes/tiny/sparse/allocating/a1/r0.5 | 0.026854 | 0.023302 | 1.180 | 1.138–1.265 | 0.080 | 1.068 |
| simplify_with_attributes/tiny/sparse/caller-buffer/a1/r0.5 | 0.016243 | 0.013567 | 1.166 | 1.118–1.195 | 0.105 | 1.071 |
| simplify_with_attributes/tiny/sparse/allocating/a8/r0.25 | 0.030218 | 0.023379 | 1.290 | 1.286–1.296 | 0.009 | 1.039 |
| simplify_with_attributes/tiny/sparse/caller-buffer/a8/r0.25 | 0.032108 | 0.024764 | 1.299 | 1.272–1.312 | 0.039 | 1.040 |
| simplify_with_attributes/tiny/sparse/allocating/a12/r0.125 | 0.019842 | 0.016425 | 1.205 | 1.197–1.210 | 0.018 | 1.035 |
| simplify_with_attributes/tiny/sparse/caller-buffer/a12/r0.125 | 0.029579 | 0.024161 | 1.227 | 1.211–1.240 | 0.024 | 1.036 |
| vertex_cache/medium/smooth/allocating/a0/rNone | 1.179885 | 1.050285 | 1.105 | 1.058–1.242 | 0.162 | 1.000 |
| vertex_cache/medium/smooth/caller-buffer/a0/rNone | 0.866540 | 0.804293 | 1.098 | 1.065–1.143 | 0.113 | 1.000 |
| overdraw/medium/smooth/allocating/a0/rNone | 0.236836 | 0.213542 | 1.109 | 1.088–1.148 | 0.094 | 0.999 |
| overdraw/medium/smooth/caller-buffer/a0/rNone | 0.228309 | 0.217344 | 1.044 | 1.024–1.067 | 0.045 | 0.998 |
| simplify_scale/medium/smooth/allocation-free/a0/rNone | 0.010464 | 0.013725 | 0.783 | 0.736–1.192 | 0.273 | 1.000 |
| simplify/medium/smooth/allocating/a0/r0.5 | 3.247086 | 3.117948 | 1.166 | 1.090–1.221 | 0.217 | 1.059 |
| simplify/medium/smooth/caller-buffer/a0/r0.5 | 2.248559 | 2.052550 | 1.163 | 1.048–1.229 | 0.172 | 1.067 |
| simplify/medium/smooth/allocating/a0/r0.25 | 1.951701 | 2.247151 | 1.143 | 0.926–1.241 | 0.230 | 1.059 |
| simplify/medium/smooth/caller-buffer/a0/r0.25 | 2.644378 | 2.160372 | 1.224 | 1.197–1.283 | 0.107 | 1.067 |
| simplify/medium/smooth/allocating/a0/r0.125 | 2.388952 | 2.030283 | 1.148 | 1.050–1.243 | 0.223 | 1.059 |
| simplify/medium/smooth/caller-buffer/a0/r0.125 | 2.149056 | 1.755817 | 1.199 | 1.118–1.345 | 0.156 | 1.067 |
| simplify_with_attributes/medium/smooth/allocating/a1/r0.5 | 2.190158 | 1.883466 | 1.149 | 1.107–1.233 | 0.127 | 1.060 |
| simplify_with_attributes/medium/smooth/caller-buffer/a1/r0.5 | 2.524913 | 1.886518 | 1.236 | 1.178–1.342 | 0.217 | 1.066 |
| simplify_with_attributes/medium/smooth/allocating/a8/r0.25 | 4.972555 | 3.776066 | 1.277 | 1.180–1.430 | 0.155 | 1.039 |
| simplify_with_attributes/medium/smooth/caller-buffer/a8/r0.25 | 5.150446 | 3.771031 | 1.342 | 1.200–1.430 | 0.256 | 1.041 |
| simplify_with_attributes/medium/smooth/allocating/a12/r0.125 | 7.299762 | 6.210624 | 1.200 | 1.115–1.272 | 0.104 | 1.036 |
| simplify_with_attributes/medium/smooth/caller-buffer/a12/r0.125 | 5.420814 | 4.625288 | 1.165 | 1.094–1.231 | 0.178 | 1.037 |
| vertex_cache/medium/seam-heavy/allocating/a0/rNone | 0.568769 | 0.529166 | 1.056 | 1.017–1.209 | 0.146 | 1.000 |
| vertex_cache/medium/seam-heavy/caller-buffer/a0/rNone | 0.547956 | 0.476436 | 1.147 | 1.138–1.157 | 0.029 | 1.000 |
| overdraw/medium/seam-heavy/allocating/a0/rNone | 0.150754 | 0.139318 | 1.072 | 1.059–1.106 | 0.099 | 0.999 |
| overdraw/medium/seam-heavy/caller-buffer/a0/rNone | 0.150818 | 0.140509 | 1.052 | 1.003–1.096 | 0.098 | 0.997 |
| simplify_scale/medium/seam-heavy/allocation-free/a0/rNone | 0.013160 | 0.017914 | 0.711 | 0.680–1.176 | 0.273 | 1.000 |
| simplify/medium/seam-heavy/allocating/a0/r0.5 | 2.509654 | 2.047756 | 1.191 | 1.170–1.240 | 0.105 | 1.057 |
| simplify/medium/seam-heavy/caller-buffer/a0/r0.5 | 3.020790 | 2.363560 | 1.223 | 1.172–1.310 | 0.176 | 1.063 |
| simplify/medium/seam-heavy/allocating/a0/r0.25 | 3.601127 | 2.987393 | 1.211 | 1.193–1.255 | 0.088 | 1.057 |
| simplify/medium/seam-heavy/caller-buffer/a0/r0.25 | 4.267290 | 3.428212 | 1.248 | 1.202–1.276 | 0.039 | 1.063 |
| simplify/medium/seam-heavy/allocating/a0/r0.125 | 4.917332 | 4.062289 | 1.212 | 1.147–1.254 | 0.126 | 1.057 |
| simplify/medium/seam-heavy/caller-buffer/a0/r0.125 | 3.347886 | 2.738751 | 1.225 | 1.203–1.232 | 0.063 | 1.063 |
| simplify_with_attributes/medium/seam-heavy/allocating/a1/r0.5 | 2.852509 | 2.401591 | 1.212 | 1.149–1.224 | 0.034 | 1.057 |
| simplify_with_attributes/medium/seam-heavy/caller-buffer/a1/r0.5 | 3.000923 | 2.423112 | 1.244 | 1.216–1.275 | 0.051 | 1.062 |
| simplify_with_attributes/medium/seam-heavy/allocating/a8/r0.25 | 4.436488 | 3.743189 | 1.259 | 1.230–1.291 | 0.079 | 1.037 |
| simplify_with_attributes/medium/seam-heavy/caller-buffer/a8/r0.25 | 4.482034 | 3.530694 | 1.269 | 1.262–1.276 | 0.012 | 1.039 |
| simplify_with_attributes/medium/seam-heavy/allocating/a12/r0.125 | 5.062560 | 4.219670 | 1.199 | 1.190–1.210 | 0.019 | 1.033 |
| simplify_with_attributes/medium/seam-heavy/caller-buffer/a12/r0.125 | 8.887894 | 7.274335 | 1.219 | 1.205–1.239 | 0.074 | 1.035 |
| vertex_cache/medium/disconnected/allocating/a0/rNone | 1.222419 | 1.106812 | 1.096 | 1.031–1.199 | 0.081 | 1.000 |
| vertex_cache/medium/disconnected/caller-buffer/a0/rNone | 1.224298 | 1.066656 | 1.146 | 1.113–1.181 | 0.038 | 1.000 |
| overdraw/medium/disconnected/allocating/a0/rNone | 0.327692 | 0.292931 | 1.118 | 1.085–1.187 | 0.219 | 0.999 |
| overdraw/medium/disconnected/caller-buffer/a0/rNone | 0.190249 | 0.171147 | 1.111 | 1.089–1.131 | 0.027 | 0.997 |
| simplify_scale/medium/disconnected/allocation-free/a0/rNone | 0.019230 | 0.029335 | 0.670 | 0.601–0.704 | 0.153 | 1.000 |
| simplify/medium/disconnected/allocating/a0/r0.5 | 3.645921 | 2.883071 | 1.258 | 1.247–1.282 | 0.019 | 1.056 |
| simplify/medium/disconnected/caller-buffer/a0/r0.5 | 4.623489 | 3.537713 | 1.292 | 1.253–1.330 | 0.649 | 1.062 |
| simplify/medium/disconnected/allocating/a0/r0.25 | 4.033740 | 3.380687 | 1.199 | 1.190–1.213 | 0.070 | 1.056 |
| simplify/medium/disconnected/caller-buffer/a0/r0.25 | 5.304971 | 4.189569 | 1.268 | 1.243–1.294 | 0.040 | 1.062 |
| simplify/medium/disconnected/allocating/a0/r0.125 | 4.578781 | 3.805612 | 1.210 | 1.199–1.215 | 0.080 | 1.056 |
| simplify/medium/disconnected/caller-buffer/a0/r0.125 | 4.775971 | 4.002536 | 1.212 | 1.184–1.218 | 0.051 | 1.062 |
| simplify_with_attributes/medium/disconnected/allocating/a1/r0.5 | 3.447413 | 2.837396 | 1.221 | 1.184–1.249 | 0.029 | 1.057 |
| simplify_with_attributes/medium/disconnected/caller-buffer/a1/r0.5 | 3.457422 | 2.701936 | 1.275 | 1.254–1.331 | 0.128 | 1.062 |
| simplify_with_attributes/medium/disconnected/allocating/a8/r0.25 | 3.601859 | 2.836574 | 1.272 | 1.263–1.281 | 0.047 | 1.037 |
| simplify_with_attributes/medium/disconnected/caller-buffer/a8/r0.25 | 3.797456 | 2.957076 | 1.292 | 1.281–1.314 | 0.062 | 1.039 |
| simplify_with_attributes/medium/disconnected/allocating/a12/r0.125 | 8.339457 | 6.728794 | 1.220 | 1.201–1.260 | 0.114 | 1.033 |
| simplify_with_attributes/medium/disconnected/caller-buffer/a12/r0.125 | 11.437433 | 9.186901 | 1.242 | 1.199–1.269 | 0.257 | 1.035 |
| vertex_cache/medium/sparse/allocating/a0/rNone | 1.190599 | 1.091392 | 1.128 | 1.014–1.182 | 0.105 | 1.000 |
| vertex_cache/medium/sparse/caller-buffer/a0/rNone | 1.175896 | 1.018015 | 1.201 | 1.070–1.224 | 0.292 | 1.000 |
| overdraw/medium/sparse/allocating/a0/rNone | 0.254303 | 0.240387 | 1.047 | 1.020–1.077 | 0.092 | 0.999 |
| overdraw/medium/sparse/caller-buffer/a0/rNone | 0.257093 | 0.240702 | 1.059 | 1.043–1.074 | 0.040 | 0.998 |
| simplify_scale/medium/sparse/allocation-free/a0/rNone | 0.035497 | 0.053110 | 0.668 | 0.649–0.727 | 0.797 | 1.000 |
| simplify/medium/sparse/allocating/a0/r0.5 | 2.307063 | 1.773224 | 1.309 | 1.254–1.382 | 0.138 | 1.083 |
| simplify/medium/sparse/caller-buffer/a0/r0.5 | 3.100085 | 2.245072 | 1.356 | 1.260–1.408 | 0.077 | 1.091 |
| simplify/medium/sparse/allocating/a0/r0.25 | 3.298644 | 2.531782 | 1.324 | 1.288–1.349 | 0.085 | 1.083 |
| simplify/medium/sparse/caller-buffer/a0/r0.25 | 4.415598 | 3.355565 | 1.323 | 1.275–1.362 | 0.037 | 1.091 |
| simplify/medium/sparse/allocating/a0/r0.125 | 4.833499 | 3.606768 | 1.328 | 1.263–1.361 | 0.239 | 1.083 |
| simplify/medium/sparse/caller-buffer/a0/r0.125 | 4.533867 | 3.271397 | 1.371 | 1.310–1.439 | 0.152 | 1.091 |
| simplify_with_attributes/medium/sparse/allocating/a1/r0.5 | 4.741657 | 3.970502 | 1.207 | 1.176–1.272 | 0.511 | 1.077 |
| simplify_with_attributes/medium/sparse/caller-buffer/a1/r0.5 | 5.647464 | 4.603963 | 1.236 | 1.142–1.341 | 0.212 | 1.082 |
| simplify_with_attributes/medium/sparse/allocating/a8/r0.25 | 7.584759 | 6.288025 | 1.223 | 1.170–1.313 | 0.107 | 1.046 |
| simplify_with_attributes/medium/sparse/caller-buffer/a8/r0.25 | 7.747139 | 6.085293 | 1.265 | 1.211–1.341 | 0.079 | 1.047 |
| simplify_with_attributes/medium/sparse/allocating/a12/r0.125 | 6.912875 | 5.653181 | 1.205 | 1.132–1.245 | 0.118 | 1.041 |
| simplify_with_attributes/medium/sparse/caller-buffer/a12/r0.125 | 10.211979 | 8.462785 | 1.211 | 1.094–1.263 | 0.143 | 1.042 |
| vertex_cache/million/smooth/allocating/a0/rNone | 130.487591 | 114.121075 | 1.149 | 1.003–1.343 | 0.210 | 1.000 |
| vertex_cache/million/smooth/caller-buffer/a0/rNone | 344.986724 | 271.930030 | 1.111 | 1.012–1.336 | 0.315 | 1.000 |
| overdraw/million/smooth/allocating/a0/rNone | 92.136315 | 69.770492 | 1.318 | 1.076–1.469 | 0.270 | 0.999 |
| overdraw/million/smooth/caller-buffer/a0/rNone | 126.288517 | 99.885222 | 1.261 | 0.862–1.573 | 0.383 | 0.998 |
| simplify_scale/million/smooth/allocation-free/a0/rNone | 6.817500 | 9.802814 | 0.668 | 0.385–0.742 | 0.330 | 1.000 |
| simplify/million/smooth/allocating/a0/r0.5 | 2074.390694 | 1632.322529 | 1.319 | 1.198–1.449 | 0.120 | 1.061 |
| simplify/million/smooth/caller-buffer/a0/r0.5 | 2020.275269 | 1541.431798 | 1.320 | 1.238–1.367 | 0.074 | 1.069 |
| simplify/million/smooth/allocating/a0/r0.25 | 1941.428392 | 1514.367131 | 1.277 | 1.171–1.363 | 0.091 | 1.061 |
| simplify/million/smooth/caller-buffer/a0/r0.25 | 1616.774492 | 1259.789209 | 1.255 | 1.192–1.319 | 0.084 | 1.069 |
| simplify/million/smooth/allocating/a0/r0.125 | 1804.581663 | 1435.544879 | 1.207 | 1.147–1.344 | 0.143 | 1.061 |
| simplify/million/smooth/caller-buffer/a0/r0.125 | 1922.219126 | 1488.799158 | 1.277 | 1.191–1.418 | 0.107 | 1.069 |
| simplify_with_attributes/million/smooth/allocating/a1/r0.5 | 1446.769496 | 1202.136697 | 1.243 | 1.125–1.334 | 0.266 | 1.062 |
| simplify_with_attributes/million/smooth/caller-buffer/a1/r0.5 | 862.010718 | 781.985413 | 1.192 | 1.081–1.268 | 0.132 | 1.068 |
| simplify_with_attributes/million/smooth/allocating/a8/r0.25 | 756.545135 | 615.454175 | 1.234 | 1.192–1.280 | 0.108 | 1.040 |
| simplify_with_attributes/million/smooth/caller-buffer/a8/r0.25 | 897.272476 | 712.555626 | 1.223 | 1.150–1.320 | 0.115 | 1.043 |
| simplify_with_attributes/million/smooth/allocating/a12/r0.125 | 1226.910757 | 993.505652 | 1.170 | 1.119–1.310 | 0.132 | 1.037 |
| simplify_with_attributes/million/smooth/caller-buffer/a12/r0.125 | 1529.952284 | 1353.144353 | 1.160 | 1.070–1.273 | 0.185 | 1.039 |
| vertex_cache/million/seam-heavy/allocating/a0/rNone | 68.905090 | 58.919450 | 1.154 | 1.109–1.200 | 0.077 | 1.000 |
| vertex_cache/million/seam-heavy/caller-buffer/a0/rNone | 67.494178 | 57.711699 | 1.161 | 1.106–1.195 | 0.248 | 1.000 |
| overdraw/million/seam-heavy/allocating/a0/rNone | 19.983878 | 15.570180 | 1.271 | 1.252–1.316 | 0.106 | 0.999 |
| overdraw/million/seam-heavy/caller-buffer/a0/rNone | 21.215324 | 16.600762 | 1.277 | 1.242–1.306 | 0.066 | 0.997 |
| simplify_scale/million/seam-heavy/allocation-free/a0/rNone | 1.362496 | 1.801622 | 0.741 | 0.694–1.185 | 0.268 | 1.000 |
| simplify/million/seam-heavy/allocating/a0/r0.5 | 478.237058 | 377.853618 | 1.270 | 1.229–1.307 | 0.088 | 1.060 |
| simplify/million/seam-heavy/caller-buffer/a0/r0.5 | 584.292462 | 464.687686 | 1.268 | 1.227–1.313 | 0.078 | 1.067 |
| simplify/million/seam-heavy/allocating/a0/r0.25 | 1217.684611 | 937.440670 | 1.295 | 1.223–1.424 | 0.117 | 1.060 |
| simplify/million/seam-heavy/caller-buffer/a0/r0.25 | 1229.310716 | 927.359216 | 1.247 | 1.163–1.370 | 0.128 | 1.067 |
| simplify/million/seam-heavy/allocating/a0/r0.125 | 1125.579735 | 936.271565 | 1.319 | 1.118–1.354 | 0.160 | 1.060 |
| simplify/million/seam-heavy/caller-buffer/a0/r0.125 | 1363.460779 | 1077.968364 | 1.385 | 1.243–1.468 | 0.134 | 1.067 |
| simplify_with_attributes/million/seam-heavy/allocating/a1/r0.5 | 784.773638 | 636.661210 | 1.156 | 1.013–1.503 | 0.248 | 1.059 |
| simplify_with_attributes/million/seam-heavy/caller-buffer/a1/r0.5 | 1125.125144 | 988.172572 | 1.205 | 1.063–1.388 | 0.226 | 1.063 |
| simplify_with_attributes/million/seam-heavy/allocating/a8/r0.25 | 1114.362363 | 1003.809127 | 1.172 | 1.059–1.288 | 0.164 | 1.038 |
| simplify_with_attributes/million/seam-heavy/caller-buffer/a8/r0.25 | 970.062518 | 819.363524 | 1.195 | 1.131–1.226 | 0.064 | 1.039 |
| simplify_with_attributes/million/seam-heavy/allocating/a12/r0.125 | 1045.270891 | 901.570821 | 1.151 | 1.129–1.195 | 0.060 | 1.034 |
| simplify_with_attributes/million/seam-heavy/caller-buffer/a12/r0.125 | 1094.083127 | 913.852968 | 1.165 | 1.125–1.208 | 0.062 | 1.036 |
| vertex_cache/million/disconnected/allocating/a0/rNone | 71.992742 | 63.189524 | 1.145 | 0.991–1.196 | 0.141 | 1.000 |
| vertex_cache/million/disconnected/caller-buffer/a0/rNone | 55.947412 | 49.603941 | 1.132 | 1.100–1.156 | 0.078 | 1.000 |
| overdraw/million/disconnected/allocating/a0/rNone | 29.589512 | 22.113044 | 1.331 | 1.314–1.394 | 0.106 | 0.999 |
| overdraw/million/disconnected/caller-buffer/a0/rNone | 19.645917 | 13.811096 | 1.417 | 1.410–1.428 | 0.013 | 0.997 |
| simplify_scale/million/disconnected/allocation-free/a0/rNone | 1.343648 | 1.801638 | 0.742 | 0.688–1.156 | 0.262 | 1.000 |
| simplify/million/disconnected/allocating/a0/r0.5 | 580.082880 | 512.627740 | 1.206 | 1.107–1.305 | 0.236 | 1.057 |
| simplify/million/disconnected/caller-buffer/a0/r0.5 | 623.935814 | 500.750551 | 1.278 | 1.234–1.323 | 0.101 | 1.064 |
| simplify/million/disconnected/allocating/a0/r0.25 | 754.872061 | 638.528949 | 1.224 | 1.169–1.270 | 0.072 | 1.057 |
| simplify/million/disconnected/caller-buffer/a0/r0.25 | 694.890747 | 555.321770 | 1.228 | 1.150–1.338 | 0.100 | 1.064 |
| simplify/million/disconnected/allocating/a0/r0.125 | 756.270986 | 646.751324 | 1.217 | 1.138–1.302 | 0.089 | 1.057 |
| simplify/million/disconnected/caller-buffer/a0/r0.125 | 751.125651 | 566.101960 | 1.342 | 1.270–1.394 | 0.073 | 1.064 |
| simplify_with_attributes/million/disconnected/allocating/a1/r0.5 | 828.325049 | 664.934537 | 1.226 | 1.130–1.293 | 0.152 | 1.059 |
| simplify_with_attributes/million/disconnected/caller-buffer/a1/r0.5 | 818.926217 | 669.474211 | 1.210 | 1.144–1.246 | 0.108 | 1.063 |
| simplify_with_attributes/million/disconnected/allocating/a8/r0.25 | 821.836015 | 632.148268 | 1.293 | 1.197–1.359 | 0.115 | 1.038 |
| simplify_with_attributes/million/disconnected/caller-buffer/a8/r0.25 | 781.759990 | 707.663157 | 1.192 | 1.146–1.245 | 0.105 | 1.039 |
| simplify_with_attributes/million/disconnected/allocating/a12/r0.125 | 969.452119 | 790.782173 | 1.271 | 1.156–1.369 | 0.202 | 1.034 |
| simplify_with_attributes/million/disconnected/caller-buffer/a12/r0.125 | 976.472020 | 801.699115 | 1.205 | 1.110–1.255 | 0.113 | 1.036 |
| vertex_cache/million/sparse/allocating/a0/rNone | 98.205658 | 101.145362 | 0.998 | 0.976–1.084 | 0.115 | 1.000 |
| vertex_cache/million/sparse/caller-buffer/a0/rNone | 96.631872 | 93.189189 | 1.039 | 0.998–1.109 | 0.098 | 1.000 |
| overdraw/million/sparse/allocating/a0/rNone | 26.393138 | 22.564484 | 1.188 | 1.148–1.241 | 0.080 | 0.999 |
| overdraw/million/sparse/caller-buffer/a0/rNone | 37.543365 | 31.900282 | 1.181 | 1.136–1.202 | 0.041 | 0.998 |
| simplify_scale/million/sparse/allocation-free/a0/rNone | 2.534412 | 3.072198 | 0.693 | 0.678–1.174 | 0.286 | 1.000 |
| simplify/million/sparse/allocating/a0/r0.5 | 501.454996 | 431.636478 | 1.211 | 1.110–1.294 | 0.114 | 1.087 |
| simplify/million/sparse/caller-buffer/a0/r0.5 | 460.504131 | 369.552021 | 1.247 | 1.207–1.281 | 0.069 | 1.095 |
| simplify/million/sparse/allocating/a0/r0.25 | 865.530832 | 701.908642 | 1.182 | 1.016–1.282 | 0.176 | 1.087 |
| simplify/million/sparse/caller-buffer/a0/r0.25 | 717.625828 | 600.641415 | 1.236 | 1.178–1.280 | 0.103 | 1.095 |
| simplify/million/sparse/allocating/a0/r0.125 | 679.966189 | 570.888620 | 1.198 | 1.113–1.311 | 0.212 | 1.087 |
| simplify/million/sparse/caller-buffer/a0/r0.125 | 933.902555 | 711.107851 | 1.292 | 1.141–1.413 | 0.203 | 1.095 |
| simplify_with_attributes/million/sparse/allocating/a1/r0.5 | 928.481993 | 755.015361 | 1.172 | 1.088–1.276 | 0.123 | 1.079 |
| simplify_with_attributes/million/sparse/caller-buffer/a1/r0.5 | 992.993924 | 803.749475 | 1.184 | 1.085–1.265 | 0.194 | 1.084 |
| simplify_with_attributes/million/sparse/allocating/a8/r0.25 | 1331.953894 | 1171.571187 | 1.169 | 1.010–1.294 | 0.215 | 1.047 |
| simplify_with_attributes/million/sparse/caller-buffer/a8/r0.25 | 1131.322175 | 1012.263451 | 1.110 | 0.933–1.195 | 0.134 | 1.049 |
| simplify_with_attributes/million/sparse/allocating/a12/r0.125 | 1991.120916 | 1897.960370 | 1.162 | 0.988–1.364 | 0.357 | 1.042 |
| simplify_with_attributes/million/sparse/caller-buffer/a12/r0.125 | 1700.237294 | 1328.406477 | 1.189 | 1.108–1.367 | 0.199 | 1.044 |

## Lane 3b before and after

Both matrices use at least 20 alternating same-core Rust/C++ pairs per case. The case metric is the median paired ratio; each family mean is the geometric mean of its case medians. The coordinator removed the quiet-machine admission requirement. Waits: zero.

| Family | Before GM (max) | After GM (max) | After median case ratio CV | After median relative IQR width |
|---|---:|---:|---:|---:|
| vertex_cache | 1.328 (1.407) | 1.123 (1.201) | 11.4% | 8.8% |
| overdraw | 1.282 (1.462) | 1.098 (1.417) | 8.6% | 5.3% |
| simplify | 1.315 (1.511) | 1.212 (1.385) | 10.1% | 8.7% |
| simplify_with_attributes | 1.314 (1.495) | 1.208 (1.342) | 10.7% | 7.5% |
| simplify_scale | 0.943 (1.088) | 0.748 (0.884) | 26.5% | 34.4% |

Before CPU 19, physical core 9; after CPU 20, physical core 10. Each was selected from three one-second /proc/stat intervals. Both backends remained pinned to that run's chosen CPU.
Before one-minute load: 2.14–18.83; start/end triples are retained in benchmark.json.
After one-minute load: 9.62–89.98; start/end triples are retained in benchmark.json.

All five timing and memory bars pass. Raw pair ratios, IQR, standard deviation, MAD, CV, per-pair load and core-selection samples remain in the JSON. Shared-machine load is recorded; no quiet qualification is claimed.

Final source: phase 0.1 and default 10,000-case sweep have zero mismatches, including executed WASM identity; both feature-mode tests, no_std WASM build, fmt/clippy and measurement regressions pass. All five final seeded fuzz smokes run for 300 seconds with unchanged source/executable identities.

Artifacts are retained at /mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/lane3b because the requested /mnt/linux-extra/meshopt-artifacts/lane3b is not visible here and mkdir returns Read-only file system. No artifact writes use the reserved capture archive.

Final hardware profiles retain all four counters, exact outputs, annotated samples and full disassembly for the measured maxima of the four previously failing families. Rust/C++ retired-instruction ratios are:

- vertex_cache/medium/sparse/caller-buffer/a0/rNone: 1.490.
- overdraw/million/disconnected/caller-buffer/a0/rNone: 1.306.
- simplify/million/seam-heavy/caller-buffer/a0/r0.125: 1.507.
- simplify_with_attributes/medium/smooth/caller-buffer/a8/r0.25: 1.331.

The short C++ cache capture initially lost 27 samples. Its repeat uses four times the event period, retains exact output and has zero lost samples; the superseded capture is retained. All final captures have zero lost samples. These profiles are diagnostic; timing acceptance uses the paired matrix.

Maximum per-case paired-ratio CVs (all outliers retained): vertex_cache 75.0%, overdraw 166.7%, simplify 113.5%, simplify_with_attributes 76.0%, simplify_scale 79.7%.

Cleanup is complete: the isolated build target is deleted, all 125 Git metadata file hashes are unchanged, and all 612 relocated original file hashes are verified. `results/lane3b-cleanup.json` and `results/lane3b-urgent-relocation.json` retain the cleanup and final-location evidence.
