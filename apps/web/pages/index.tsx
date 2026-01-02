import Link from 'next/link'
import { useEffect, useState } from 'react'

export default function Home() {
  const [status, setStatus] = useState('unknown')
  useEffect(() => {
    fetch('/api/status').then((r) => r.json()).then((j) => setStatus(j.status))
  }, [])

  return (
    <main className="p-8">
      <h1 className="text-2xl font-bold">$LEGAL — Secure Memecoin Launchpad</h1>
      <p className="mt-4">Security-first token launches on Solana.</p>
      <p className="mt-4">API status: {status}</p>
      <div className="mt-6">
        <Link href="/docs">Docs</Link>
      </div>
    </main>
  )
}
