"""Inspect the delivered PE/ZIP without loading or executing it (stdlib only)."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import struct
import zipfile


def inspect(executable: Path, archive: Path, expected_version: str, expected_commit: str = None) -> dict:
    data = executable.read_bytes()
    u16 = lambda pos: struct.unpack_from('<H', data, pos)[0]
    u32 = lambda pos: struct.unpack_from('<I', data, pos)[0]
    assert data[:2] == b'MZ'
    pe = u32(0x3C)
    assert data[pe:pe + 4] == b'PE\0\0'
    machine, count = u16(pe + 4), u16(pe + 6)
    optional = pe + 24
    assert machine == 0x8664 and u16(optional) == 0x20B, 'Expected Windows x64 PE32+'
    assert u16(optional + 68) == 2, 'Expected GUI subsystem'
    directories = optional + 112
    assert u32(directories + 14 * 8) == u32(directories + 14 * 8 + 4) == 0, 'CLR directory present'
    sections = optional + u16(pe + 20)

    def offset(rva: int) -> int:
        if rva < u32(optional + 60):
            return rva
        for i in range(count):
            section = sections + 40 * i
            virtual_size, address, raw_size, raw = struct.unpack_from('<4I', data, section + 8)
            if address <= rva < address + max(virtual_size, raw_size):
                return raw + rva - address
        raise ValueError(f'Unmapped RVA {rva:x}')

    imports = []
    descriptor = offset(u32(directories + 8))
    while any(data[descriptor:descriptor + 20]):
        name = offset(u32(descriptor + 12))
        imports.append(data[name:data.index(b'\0', name)].decode('ascii'))
        descriptor += 20
    forbidden = [dll for dll in imports if dll.lower().startswith(('vcruntime', 'msvcp', 'api-ms-win-crt'))
                 or dll.lower() in {'ucrtbase.dll', 'mscoree.dll', 'hostfxr.dll', 'coreclr.dll'}]
    assert not forbidden, f'Unexpected runtime imports: {forbidden}'
    resource_rva = u32(directories + 16)
    resource_base = offset(resource_rva)
    entry_count = u16(resource_base + 12) + u16(resource_base + 14)
    resource_types = sorted(u32(resource_base + 16 + i * 8) for i in range(entry_count))
    assert all(t in resource_types for t in [3, 14, 16, 24]), 'Missing icon/group/version/manifest'
    parts = [int(x) for x in expected_version.split('.')] + [0]
    fixed_info = struct.pack('<4I', 0xFEEF04BD, 0x10000, parts[0] << 16 | parts[1], parts[2] << 16 | parts[3])
    assert fixed_info in data, 'File version does not match release'
    assert expected_version.encode('utf-16-le') in data, 'Version string missing'
    assert b'PerMonitorV2' in data and b'asInvoker' in data
    with zipfile.ZipFile(archive) as package:
        assert package.namelist() == ['QuickRemoteToolkitRust.exe', 'RUNTIME.txt', 'BUILD-INFO.json']
        assert package.read('QuickRemoteToolkitRust.exe') == data
        build_info = json.loads(package.read('BUILD-INFO.json'))
        assert build_info['version'] == expected_version
        assert build_info['exe_sha256'] == hashlib.sha256(data).hexdigest()
        assert len(build_info['commit']) == 40
        if expected_commit:
            assert build_info['commit'] == expected_commit, 'Wrong source commit'
            assert build_info['source_dirty'] is False, 'Release source tree is dirty'
        assert expected_version in package.read('RUNTIME.txt').decode('utf-8')
        assert all(entry.date_time == (1980, 1, 1, 0, 0, 0) for entry in package.infolist())
    return {'version': expected_version, 'source_commit': build_info['commit'], 'source_dirty': build_info['source_dirty'], 'machine': 'AMD64', 'subsystem': 'Windows GUI', 'clr_directory': False,
            'external_vc_runtime_imports': False, 'imports': sorted(imports),
            'resource_types': resource_types, 'dpi_manifest': 'PerMonitorV2,PerMonitor',
            'execution_level': 'asInvoker',
            'exe_bytes': len(data), 'exe_sha256': hashlib.sha256(data).hexdigest(),
            'zip_bytes': archive.stat().st_size, 'zip_sha256': hashlib.sha256(archive.read_bytes()).hexdigest(),
            'zip_entries_verified': True}


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('executable', type=Path)
    parser.add_argument('archive', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('--version', help='Expected version (defaults to Cargo.toml)')
    parser.add_argument('--commit')
    args = parser.parse_args()
    version = args.version
    if version is None:
        manifest = Path(__file__).resolve().parents[1] / 'Cargo.toml'
        match = re.search(r'^version\s*=\s*"(\d+\.\d+\.\d+)"', manifest.read_text(encoding='utf-8'), re.M)
        if match is None:
            parser.error('Package version missing from Cargo.toml')
        version = match.group(1)
    result = inspect(args.executable, args.archive, version, args.commit)
    args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(result, ensure_ascii=False, indent=2))
