import { useRouter } from 'next/router'
import { useEffect, useState } from 'react'

export default function TokenPage() {
  const { query } = useRouter()
  const [data, setData] = useState<any>(null)

  useEffect(() => {
    if (!query.mint) return
    fetch(`/api/token/${query.mint}`).then((r) => r.json()).then(setData)
  }, [query.mint])

  if (!data) return <div className="p-8">Loading...</div>

  return (
    <div className="p-8">
      <h2 className="text-xl font-bold">Token {data.mint}</h2>
      <ul>
        <li>Supply: {data.supply}</li>
        <li>Authorities: {data.mintAuthority ? data.mintAuthority : 'none'}</li>
        <li>Verified: {data.safe ? 'YES' : 'NO'}</li>
      </ul>
    </div>
  )
}
