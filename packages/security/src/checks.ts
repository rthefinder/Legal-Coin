import fs from 'fs';

type Idl = any;

export function checkIdl(path: string) {
  const raw = fs.readFileSync(path, 'utf-8');
  const idl: Idl = JSON.parse(raw);

  const issues: string[] = [];

  if (!idl.program) {
    issues.push('Missing program section in IDL');
  }

  // Simple check: ensure no instruction called `set_admin` or similar
  const badNames = ['set_admin', 'pause', 'upgrade', 'set_authority'];
  for (const ix of idl.instructions || []) {
    for (const bad of badNames) {
      if ((ix.name || '').toLowerCase().includes(bad)) {
        issues.push(`Suspicious instruction name: ${ix.name}`);
      }
    }
  }

  return { idl, issues };
}

if (require.main === module) {
  const p = process.argv[2] || 'target/idl/legal.json';
  const { issues } = checkIdl(p);
  if (issues.length) {
    console.error('Security issues found:');
    issues.forEach((i) => console.error('-', i));
    process.exit(2);
  }
  console.log('No IDL issues detected');
}
