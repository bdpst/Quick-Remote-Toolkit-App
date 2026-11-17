"""Inspect the delivered PE/ZIP without loading or executing it (stdlib only)."""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import zipfile


def inspect(executable: Path, archive: Path) -> dict:
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
    assert 3 in resource_types and 14 in resource_types and 24 in resource_types, 'Missing icon/group/manifest'
    assert b'PerMonitorV2' in data and b'asInvoker' in data
    with zipfile.ZipFile(archive) as package:
        assert package.namelist() == ['QuickRemoteToolkitRust.exe', 'RUNTIME.txt']
        assert package.read('QuickRemoteToolkitRust.exe') == data
        assert all(entry.date_time == (1980, 1, 1, 0, 0, 0) for entry in package.infolist())
    return {'machine': 'AMD64', 'subsystem': 'Windows GUI', 'clr_directory': False,
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
    args = parser.parse_args()
    result = inspect(args.executable, args.archive)
    args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(result, ensure_ascii=False, indent=2))
